use solana_program::{
    account_info::AccountInfo,
    entrypoint::ProgramResult,
    msg,
    program_error::ProgramError,
    program_pack::{IsInitialized, Pack},
    pubkey::Pubkey,
    rent::Rent,
};

use crate::cmp_pubkeys;

pub fn assert_signer(account_info: &AccountInfo) -> ProgramResult {
    if !account_info.is_signer {
        msg!("{key} is not a signer", key = account_info.key,);
        Err(ProgramError::MissingRequiredSignature)
    } else {
        Ok(())
    }
}

pub fn assert_initialized<T: Pack + IsInitialized>(
    account_info: &AccountInfo,
    error: impl Into<ProgramError>,
) -> Result<T, ProgramError> {
    let account: T = T::unpack_unchecked(&account_info.data.borrow())?;
    if !account.is_initialized() {
        msg!("{key} is not initialized", key = account_info.key,);
        Err(error.into())
    } else {
        Ok(account)
    }
}

pub fn assert_owned_by(
    account: &AccountInfo,
    owner: &Pubkey,
    error: impl Into<ProgramError>,
) -> ProgramResult {
    if account.owner != owner {
        msg!(
            "invalid owner for {key}: expected {expected}, got {actual}",
            key = account.key,
            expected = owner,
            actual = account.owner
        );
        Err(error.into())
    } else {
        Ok(())
    }
}

pub fn assert_owner_in(
    account: &AccountInfo,
    owners: &[Pubkey],
    error: impl Into<ProgramError>,
) -> ProgramResult {
    if owners.iter().any(|owner| cmp_pubkeys(owner, account.owner)) {
        Ok(())
    } else {
        Err(error.into())
    }
}

pub fn assert_derivation(
    program_id: &Pubkey,
    account: &AccountInfo,
    path: &[&[u8]],
    error: impl Into<ProgramError>,
) -> Result<u8, ProgramError> {
    let (key, bump) = Pubkey::find_program_address(path, program_id);
    if key != *account.key {
        msg!(
            "derivation assertion failed for {actual_key}:
                \x20   expected {key} with program {program_id}, path {path:?}",
            actual_key = account.key,
            key = key,
            program_id = program_id,
            path = path,
        );
        return Err(error.into());
    }
    Ok(bump)
}

pub fn assert_derivation_with_bump(
    program_id: &Pubkey,
    account: &AccountInfo,
    path: &[&[u8]],
    error: impl Into<ProgramError>,
) -> Result<(), ProgramError> {
    let key = Pubkey::create_program_address(path, program_id)?;
    if key != *account.key {
        return Err(error.into());
    }
    Ok(())
}

pub fn assert_rent_exempt(
    rent: &Rent,
    account_info: &AccountInfo,
    error: impl Into<ProgramError>,
) -> ProgramResult {
    if !rent.is_exempt(account_info.lamports(), account_info.data_len()) {
        msg!(
            "{key} is not rent-exempt: has {balance} lamports, requires {min} lamports",
            key = account_info.key,
            balance = account_info.lamports(),
            min = rent.minimum_balance(account_info.data_len()),
        );
        Err(error.into())
    } else {
        Ok(())
    }
}
