use anchor_lang::prelude::*;

use crate::{
    approve_collection_authority_helper, cmp_pubkeys, constants::AUTHORITY_SEED,
    revoke_collection_authority_helper, ApproveCollectionAuthorityHelperAccounts, CandyError,
    CandyMachine, RevokeCollectionAuthorityHelperAccounts,
};

pub fn set_collection(ctx: Context<SetCollection>) -> Result<()> {
    let accounts = ctx.accounts;
    let candy_machine = &mut accounts.candy_machine;

    if candy_machine.items_redeemed > 0 {
        return err!(CandyError::NoChangingCollectionDuringMint);
    } else if !cmp_pubkeys(accounts.collection_mint.key, &candy_machine.collection_mint) {
        return err!(CandyError::MintMismatch);
    }

    // revoking the existing collection authority

    let revoke_accounts = RevokeCollectionAuthorityHelperAccounts {
        authority_pda: accounts.authority_pda.to_account_info(),
        collection_authority_record: accounts.collection_authority_record.to_account_info(),
        collection_metadata: accounts.collection_metadata.to_account_info(),
        collection_mint: accounts.collection_mint.to_account_info(),
        token_metadata_program: accounts.token_metadata_program.to_account_info(),
    };

    revoke_collection_authority_helper(
        revoke_accounts,
        candy_machine.key(),
        *ctx.bumps.get("authority_pda").unwrap(),
    )?;

    // approving the new collection authority

    candy_machine.collection_mint = accounts.new_collection_mint.key();

    let approve_collection_authority_helper_accounts = ApproveCollectionAuthorityHelperAccounts {
        payer: accounts.payer.to_account_info(),
        authority_pda: accounts.authority_pda.to_account_info(),
        collection_update_authority: accounts.new_collection_update_authority.to_account_info(),
        collection_mint: accounts.new_collection_mint.to_account_info(),
        collection_metadata: accounts.new_collection_metadata.to_account_info(),
        collection_authority_record: accounts.new_collection_authority_record.to_account_info(),
        token_metadata_program: accounts.token_metadata_program.to_account_info(),
        system_program: accounts.system_program.to_account_info(),
    };

    approve_collection_authority_helper(approve_collection_authority_helper_accounts)
}

/// Set the collection PDA for the candy machine
#[derive(Accounts)]
pub struct SetCollection<'info> {
    #[account(mut, has_one = authority)]
    candy_machine: Account<'info, CandyMachine>,

    // candy machine authority
    authority: Signer<'info>,

    /// CHECK: account checked in seeds constraint
    #[account(
        mut, seeds = [AUTHORITY_SEED.as_bytes(), candy_machine.to_account_info().key.as_ref()],
        bump
    )]
    authority_pda: UncheckedAccount<'info>,

    // payer of the transaction
    payer: Signer<'info>,

    /// CHECK: account checked in CPI
    collection_mint: UncheckedAccount<'info>,

    /// CHECK: account checked in CPI
    collection_metadata: UncheckedAccount<'info>,

    /// CHECK: account checked in CPI
    #[account(mut)]
    collection_authority_record: UncheckedAccount<'info>,

    // update authority of the new collection NFT
    #[account(mut)]
    new_collection_update_authority: Signer<'info>,

    /// CHECK: account checked in CPI
    new_collection_metadata: UncheckedAccount<'info>,

    /// CHECK: account checked in CPI
    new_collection_mint: UncheckedAccount<'info>,

    /// CHECK: account checked in CPI
    new_collection_master_edition: UncheckedAccount<'info>,

    /// CHECK: account checked in CPI
    #[account(mut)]
    new_collection_authority_record: UncheckedAccount<'info>,

    /// CHECK: account checked in CPI
    #[account(address = mpl_token_metadata::id())]
    token_metadata_program: UncheckedAccount<'info>,

    /// System program account.
    system_program: Program<'info, System>,
}
