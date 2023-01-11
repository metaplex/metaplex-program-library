use std::fmt::Display;

use mpl_utils::{assert_signer, token::TokenTransferParams};
use solana_program::{
    account_info::AccountInfo, entrypoint::ProgramResult, msg, program::invoke,
    program_error::ProgramError, pubkey::Pubkey, system_program, sysvar,
};

use crate::{
    assertions::{assert_delegate, assert_owned_by, metadata::assert_currently_holding},
    error::MetadataError,
    instruction::{Context, DelegateRole, Transfer, TransferArgs},
    state::{DelegateRecord, Metadata, Operation, TokenMetadataAccount, TokenStandard},
    utils::{assert_derivation, auth_rules_validate, frozen_transfer, AuthRulesValidateParams},
};

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum TransferAuthority {
    Owner,
    TransferDelegate,
    SaleDelegate,
    UtilityDelegate,
}

impl Display for TransferAuthority {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Owner => write!(f, "Owner"),
            Self::TransferDelegate => write!(f, "TransferDelegate"),
            Self::SaleDelegate => write!(f, "SaleDelegate"),
            Self::UtilityDelegate => write!(f, "UtilityDelegate"),
        }
    }
}

impl From<TransferAuthority> for DelegateRole {
    fn from(delegate: TransferAuthority) -> Self {
        match delegate {
            TransferAuthority::TransferDelegate => DelegateRole::Transfer,
            TransferAuthority::SaleDelegate => DelegateRole::Sale,
            TransferAuthority::UtilityDelegate => DelegateRole::Utility,
            _ => panic!("Invalid delegate role"),
        }
    }
}

impl From<DelegateRole> for TransferAuthority {
    fn from(delegate: DelegateRole) -> Self {
        match delegate {
            DelegateRole::Transfer => TransferAuthority::TransferDelegate,
            DelegateRole::Sale => TransferAuthority::SaleDelegate,
            DelegateRole::Utility => TransferAuthority::UtilityDelegate,
            _ => panic!("Invalid delegate role"),
        }
    }
}

pub fn transfer<'a>(
    program_id: &Pubkey,
    accounts: &'a [AccountInfo<'a>],
    args: TransferArgs,
) -> ProgramResult {
    let context = Transfer::to_context(accounts)?;

    match args {
        TransferArgs::V1 { .. } => transfer_v1(program_id, context, args),
    }
}

