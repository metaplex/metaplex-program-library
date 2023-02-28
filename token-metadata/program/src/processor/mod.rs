mod bubblegum;
mod burn;
mod collection;
mod delegate;
pub(crate) mod deprecated;
mod edition;
pub(crate) mod escrow;
mod freeze;
mod metadata;
mod state;
mod uses;

use borsh::{BorshDeserialize, BorshSerialize};
pub use bubblegum::*;
pub use burn::*;
pub use collection::*;
pub use delegate::*;
use deprecated::process_create_metadata_accounts_v2;
pub use edition::*;
pub use escrow::*;
pub use freeze::*;
pub use metadata::*;
use mpl_token_auth_rules::payload::Payload;
use mpl_utils::cmp_pubkeys;
#[cfg(feature = "serde-feature")]
use serde::{Deserialize, Serialize};
use solana_program::{
    account_info::AccountInfo, entrypoint::ProgramResult, msg, program_error::ProgramError,
    pubkey::Pubkey,
};
pub use state::*;
pub use uses::*;

use crate::{
    deprecated_processor::{
        process_deprecated_create_metadata_accounts, process_deprecated_update_metadata_accounts,
    },
    error::MetadataError,
    instruction::MetadataInstruction,
    processor::{
        edition::{
            process_convert_master_edition_v1_to_v2, process_create_master_edition,
            process_mint_new_edition_from_master_edition_via_token,
        },
        escrow::process_transfer_out_of_escrow,
    },
    state::{
        Key, Metadata, TokenMetadataAccount, TokenStandard, TokenState, DISCRIMINATOR_INDEX,
        TOKEN_STATE_INDEX,
    },
};

#[repr(C)]
#[cfg_attr(feature = "serde-feature", derive(Serialize, Deserialize))]
#[derive(BorshSerialize, BorshDeserialize, PartialEq, Eq, Debug, Clone)]
pub struct AuthorizationData {
    pub payload: Payload,
}

impl AuthorizationData {
    pub fn new(payload: Payload) -> Self {
        Self { payload }
    }
    pub fn new_empty() -> Self {
        Self {
            payload: Payload::new(),
        }
    }
}

/// Process Token Metadata instructions.
///
/// The processor is divided into two parts:
/// * It first tries to match the instruction into the new API;
/// * If it is not one of the new instructions, it checks that any metadata
///   account is not a pNFT before forwarding the transaction processing to
///   the "legacy" processor.
pub fn process_instruction<'a>(
    program_id: &'a Pubkey,
    accounts: &'a [AccountInfo<'a>],
    input: &[u8],
) -> ProgramResult {
    let instruction = MetadataInstruction::try_from_slice(input)?;

    // checks if there is a locked token; this will block any instruction that
    // requires the token record account when the token is locked – 'Update' is
    // an example of an instruction that does not require the token record, so
    // it can be executed even when a token is locked
    if is_locked(program_id, accounts) && !matches!(instruction, MetadataInstruction::Unlock(_)) {
        return Err(MetadataError::LockedToken.into());
    }

    // match on the new instruction set
    match instruction {
        MetadataInstruction::Burn(args) => {
            msg!("IX: Burn");
            burn::burn(program_id, accounts, args)
        }
        MetadataInstruction::Create(args) => {
            msg!("IX: Create");
            metadata::create(program_id, accounts, args)
        }
        MetadataInstruction::Mint(args) => {
            msg!("IX: Mint");
            metadata::mint(program_id, accounts, args)
        }
        MetadataInstruction::Delegate(args) => {
            msg!("IX: Delegate");
            delegate::delegate(program_id, accounts, args)
        }
        MetadataInstruction::Revoke(args) => {
            msg!("IX: Revoke");
            delegate::revoke(program_id, accounts, args)
        }
        MetadataInstruction::Lock(args) => {
            msg!("IX: Lock");
            state::lock(program_id, accounts, args)
        }
        MetadataInstruction::Unlock(args) => {
            msg!("IX: Unlock");
            state::unlock(program_id, accounts, args)
        }
        MetadataInstruction::Migrate(args) => {
            msg!("IX: Migrate");
            metadata::migrate(program_id, accounts, args)
        }
        MetadataInstruction::Transfer(args) => {
            msg!("IX: Transfer");
            metadata::transfer(program_id, accounts, args)
        }
        MetadataInstruction::Update(args) => {
            msg!("IX: Update");
            metadata::update(program_id, accounts, args)
        }
        MetadataInstruction::Verify(args) => {
            msg!("IX: Verify");
            collection::verify(program_id, accounts, args)
        }
        _ => {
            // pNFT accounts can only be used by the "new" API; before forwarding
            // the transaction to the "legacy" processor we determine whether we are
            // dealing with a pNFT or not
            if !has_programmable_metadata(program_id, accounts)? {
                process_legacy_instruction(program_id, accounts, instruction)
            } else {
                Err(MetadataError::InstructionNotSupported.into())
            }
        }
    }
}

