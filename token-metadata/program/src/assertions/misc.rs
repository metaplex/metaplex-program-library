use solana_program::{
    account_info::AccountInfo,
    entrypoint::ProgramResult,
    program_error::ProgramError,
    program_option::COption,
    program_pack::{IsInitialized, Pack},
    pubkey::Pubkey,
    rent::Rent,
};
use spl_token_2022::{
    extension::{BaseState, StateWithExtensions},
    state::Account,
};

use crate::error::MetadataError;

/// assert initialized account
pub fn assert_initialized<T: Pack + IsInitialized>(
    account_info: &AccountInfo,
) -> Result<T, ProgramError> {
    mpl_utils::assert_initialized(account_info, MetadataError::Uninitialized)
}

pub fn token_unpack<S: BaseState>(
    account_data: &[u8],
) -> Result<StateWithExtensions<'_, S>, ProgramError> {
    mpl_utils::token::unpack_with_error(account_data, MetadataError::Uninitialized)
}

pub fn assert_mint_authority_matches_mint(
    mint_authority: &COption<Pubkey>,
    mint_authority_info: &AccountInfo,
) -> ProgramResult {
    match mint_authority {
        COption::None => {
            return Err(MetadataError::InvalidMintAuthority.into());
        }
        COption::Some(key) => {
            if mint_authority_info.key != key {
                return Err(MetadataError::InvalidMintAuthority.into());
            }
        }
    }

    if !mint_authority_info.is_signer {
        return Err(MetadataError::NotMintAuthority.into());
    }

    Ok(())
}

pub fn assert_freeze_authority_matches_mint(
    freeze_authority: &COption<Pubkey>,
    freeze_authority_info: &AccountInfo,
) -> ProgramResult {
    match freeze_authority {
        COption::None => {
            return Err(MetadataError::InvalidFreezeAuthority.into());
        }
        COption::Some(key) => {
            if freeze_authority_info.key != key {
                return Err(MetadataError::InvalidFreezeAuthority.into());
            }
        }
    }
    Ok(())
}

pub fn assert_delegated_tokens(
    delegate: &AccountInfo,
    mint_info: &AccountInfo,
    token_account_info: &AccountInfo,
) -> ProgramResult {
    assert_owner_in(mint_info, &mpl_utils::token::TOKEN_PROGRAM_IDS)?;

    let token_account = token_unpack::<Account>(&token_account_info.try_borrow_data()?)?.base;

    assert_owner_in(token_account_info, &mpl_utils::token::TOKEN_PROGRAM_IDS)?;

    if token_account.mint != *mint_info.key {
        return Err(MetadataError::MintMismatch.into());
    }

    if token_account.amount < 1 {
        return Err(MetadataError::NotEnoughTokens.into());
    }

    if token_account.delegate == COption::None
        || token_account.delegated_amount != token_account.amount
        || token_account.delegate.unwrap() != *delegate.key
    {
        return Err(MetadataError::InvalidDelegate.into());
    }
    Ok(())
}

pub fn assert_derivation(
    program_id: &Pubkey,
    account: &AccountInfo,
    path: &[&[u8]],
) -> Result<u8, ProgramError> {
    mpl_utils::assert_derivation(program_id, account, path, MetadataError::DerivedKeyInvalid)
}

pub fn assert_owned_by(account: &AccountInfo, owner: &Pubkey) -> ProgramResult {
    mpl_utils::assert_owned_by(account, owner, MetadataError::IncorrectOwner)
}

pub fn assert_owner_in(account: &AccountInfo, possible_owners: &[&Pubkey]) -> ProgramResult {
    mpl_utils::assert_owner_in(account, possible_owners, MetadataError::IncorrectOwner)
}

pub fn assert_token_program_matches_package(token_program_info: &AccountInfo) -> ProgramResult {
    mpl_utils::token::assert_token_program_matches_package(
        token_program_info,
        MetadataError::InvalidTokenProgram,
    )
}

pub fn assert_rent_exempt(rent: &Rent, account_info: &AccountInfo) -> ProgramResult {
    mpl_utils::assert_rent_exempt(rent, account_info, MetadataError::NotRentExempt)
}
