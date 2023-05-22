use borsh::BorshSerialize;
use mpl_utils::{assert_signer, create_or_allocate_account_raw};
use solana_program::{
    account_info::AccountInfo, entrypoint::ProgramResult, program_error::ProgramError,
    pubkey::Pubkey,
};

use crate::{
    assertions::{assert_derivation, assert_owned_by},
    error::MetadataError,
    state::{
        CollectionAuthorityRecord, Key, Metadata, TokenMetadataAccount, COLLECTION_AUTHORITY,
        COLLECTION_AUTHORITY_RECORD_SIZE, PREFIX,
    },
};

pub fn process_approve_collection_authority(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
) -> ProgramResult {
    let [collection_authority_record, new_collection_authority, update_authority, payer, metadata_info, mint_info, system_account_info] =
        match accounts {
            [collection_authority_record, new_collection_authority, update_authority, payer, metadata_info, mint_info, system_account_info] => {
                [
                    collection_authority_record,
                    new_collection_authority,
                    update_authority,
                    payer,
                    metadata_info,
                    mint_info,
                    system_account_info,
                ]
            }
            _ => return Err(ProgramError::NotEnoughAccountKeys),
        };

    let metadata = Metadata::from_account_info(metadata_info)?;
    assert_owned_by(metadata_info, program_id)?;
    assert_owned_by(mint_info, &spl_token::ID)?;
    assert_signer(update_authority)?;
    assert_signer(payer)?;
    if metadata.update_authority != *update_authority.key {
        return Err(MetadataError::UpdateAuthorityIncorrect.into());
    }
    if metadata.mint != *mint_info.key {
        return Err(MetadataError::MintMismatch.into());
    }
    let collection_authority_info_empty = collection_authority_record.try_data_is_empty()?;
    if !collection_authority_info_empty {
        return Err(MetadataError::CollectionAuthorityRecordAlreadyExists.into());
    }
    let collection_authority_path = Vec::from([
        PREFIX.as_bytes(),
        program_id.as_ref(),
        mint_info.key.as_ref(),
        COLLECTION_AUTHORITY.as_bytes(),
        new_collection_authority.key.as_ref(),
    ]);
    let collection_authority_bump_seed = &[assert_derivation(
        program_id,
        collection_authority_record,
        &collection_authority_path,
    )?];
    let mut collection_authority_seeds = collection_authority_path.clone();
    collection_authority_seeds.push(collection_authority_bump_seed);
    create_or_allocate_account_raw(
        *program_id,
        collection_authority_record,
        system_account_info,
        payer,
        COLLECTION_AUTHORITY_RECORD_SIZE,
        &collection_authority_seeds,
    )?;

    let mut record = CollectionAuthorityRecord::from_account_info(collection_authority_record)?;
    record.key = Key::CollectionAuthorityRecord;
    record.bump = collection_authority_bump_seed[0];
    record.update_authority = Some(*update_authority.key);
    record.serialize(&mut *collection_authority_record.try_borrow_mut_data()?)?;
    Ok(())
}