fn transfer_v1(program_id: &Pubkey, ctx: Context<Transfer>, args: TransferArgs) -> ProgramResult {
    let TransferArgs::V1 {
        authorization_data: auth_data,
        amount,
    } = args;

    // Check signers

    // This authority must always be a signer, regardless of if it's the
    // actual token owner, a delegate or some other authority authorized
    // by a rule set.
    assert_signer(ctx.accounts.authority_info)?;

    // Assert program ownership.
    assert_owned_by(ctx.accounts.metadata_info, program_id)?;
    assert_owned_by(ctx.accounts.mint_info, &spl_token::ID)?;
    assert_owned_by(ctx.accounts.token_info, &spl_token::ID)?;
    if let Some(delegate_record_info) = ctx.accounts.delegate_record_info {
        assert_owned_by(delegate_record_info, program_id)?;
    }
    if let Some(master_edition) = ctx.accounts.edition_info {
        assert_owned_by(master_edition, program_id)?;
    }
    if let Some(authorization_rules) = ctx.accounts.authorization_rules_info {
        assert_owned_by(authorization_rules, &mpl_token_auth_rules::ID)?;
    }

    // Check if the destination exists.
    if ctx.accounts.destination_info.data_is_empty() {
        // if the token account is empty, we will initialize a new one but it must
        // be a ATA account
        assert_derivation(
            &spl_associated_token_account::id(),
            ctx.accounts.destination_info,
            &[
                ctx.accounts.destination_owner_info.key.as_ref(),
                spl_token::id().as_ref(),
                ctx.accounts.mint_info.key.as_ref(),
            ],
        )?;

        msg!("Initializing associate token account");

        // creating the associated token account
        invoke(
            &spl_associated_token_account::instruction::create_associated_token_account(
                ctx.accounts.payer_info.key,
                ctx.accounts.destination_owner_info.key,
                ctx.accounts.mint_info.key,
                &spl_token::id(),
            ),
            &[
                ctx.accounts.payer_info.clone(),
                ctx.accounts.destination_owner_info.clone(),
                ctx.accounts.mint_info.clone(),
                ctx.accounts.destination_info.clone(),
            ],
        )?;
    } else {
        assert_owned_by(ctx.accounts.destination_info, &spl_token::id())?;
    }

    // Check program IDs.

    if ctx.accounts.spl_token_program_info.key != &spl_token::ID {
        return Err(ProgramError::IncorrectProgramId);
    }

    if ctx.accounts.spl_ata_program_info.key != &spl_associated_token_account::ID {
        return Err(ProgramError::IncorrectProgramId);
    }

    if ctx.accounts.system_program_info.key != &system_program::ID {
        return Err(ProgramError::IncorrectProgramId);
    }

    if ctx.accounts.sysvar_instructions_info.key != &sysvar::instructions::ID {
        return Err(ProgramError::IncorrectProgramId);
    }

    if let Some(auth_rules_program) = ctx.accounts.authorization_rules_program_info {
        if auth_rules_program.key != &mpl_token_auth_rules::ID {
            return Err(ProgramError::IncorrectProgramId);
        }
    }

    // Deserialize metadata.

    let metadata = Metadata::from_account_info(ctx.accounts.metadata_info)?;

    let token_transfer_params: TokenTransferParams = TokenTransferParams {
        mint: ctx.accounts.mint_info.clone(),
        source: ctx.accounts.token_info.clone(),
        destination: ctx.accounts.destination_info.clone(),
        amount,
        authority: ctx.accounts.authority_info.clone(),
        authority_signer_seeds: None,
        token_program: ctx.accounts.spl_token_program_info.clone(),
    };

    let token_standard = metadata
        .token_standard
        .ok_or(MetadataError::InvalidTokenStandard)?;

    // Wallet-to-wallet transfer are always allowed except if a sale
    // delegate is set.
    let is_wallet_to_wallet = ctx.accounts.token_owner_info.owner == &system_program::ID
        && ctx.accounts.destination_owner_info.owner == &system_program::ID;

    // Sale delegates prevent any other kind of transfer.
    let is_sale_delegate_set = if let Some(delegate_role) = metadata.persistent_delegate {
        delegate_role == DelegateRole::Sale
    } else {
        false
    };

    // Determine transfer type.
    let transfer_authority = if let Some(delegate_role) = metadata.persistent_delegate {
        msg!("Found delegate role");
        // We have a delegate set on the metadata so we convert it
        // into our TransferAuthority enum.
        // This will panic for anything other than Sale, Transfer and Utility delegates
        // as those are the only valid delegate roles that can be set on
        // the metadata.
        delegate_role.into()
    } else if ctx.accounts.token_owner_info.key == ctx.accounts.authority_info.key {
        // Owner and authority are the same so this is a normal owner transfer.
        TransferAuthority::Owner
    } else {
        msg!("Invalid transfer authority");
        // Invalid transfer authority.
        return Err(MetadataError::InvalidTransferAuthority.into());
    };

    // Short circuit here if sale delegate is set and it's not a SaleDelegate
    // trying to transfer.
    if transfer_authority != TransferAuthority::SaleDelegate && is_sale_delegate_set {
        return Err(MetadataError::OnlySaleDelegateCanTransfer.into());
    }

    // Build our auth rules params.
    let auth_rules_validate_params = AuthRulesValidateParams {
        mint_info: ctx.accounts.mint_info,
        owner_info: None,
        authority_info: Some(ctx.accounts.authority_info),
        source_info: Some(ctx.accounts.token_owner_info),
        destination_info: Some(ctx.accounts.destination_owner_info),
        programmable_config: metadata.programmable_config.clone(),
        amount,
        auth_data,
        auth_rules_info: ctx.accounts.authorization_rules_info,
        operation: Operation::Transfer {
            scenario: transfer_authority.clone(),
        },
        is_wallet_to_wallet,
    };
    msg!("Transfer Authority: {transfer_authority:?}");

    // Validates the transfer authority.

    // Owner transfers are just a wrapper around SPL token transfer
    // for non-programmable assets.
    // Programmable assets have to follow auth rules.
    match transfer_authority {
        TransferAuthority::Owner => {
            msg!("Owner transfer");

            // Must be the actual current owner of the token where
            // mint, token, owner and metadata accounts all match up.
            assert_currently_holding(
                &crate::ID,
                ctx.accounts.token_owner_info,
                ctx.accounts.metadata_info,
                &metadata,
                ctx.accounts.mint_info,
                ctx.accounts.token_info,
            )?;

            // If it's a pNFT then we follow authorization rules for
            // the transfer.
            if matches!(token_standard, TokenStandard::ProgrammableNonFungible) {
                msg!("Owner transfer for pNFT");
                auth_rules_validate(auth_rules_validate_params)?;
            }
            // Otherwise, proceed to transfer normally.
        }
        // This delegate is only valid for pNFTs and submits to auth rules.
        TransferAuthority::SaleDelegate => {
            msg!("SaleDelegate transfer for pNFT");
            // Validate the delegate
            let delegate_record = DelegateRecord::from_account_info(
                if let Some(delegate_record) = ctx.accounts.delegate_record_info {
                    delegate_record
                } else {
                    return Err(ProgramError::NotEnoughAccountKeys);
                },
            )?;

            assert_delegate(
                ctx.accounts.authority_info.key,
                transfer_authority.into(),
                &delegate_record,
            )?;

            if matches!(token_standard, TokenStandard::ProgrammableNonFungible) {
                auth_rules_validate(auth_rules_validate_params)?;
            } else {
                panic!("Only programmable NFTs can have a sale delegate");
            }
        }
        // A TransferDelegate means PNFTs are subject auth rules,
        // but non-programmable NFTs are transferred normally by either
        // the owner or the delegate.
        TransferAuthority::TransferDelegate => {
            msg!("TransferDelegate transfer for pNFT");
            // Validate the delegate
            let delegate_record = DelegateRecord::from_account_info(
                if let Some(delegate_record) = ctx.accounts.delegate_record_info {
                    delegate_record
                } else {
                    return Err(ProgramError::NotEnoughAccountKeys);
                },
            )?;

            assert_delegate(
                ctx.accounts.authority_info.key,
                transfer_authority.into(),
                &delegate_record,
            )?;

            // If it's a pNFT then we follow authorization rules for
            // the transfer.
            if matches!(token_standard, TokenStandard::ProgrammableNonFungible) {
                auth_rules_validate(auth_rules_validate_params)?;
            }
            // Otherwise, proceed to transfer normally.
        }
        TransferAuthority::UtilityDelegate => {
            // Need to check lock state
            msg!("UtilityDelegate transfer for pNFT");
            if matches!(token_standard, TokenStandard::ProgrammableNonFungible) {
                auth_rules_validate(auth_rules_validate_params)?;
            } else {
                return Err(MetadataError::InvalidTransferAuthority.into());
            }
        }
    }

    // Performs the transfer.

    match token_standard {
        TokenStandard::ProgrammableNonFungible => {
            msg!("Transferring programmable asset");
            frozen_transfer(token_transfer_params, ctx.accounts.edition_info)?
        }
        _ => {
            msg!("Transferring asset");
            mpl_utils::token::spl_token_transfer(token_transfer_params).unwrap()
        }
    }

    Ok(())
}
