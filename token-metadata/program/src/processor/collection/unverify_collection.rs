use borsh::BorshSerialize;
use mpl_utils::assert_signer;
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    pubkey::Pubkey,
};

use crate::{
    assertions::{
        assert_owned_by, assert_owner_in,
        collection::{assert_collection_verify_is_valid, assert_has_collection_authority},
    },
    error::MetadataError,
    state::{Metadata, TokenMetadataAccount},
};

pub fn unverify_collection(program_id: &Pubkey, accounts: &[AccountInfo]) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();
    let metadata_info = next_account_info(account_info_iter)?;
    let collection_authority_info = next_account_info(account_info_iter)?;
    let collection_mint = next_account_info(account_info_iter)?;
    let collection_info = next_account_info(account_info_iter)?;
    let edition_account_info = next_account_info(account_info_iter)?;
    let using_delegated_collection_authority = accounts.len() == 6;

    assert_signer(collection_authority_info)?;
    assert_owned_by(metadata_info, program_id)?;
    assert_owned_by(collection_info, program_id)?;
    assert_owner_in(collection_mint, &mpl_utils::token::TOKEN_PROGRAM_IDS)?;
    assert_owned_by(edition_account_info, program_id)?;

    let mut metadata = Metadata::from_account_info(metadata_info)?;
    let collection_data = Metadata::from_account_info(collection_info)?;

    assert_collection_verify_is_valid(
        &metadata.collection,
        &collection_data,
        collection_mint,
        edition_account_info,
    )?;
    if using_delegated_collection_authority {
        let collection_authority_record = next_account_info(account_info_iter)?;
        assert_has_collection_authority(
            collection_authority_info,
            &collection_data,
            collection_mint.key,
            Some(collection_authority_record),
        )?;
    } else {
        assert_has_collection_authority(
            collection_authority_info,
            &collection_data,
            collection_mint.key,
            None,
        )?;
    }

    // This handler can only unverify non-sized NFTs
    if collection_data.collection_details.is_some() {
        return Err(MetadataError::SizedCollection.into());
    }

    // If the NFT has collection data, we set it to be unverified and then update the collection
    // size on the Collection Parent.
    if let Some(collection) = &mut metadata.collection {
        collection.verified = false;
    }
    metadata.serialize(&mut *metadata_info.try_borrow_mut_data()?)?;
    Ok(())
}
