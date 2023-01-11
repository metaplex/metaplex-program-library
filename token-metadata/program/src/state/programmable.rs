use borsh::{BorshDeserialize, BorshSerialize};
use mpl_utils::cmp_pubkeys;
use num_derive::ToPrimitive;
#[cfg(feature = "serde-feature")]
use serde::{Deserialize, Serialize};
use solana_program::program_pack::Pack;
use solana_program::{
    account_info::AccountInfo, instruction::AccountMeta, program_error::ProgramError,
    pubkey::Pubkey,
};
use spl_token::state::Account;

use crate::{error::MetadataError, instruction::DelegateRole, pda::find_delegate_account};

use super::{DelegateRecord, TokenMetadataAccount};

#[repr(C)]
#[cfg_attr(feature = "serde-feature", derive(Serialize, Deserialize))]
#[derive(BorshSerialize, BorshDeserialize, PartialEq, Eq, Debug, Clone)]
pub enum AuthorityType {
    None,
    Metadata,
    Delegate,
    Holder,
}

pub struct AuthorityRequest<'a, 'b> {
    /// Pubkey of the authority.
    pub authority: &'a Pubkey,
    /// Metadata's update authority pubkey of the asset.
    pub update_authority: &'b Pubkey,
    /// Mint address.
    pub mint: &'a Pubkey,
    /// Holder's token account.
    pub token_info: Option<&'a AccountInfo<'a>>,
    /// `DelegateRecord` account of the authority (when the authority is a delegate).
    pub delegate_record_info: Option<&'a AccountInfo<'a>>,
    /// Expected `DelegateRole` for the request.
    pub delegate_role: Option<DelegateRole>,
}

impl AuthorityType {
    /// Determines the `AuthorityType`.
    pub fn get_authority_type(request: AuthorityRequest) -> Result<Self, ProgramError> {
        if let Some(token_info) = request.token_info {
            let token = Account::unpack(&token_info.try_borrow_data()?)?;
            if cmp_pubkeys(&token.owner, request.authority) {
                return Ok(AuthorityType::Holder);
            }
        }

        if let Some(delegate_record_info) = request.delegate_record_info {
            let (pda_key, _) = find_delegate_account(
                request.mint,
                request
                    .delegate_role
                    .ok_or(MetadataError::MissingDelegateRole)?,
                request.update_authority,
                request.authority,
            );

            if cmp_pubkeys(&pda_key, delegate_record_info.key) {
                let delegate_record = DelegateRecord::from_account_info(delegate_record_info)?;

                if Some(delegate_record.role) == request.delegate_role {
                    return Ok(AuthorityType::Delegate);
                }
            }
        }

        if cmp_pubkeys(request.update_authority, request.authority) {
            return Ok(AuthorityType::Metadata);
        }

        Ok(AuthorityType::None)
    }
}

#[derive(Debug, Clone, ToPrimitive)]
pub enum Operation {
    Delegate,
    Transfer,
    Sale,
    MigrateClass,
    Update,
}

impl ToString for Operation {
    fn to_string(&self) -> String {
        match self {
            Operation::Delegate => "Delegate",
            Operation::Transfer => "Transfer",
            Operation::Sale => "Sale",
            Operation::MigrateClass => "MigrateClass",
            Operation::Update => "Update",
        }
        .to_string()
    }
}

#[derive(Debug, Clone, ToPrimitive)]
pub enum PayloadKey {
    Amount,
    Authority,
    Destination,
    Holder,
    Delegate,
    Target,
}

impl ToString for PayloadKey {
    fn to_string(&self) -> String {
        match self {
            PayloadKey::Amount => "Amount",
            PayloadKey::Authority => "Authority",
            PayloadKey::Holder => "Holder",
            PayloadKey::Delegate => "Delegate",
            PayloadKey::Destination => "Destination",
            PayloadKey::Target => "Target",
        }
        .to_string()
    }
}

pub trait ToAccountMeta {
    fn to_account_meta(&self) -> AccountMeta;
}

impl<'info> ToAccountMeta for AccountInfo<'info> {
    fn to_account_meta(&self) -> AccountMeta {
        AccountMeta {
            pubkey: *self.key,
            is_signer: self.is_signer,
            is_writable: self.is_writable,
        }
    }
}
