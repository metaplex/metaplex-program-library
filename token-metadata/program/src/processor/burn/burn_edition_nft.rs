use mpl_utils::assert_signer;
use solana_program::{
    account_info::AccountInfo, entrypoint::ProgramResult, program_error::ProgramError,
    pubkey::Pubkey,
};
use spl_token::state::Account as TokenAccount;

use crate::{
    assertions::assert_owned_by,
    error::MetadataError,
    instruction::{Burn, Context},
    state::{Metadata, TokenMetadataAccount},
    utils::assert_initialized,
};

use super::nonfungible_edition::burn_nonfungible_edition;

pub fn process_burn_edition_nft<'a>(
    program_id: &Pubkey,
    accounts: &'a [AccountInfo<'a>],
) -> ProgramResult {
    let [metadata_info, owner_info, print_edition_mint_info, master_edition_mint_info, print_edition_token_info, master_edition_token_info, master_edition_info, print_edition_info, edition_marker_info, spl_token_program_info] =
        match accounts {
            [metadata_info, owner_info, print_edition_mint_info, master_edition_mint_info, print_edition_token_info, master_edition_token_info, master_edition_info, print_edition_info, edition_marker_info, spl_token_program_info] => {
                [
                    metadata_info,
                    owner_info,
                    print_edition_mint_info,
                    master_edition_mint_info,
                    print_edition_token_info,
                    master_edition_token_info,
                    master_edition_info,
                    print_edition_info,
                    edition_marker_info,
                    spl_token_program_info,
                ]
            }
            _ => return Err(ProgramError::NotEnoughAccountKeys),
        };

    // Validate accounts
    // Owner is a signer.
    assert_signer(owner_info)?;

    // Owned by token-metadata program.
    assert_owned_by(metadata_info, program_id)?;
    assert_owned_by(master_edition_info, program_id)?;
    assert_owned_by(print_edition_info, program_id)?;
    assert_owned_by(edition_marker_info, program_id)?;

    // Owned by spl-token program.
    assert_owned_by(master_edition_mint_info, &spl_token::ID)?;
    assert_owned_by(master_edition_token_info, &spl_token::ID)?;
    assert_owned_by(print_edition_mint_info, &spl_token::ID)?;
    assert_owned_by(print_edition_token_info, &spl_token::ID)?;

    let metadata = Metadata::from_account_info(metadata_info)?;
    let token: TokenAccount = assert_initialized(print_edition_token_info)?;

    // Validate relationships between accounts.

    // Owner passed in matches the owner of the token account.
    if token.owner != *owner_info.key {
        return Err(MetadataError::InvalidOwner.into());
    }

    // Mint account passed in matches the mint of the token account.
    if &token.mint != print_edition_mint_info.key {
        return Err(MetadataError::MintMismatch.into());
    }

    // Token account must have sufficient balance for burn.
    if token.amount != 1 {
        return Err(MetadataError::InsufficientTokenBalance.into());
    }

    // Metadata account must match the mint.
    if token.mint != metadata.mint {
        return Err(MetadataError::MintMismatch.into());
    }
    // Contruct our new Burn handler context so we can re-use the same code for both.
    let accounts = Burn {
        authority_info: owner_info,
        collection_metadata_info: None,
        metadata_info,
        edition_info: Some(print_edition_info),
        mint_info: print_edition_mint_info,
        token_info: print_edition_token_info,
        master_edition_info: Some(master_edition_info),
        master_edition_mint_info: Some(master_edition_mint_info),
        master_edition_token_info: Some(master_edition_token_info),
        edition_marker_info: Some(edition_marker_info),
        token_record_info: None,
        // This handler doesn't get system program and sysvars instructions
        // but we need them to create the Burn struct. They are not used in the burn_nonfungible_edition handler.
        system_program_info: spl_token_program_info,
        sysvar_instructions_info: spl_token_program_info,
        spl_token_program_info,
    };
    let context = Context {
        accounts,
        remaining_accounts: vec![],
    };

    burn_nonfungible_edition(&context)
}
