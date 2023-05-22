use borsh::BorshSerialize;
use mpl_utils::{assert_signer, create_or_allocate_account_raw};
use solana_program::{
    account_info::AccountInfo, entrypoint::ProgramResult, program::invoke,
    program_error::ProgramError, pubkey::Pubkey,
};
use spl_token::instruction::approve;

use crate::{
    assertions::{
        metadata::assert_currently_holding,
        uses::{assert_burner, assert_use_authority_derivation, process_use_authority_validation},
    },
    error::MetadataError,
    state::{
        Key, Metadata, TokenMetadataAccount, UseAuthorityRecord, UseMethod, PREFIX, USER,
        USE_AUTHORITY_RECORD_SIZE,
    },
};

pub fn process_approve_use_authority(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    number_of_uses: u64,
) -> ProgramResult {
    // let account_info_iter = &mut accounts.iter();

    let [use_authority_record_info, owner_info, payer, user_info, token_account_info, metadata_info, mint_info, program_as_burner, token_program_account_info, system_account_info] =
        match accounts {
            [use_authority_record_info, owner_info, payer, user_info, token_account_info, metadata_info, mint_info, program_as_burner, token_program_account_info, system_account_info] => {
                [
                    use_authority_record_info,
                    owner_info,
                    payer,
                    user_info,
                    token_account_info,
                    metadata_info,
                    mint_info,
                    program_as_burner,
                    token_program_account_info,
                    system_account_info,
                ]
            }
            _ => return Err(ProgramError::NotEnoughAccountKeys),
        };

    let metadata: Metadata = Metadata::from_account_info(metadata_info)?;

    if metadata.uses.is_none() {
        return Err(MetadataError::Unusable.into());
    }
    if *token_program_account_info.key != spl_token::ID {
        return Err(MetadataError::InvalidTokenProgram.into());
    }
    assert_signer(owner_info)?;
    assert_signer(payer)?;
    assert_currently_holding(
        program_id,
        owner_info,
        metadata_info,
        &metadata,
        mint_info,
        token_account_info,
    )?;
    let metadata_uses = metadata.uses.unwrap();
    let bump_seed = assert_use_authority_derivation(
        program_id,
        use_authority_record_info,
        user_info,
        mint_info,
    )?;
    let use_authority_seeds = &[
        PREFIX.as_bytes(),
        program_id.as_ref(),
        mint_info.key.as_ref(),
        USER.as_bytes(),
        user_info.key.as_ref(),
        &[bump_seed],
    ];
    process_use_authority_validation(use_authority_record_info.data_len(), true)?;
    create_or_allocate_account_raw(
        *program_id,
        use_authority_record_info,
        system_account_info,
        payer,
        USE_AUTHORITY_RECORD_SIZE,
        use_authority_seeds,
    )?;
    if number_of_uses > metadata_uses.remaining {
        return Err(MetadataError::NotEnoughUses.into());
    }
    if metadata_uses.use_method == UseMethod::Burn {
        assert_burner(program_as_burner.key)?;
        invoke(
            &approve(
                token_program_account_info.key,
                token_account_info.key,
                program_as_burner.key,
                owner_info.key,
                &[],
                1,
            )
            .unwrap(),
            &[
                token_program_account_info.clone(),
                token_account_info.clone(),
                program_as_burner.clone(),
                owner_info.clone(),
            ],
        )?;
    }
    let mutable_data = &mut (*use_authority_record_info.try_borrow_mut_data()?);
    let mut record = UseAuthorityRecord::from_bytes(mutable_data)?;

    record.key = Key::UseAuthorityRecord;
    record.allowed_uses = number_of_uses;
    record.bump = bump_seed;
    record.serialize(mutable_data)?;
    Ok(())
}