/// Matches "legacy" (pre-pNFT) instructions.
fn process_legacy_instruction<'a>(
    program_id: &'a Pubkey,
    accounts: &'a [AccountInfo<'a>],
    instruction: MetadataInstruction,
) -> ProgramResult {
    match instruction {
        MetadataInstruction::CreateMetadataAccount(args) => {
            msg!("(Deprecated as of 1.1.0) IX: Create Metadata Accounts");
            process_deprecated_create_metadata_accounts(
                program_id,
                accounts,
                args.data,
                args.is_mutable,
            )
        }
        MetadataInstruction::UpdateMetadataAccount(args) => {
            msg!("(Deprecated as of 1.1.0) IX: Update Metadata Accounts");
            process_deprecated_update_metadata_accounts(
                program_id,
                accounts,
                args.data,
                args.update_authority,
                args.primary_sale_happened,
            )
        }
        MetadataInstruction::CreateMetadataAccountV2(args) => {
            msg!("IX: Create Metadata Accounts v2");
            process_create_metadata_accounts_v2(program_id, accounts, args.data, args.is_mutable)
        }
        MetadataInstruction::CreateMetadataAccountV3(args) => {
            msg!("IX: Create Metadata Accounts v3");
            process_create_metadata_accounts_v3(
                program_id,
                accounts,
                args.data,
                args.is_mutable,
                args.collection_details,
            )
        }
        MetadataInstruction::UpdateMetadataAccountV2(args) => {
            msg!("IX: Update Metadata Accounts v2");
            process_update_metadata_accounts_v2(
                program_id,
                accounts,
                args.data,
                args.update_authority,
                args.primary_sale_happened,
                args.is_mutable,
            )
        }
        MetadataInstruction::DeprecatedCreateMasterEdition(_args) => {
            Err(MetadataError::Removed.into())
        }
        MetadataInstruction::DeprecatedMintNewEditionFromMasterEditionViaPrintingToken => {
            Err(MetadataError::Removed.into())
        }
        MetadataInstruction::UpdatePrimarySaleHappenedViaToken => {
            msg!("IX: Update primary sale via token");
            process_update_primary_sale_happened_via_token(program_id, accounts)
        }
        MetadataInstruction::DeprecatedSetReservationList(_args) => {
            Err(MetadataError::Removed.into())
        }
        MetadataInstruction::DeprecatedCreateReservationList => Err(MetadataError::Removed.into()),
        MetadataInstruction::SignMetadata => {
            msg!("IX: Sign Metadata");
            process_sign_metadata(program_id, accounts)
        }
        MetadataInstruction::RemoveCreatorVerification => {
            msg!("IX: Remove Creator Verification");
            process_remove_creator_verification(program_id, accounts)
        }
        MetadataInstruction::DeprecatedMintPrintingTokensViaToken(_args) => {
            Err(MetadataError::Removed.into())
        }
        MetadataInstruction::DeprecatedMintPrintingTokens(_args) => {
            Err(MetadataError::Removed.into())
        }
        MetadataInstruction::CreateMasterEdition(args) => {
            msg!("(Deprecated as of 1.1.0, please use V3 Create Master Edition)\n V2 Create Master Edition");
            process_create_master_edition(program_id, accounts, args.max_supply)
        }
        MetadataInstruction::CreateMasterEditionV3(args) => {
            msg!("V3 Create Master Edition");
            process_create_master_edition(program_id, accounts, args.max_supply)
        }
        MetadataInstruction::MintNewEditionFromMasterEditionViaToken(args) => {
            msg!("IX: Mint New Edition from Master Edition Via Token");
            process_mint_new_edition_from_master_edition_via_token(
                program_id,
                accounts,
                args.edition,
                false,
            )
        }
        MetadataInstruction::ConvertMasterEditionV1ToV2 => {
            msg!("IX: Convert Master Edition V1 to V2");
            process_convert_master_edition_v1_to_v2(program_id, accounts)
        }
        MetadataInstruction::MintNewEditionFromMasterEditionViaVaultProxy(_args) => {
            Err(MetadataError::Removed.into())
        }
        MetadataInstruction::PuffMetadata => {
            msg!("IX: Puff Metadata");
            process_puff_metadata_account(program_id, accounts)
        }
        MetadataInstruction::VerifyCollection => {
            msg!("IX: Verify Collection");
            verify_collection(program_id, accounts)
        }
        MetadataInstruction::SetAndVerifyCollection => {
            msg!("IX: Set and Verify Collection");
            set_and_verify_collection(program_id, accounts)
        }
        MetadataInstruction::UnverifyCollection => {
            msg!("IX: Unverify Collection");
            unverify_collection(program_id, accounts)
        }
        MetadataInstruction::Utilize(args) => {
            msg!("IX: Use/Utilize Token");
            process_utilize(program_id, accounts, args.number_of_uses)
        }
        MetadataInstruction::ApproveUseAuthority(args) => {
            msg!("IX: Approve Use Authority");
            process_approve_use_authority(program_id, accounts, args.number_of_uses)
        }
        MetadataInstruction::RevokeUseAuthority => {
            msg!("IX: Revoke Use Authority");
            process_revoke_use_authority(program_id, accounts)
        }
        MetadataInstruction::ApproveCollectionAuthority => {
            msg!("IX: Approve Collection Authority");
            process_approve_collection_authority(program_id, accounts)
        }
        MetadataInstruction::RevokeCollectionAuthority => {
            msg!("IX: Revoke Collection Authority");
            process_revoke_collection_authority(program_id, accounts)
        }
        MetadataInstruction::FreezeDelegatedAccount => {
            msg!("IX: Freeze Delegated Account");
            process_freeze_delegated_account(program_id, accounts)
        }
        MetadataInstruction::ThawDelegatedAccount => {
            msg!("IX: Thaw Delegated Account");
            process_thaw_delegated_account(program_id, accounts)
        }
        MetadataInstruction::BurnNft => {
            msg!("IX: Burn NFT");
            process_burn_nft(program_id, accounts)
        }
        MetadataInstruction::BurnEditionNft => {
            msg!("IX: Burn Edition NFT");
            process_burn_edition_nft(program_id, accounts)
        }
        MetadataInstruction::VerifySizedCollectionItem => {
            msg!("IX: Verify Collection V2");
            verify_sized_collection_item(program_id, accounts)
        }
        MetadataInstruction::SetAndVerifySizedCollectionItem => {
            msg!("IX: Set and Verify Collection");
            set_and_verify_sized_collection_item(program_id, accounts)
        }
        MetadataInstruction::UnverifySizedCollectionItem => {
            msg!("IX: Unverify Collection");
            unverify_sized_collection_item(program_id, accounts)
        }
        MetadataInstruction::SetCollectionSize(args) => {
            msg!("IX: Set Collection Size");
            set_collection_size(program_id, accounts, args)
        }
        MetadataInstruction::SetTokenStandard => {
            msg!("IX: Set Token Standard");
            process_set_token_standard(program_id, accounts)
        }
        MetadataInstruction::BubblegumSetCollectionSize(args) => {
            msg!("IX: Bubblegum Program Set Collection Size");
            bubblegum_set_collection_size(program_id, accounts, args)
        }
        MetadataInstruction::CreateEscrowAccount => {
            msg!("IX: Create Escrow Account");
            process_create_escrow_account(program_id, accounts)
        }
        MetadataInstruction::CloseEscrowAccount => {
            msg!("IX: Close Escrow Account");
            process_close_escrow_account(program_id, accounts)
        }
        MetadataInstruction::TransferOutOfEscrow(args) => {
            msg!("IX: Transfer Out Of Escrow");
            process_transfer_out_of_escrow(program_id, accounts, args)
        }
        _ => Err(ProgramError::InvalidInstructionData),
    }
}

