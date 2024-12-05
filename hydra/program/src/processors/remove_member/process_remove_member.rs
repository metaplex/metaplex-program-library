use crate::{
    state::{Fanout, FanoutMembershipVoucher},
    utils::{logic::calculation::*, validation::assert_owned_by},
    MembershipModel,
};
use anchor_lang::prelude::*;

use crate::error::HydraError;

#[derive(Accounts)]
pub struct RemoveMember<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    /// CHECK: Checked in program
    pub member: UncheckedAccount<'info>,
    #[account(
    mut,
    seeds = [b"fanout-config", fanout.name.as_bytes()],
    has_one = authority,
    bump = fanout.bump_seed,
    )]
    pub fanout: Account<'info, Fanout>,
    #[account(
    mut,
    close = destination,
    seeds = [b"fanout-membership", fanout.key().as_ref(), member.key().as_ref()],
    bump
    )]
    pub membership_account: Account<'info, FanoutMembershipVoucher>,
    #[account(mut)]
    /// CHECK: Checked in Program
    pub destination: UncheckedAccount<'info>,
}

pub fn remove_member(ctx: Context<RemoveMember>) -> Result<()> {
    let member_voucher = &ctx.accounts.membership_account;
    let fanout = &mut ctx.accounts.fanout;
    assert_owned_by(&fanout.to_account_info(), &crate::ID)?;
    assert_owned_by(&member_voucher.to_account_info(), &crate::ID)?;
    update_fanout_for_remove(fanout)?;
    if assert_owned_by(
        &ctx.accounts.destination,
        &anchor_lang::solana_program::system_program::id(),
    )
    .is_err()
    {
        return Err(HydraError::InvalidCloseAccountDestination.into());
    }
    if fanout.membership_model != MembershipModel::NFT
        && fanout.membership_model != MembershipModel::Wallet
    {
        return Err(HydraError::RemoveNotSupported.into());
    }
    if member_voucher.shares != 0 {
        return Err(HydraError::RemoveSharesMustBeZero.into());
    }
    Ok(())
}