/// Convenience function for accessing the next item in an [`AccountInfo`]
/// iterator and validating whether the account is present or not.
///
/// This relies on the client setting the `crate::id()` as the pubkey for
/// accounts that are not set, which effectively allows us to use positional
/// optional accounts.
pub fn next_optional_account_info<'a, 'b, I: Iterator<Item = &'a AccountInfo<'b>>>(
    iter: &mut I,
) -> Result<Option<I::Item>, ProgramError> {
    let account_info = iter.next().ok_or(ProgramError::NotEnoughAccountKeys)?;

    Ok(if cmp_pubkeys(account_info.key, &crate::id()) {
        None
    } else {
        Some(account_info)
    })
}

/// Convenience function for accessing an [`AccountInfo`] by index
/// and validating whether the account is present or not.
///
/// This relies on the client setting the `crate::id()` as the pubkey for
/// accounts that are not set, which effectively allows us to use positional
/// optional accounts.
pub fn try_get_account_info<'a>(
    accounts: &'a [AccountInfo<'a>],
    index: usize,
) -> Result<&'a AccountInfo<'a>, ProgramError> {
    let account_info = try_get_optional_account_info(accounts, index)?;
    // validates that we got an account info
    if let Some(account_info) = account_info {
        Ok(account_info)
    } else {
        Err(ProgramError::NotEnoughAccountKeys)
    }
}

/// Convenience function for accessing an [`AccountInfo`] by index
/// and validating whether the account is present or not.
///
/// This relies on the client setting the `crate::id()` as the pubkey for
/// accounts that are not set, which effectively allows us to use positional
/// optional accounts.
pub fn try_get_optional_account_info<'a>(
    accounts: &'a [AccountInfo<'a>],
    index: usize,
) -> Result<Option<&'a AccountInfo<'a>>, ProgramError> {
    if index < accounts.len() {
        Ok(if cmp_pubkeys(accounts[index].key, &crate::id()) {
            None
        } else {
            Some(&accounts[index])
        })
    } else {
        Err(ProgramError::NotEnoughAccountKeys)
    }
}

/// Checks if the instruction's accounts contain a pNFT metadata.
///
/// We need to determine if we are dealing with a pNFT metadata or not
/// so we can restrict the available instructions.
fn has_programmable_metadata<'a>(
    program_id: &Pubkey,
    accounts: &'a [AccountInfo],
) -> Result<bool, ProgramError> {
    for account_info in accounts {
        // checks the account is owned by Token Metadata and it has data
        if account_info.owner == program_id && !account_info.data_is_empty() {
            let discriminator = account_info.data.borrow()[DISCRIMINATOR_INDEX];
            // checks if the account is a Metadata account
            if discriminator == Key::MetadataV1 as u8 {
                let metadata = Metadata::from_account_info(account_info)?;

                if matches!(
                    metadata.token_standard,
                    Some(TokenStandard::ProgrammableNonFungible)
                ) {
                    return Ok(true);
                }
            }
        }
    }

    Ok(false)
}

/// Checks if the instruction's accounts contain a locked pNFT.
fn is_locked<'a>(program_id: &Pubkey, accounts: &'a [AccountInfo]) -> bool {
    for account_info in accounts {
        // checks the account is owned by Token Metadata and it has data
        if account_info.owner == program_id && !account_info.data_is_empty() {
            let data = account_info.data.borrow();
            // checks if the account is a Metadata account
            if (data[DISCRIMINATOR_INDEX] == Key::TokenRecord as u8)
                && (data[TOKEN_STATE_INDEX] == TokenState::Locked as u8)
            {
                return true;
            }
        }
    }

    false
}
