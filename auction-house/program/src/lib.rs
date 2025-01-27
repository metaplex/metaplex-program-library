//! # Metaplex Program Library: Auction House
//! AuctionHouse is a protocol for marketplaces to implement a decentralized sales contract. It is simple, fast and very cheap. AuctionHouse is a Solana program available on Mainnet Beta and Devnet. Anyone can create an AuctionHouse and accept any SPL token they wish.
//!
//! Full docs can be found [here](https://docs.metaplex.com/auction-house/definition).

#![allow(clippy::result_large_err)]

pub mod auctioneer;
pub mod bid;
pub mod cancel;
pub mod constants;
pub mod deposit;
pub mod errors;
pub mod execute_sale;
pub mod pda;
pub mod receipt;
pub mod sell;
pub mod state;
pub mod utils;
pub mod withdraw;

pub use state::*;

use crate::{
    auctioneer::*, bid::*, cancel::*, constants::*, deposit::*, errors::AuctionHouseError,
    execute_sale::*, receipt::*, sell::*, utils::*, withdraw::*,
};

use anchor_lang::{
    prelude::*,
    solana_program::{program::invoke_signed, system_instruction},
    AnchorDeserialize, AnchorSerialize,
};
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{Mint, Token, TokenAccount},
};
use spl_token::instruction::revoke;

anchor_lang::declare_id!("hausS13jsjafwWwGqZTUQRmWyvyxn9EQpqMwV1PBBmk");

#[program]
pub mod auction_house {
    use super::*;

    /// Withdraw `amount` from the Auction House Fee Account to a provided destination account.
    pub fn withdraw_from_fee<'info>(
        ctx: Context<'_, '_, '_, 'info, WithdrawFromFee<'info>>,
        amount: u64,
    ) -> Result<()> {
        let auction_house_fee_account = &ctx.accounts.auction_house_fee_account;
        let fee_withdrawal_destination = &ctx.accounts.fee_withdrawal_destination;
        let auction_house = &ctx.accounts.auction_house;
        let system_program = &ctx.accounts.system_program;

        let auction_house_key = auction_house.key();
        let seeds = [
            PREFIX.as_bytes(),
            auction_house_key.as_ref(),
            FEE_PAYER.as_bytes(),
            &[auction_house.fee_payer_bump],
        ];

        invoke_signed(
            &system_instruction::transfer(
                &auction_house_fee_account.key(),
                &fee_withdrawal_destination.key(),
                amount,
            ),
            &[
                auction_house_fee_account.to_account_info(),
                fee_withdrawal_destination.to_account_info(),
                system_program.to_account_info(),
            ],
            &[&seeds],
        )?;

        Ok(())
    }

    /// Withdraw `amount` from the Auction House Treasury Account to a provided destination account.
    pub fn withdraw_from_treasury<'info>(
        ctx: Context<'_, '_, '_, 'info, WithdrawFromTreasury<'info>>,
        amount: u64,
    ) -> Result<()> {
        let treasury_mint = &ctx.accounts.treasury_mint;
        let treasury_withdrawal_destination = &ctx.accounts.treasury_withdrawal_destination;
        let auction_house_treasury = &ctx.accounts.auction_house_treasury;
        let auction_house = &ctx.accounts.auction_house;
        let token_program = &ctx.accounts.token_program;
        let system_program = &ctx.accounts.system_program;

        let is_native = treasury_mint.key() == spl_token::native_mint::id();
        let auction_house_seeds = [
            PREFIX.as_bytes(),
            auction_house.creator.as_ref(),
            auction_house.treasury_mint.as_ref(),
            &[auction_house.bump],
        ];

        let ah_key = auction_house.key();
        let auction_house_treasury_seeds = [
            PREFIX.as_bytes(),
            ah_key.as_ref(),
            TREASURY.as_bytes(),
            &[auction_house.treasury_bump],
        ];
        if !is_native {
            invoke_signed(
                &spl_token::instruction::transfer(
                    token_program.key,
                    &auction_house_treasury.key(),
                    &treasury_withdrawal_destination.key(),
                    &auction_house.key(),
                    &[],
                    amount,
                )?,
                &[
                    auction_house_treasury.to_account_info(),
                    treasury_withdrawal_destination.to_account_info(),
                    token_program.to_account_info(),
                    auction_house.to_account_info(),
                ],
                &[&auction_house_seeds],
            )?;
        } else {
            invoke_signed(
                &system_instruction::transfer(
                    &auction_house_treasury.key(),
                    &treasury_withdrawal_destination.key(),
                    amount,
                ),
                &[
                    auction_house_treasury.to_account_info(),
                    treasury_withdrawal_destination.to_account_info(),
                    system_program.to_account_info(),
                ],
                &[&auction_house_treasury_seeds],
            )?;
        }

        Ok(())
    }

    /// Update Auction House values such as seller fee basis points, update authority, treasury account, etc.
    pub fn update_auction_house<'info>(
        ctx: Context<'_, '_, '_, 'info, UpdateAuctionHouse<'info>>,
        seller_fee_basis_points: Option<u16>,
        requires_sign_off: Option<bool>,
        can_change_sale_price: Option<bool>,
    ) -> Result<()> {
        let treasury_mint = &ctx.accounts.treasury_mint;
        let payer = &ctx.accounts.payer;
        let new_authority = &ctx.accounts.new_authority;
        let auction_house = &mut ctx.accounts.auction_house;
        let fee_withdrawal_destination = &ctx.accounts.fee_withdrawal_destination;
        let treasury_withdrawal_destination_owner =
            &ctx.accounts.treasury_withdrawal_destination_owner;
        let treasury_withdrawal_destination = &ctx.accounts.treasury_withdrawal_destination;
        let token_program = &ctx.accounts.token_program;
        let system_program = &ctx.accounts.system_program;
        let ata_program = &ctx.accounts.ata_program;
        let rent = &ctx.accounts.rent;
        let is_native = treasury_mint.key() == spl_token::native_mint::id();

        if let Some(sfbp) = seller_fee_basis_points {
            if sfbp > 10000 {
                return Err(AuctionHouseError::InvalidBasisPoints.into());
            }

            auction_house.seller_fee_basis_points = sfbp;
        }

        if let Some(rqf) = requires_sign_off {
            auction_house.requires_sign_off = rqf;
        }
        if let Some(chsp) = can_change_sale_price {
            auction_house.can_change_sale_price = chsp;
        }

        auction_house.authority = new_authority.key();
        auction_house.treasury_withdrawal_destination = treasury_withdrawal_destination.key();
        auction_house.fee_withdrawal_destination = fee_withdrawal_destination.key();

        if !is_native {
            if treasury_withdrawal_destination.data_is_empty() {
                make_ata(
                    treasury_withdrawal_destination.to_account_info(),
                    treasury_withdrawal_destination_owner.to_account_info(),
                    treasury_mint.to_account_info(),
                    payer.to_account_info(),
                    ata_program.to_account_info(),
                    token_program.to_account_info(),
                    system_program.to_account_info(),
                    rent.to_account_info(),
                    &[],
                )?;
            }

            assert_is_ata(
                &treasury_withdrawal_destination.to_account_info(),
                &treasury_withdrawal_destination_owner.key(),
                &treasury_mint.key(),
            )?;
        } else {
            assert_keys_equal(
                treasury_withdrawal_destination.key(),
                treasury_withdrawal_destination_owner.key(),
            )?;
        }

        Ok(())
    }

    /// Create a new Auction House instance.
    pub fn create_auction_house<'info>(
        ctx: Context<'_, '_, '_, 'info, CreateAuctionHouse<'info>>,
        _bump: u8,
        fee_payer_bump: u8,
        treasury_bump: u8,
        seller_fee_basis_points: u16,
        requires_sign_off: bool,
        can_change_sale_price: bool,
    ) -> Result<()> {
        let treasury_mint = &ctx.accounts.treasury_mint;
        let payer = &ctx.accounts.payer;
        let authority = &ctx.accounts.authority;
        let auction_house = &mut ctx.accounts.auction_house;
        let auction_house_fee_account = &ctx.accounts.auction_house_fee_account;
        let auction_house_treasury = &ctx.accounts.auction_house_treasury;
        let fee_withdrawal_destination = &ctx.accounts.fee_withdrawal_destination;
        let treasury_withdrawal_destination_owner =
            &ctx.accounts.treasury_withdrawal_destination_owner;
        let treasury_withdrawal_destination = &ctx.accounts.treasury_withdrawal_destination;
        let token_program = &ctx.accounts.token_program;
        let system_program = &ctx.accounts.system_program;
        let ata_program = &ctx.accounts.ata_program;
        let rent = &ctx.accounts.rent;

        auction_house.bump = *ctx
            .bumps
            .get("auction_house")
            .ok_or(AuctionHouseError::BumpSeedNotInHashMap)?;

        // Check that bumps passed in are canonical.
        if fee_payer_bump
            != *ctx
                .bumps
                .get("auction_house_fee_account")
                .ok_or(AuctionHouseError::BumpSeedNotInHashMap)?
        {
            return Err(AuctionHouseError::BumpSeedNotInHashMap.into());
        }
        auction_house.fee_payer_bump = fee_payer_bump;

        if treasury_bump
            != *ctx
                .bumps
                .get("auction_house_treasury")
                .ok_or(AuctionHouseError::BumpSeedNotInHashMap)?
        {
            return Err(AuctionHouseError::BumpSeedNotInHashMap.into());
        }
        auction_house.treasury_bump = treasury_bump;

        if seller_fee_basis_points > 10000 {
            return Err(AuctionHouseError::InvalidBasisPoints.into());
        }
        auction_house.seller_fee_basis_points = seller_fee_basis_points;
        auction_house.requires_sign_off = requires_sign_off;
        auction_house.can_change_sale_price = can_change_sale_price;
        auction_house.creator = authority.key();
        auction_house.authority = authority.key();
        auction_house.treasury_mint = treasury_mint.key();
        auction_house.auction_house_fee_account = auction_house_fee_account.key();
        auction_house.auction_house_treasury = auction_house_treasury.key();
        auction_house.treasury_withdrawal_destination = treasury_withdrawal_destination.key();
        auction_house.fee_withdrawal_destination = fee_withdrawal_destination.key();

        let is_native = treasury_mint.key() == spl_token::native_mint::id();

        let ah_key = auction_house.key();

        let auction_house_treasury_seeds = [
            PREFIX.as_bytes(),
            ah_key.as_ref(),
            TREASURY.as_bytes(),
            &[treasury_bump],
        ];

        create_program_token_account_if_not_present(
            auction_house_treasury,
            system_program,
            payer,
            token_program,
            treasury_mint,
            &auction_house.to_account_info(),
            rent,
            &auction_house_treasury_seeds,
            &[],
            is_native,
        )?;

        if !is_native {
            if treasury_withdrawal_destination.data_is_empty() {
                make_ata(
                    treasury_withdrawal_destination.to_account_info(),
                    treasury_withdrawal_destination_owner.to_account_info(),
                    treasury_mint.to_account_info(),
                    payer.to_account_info(),
                    ata_program.to_account_info(),
                    token_program.to_account_info(),
                    system_program.to_account_info(),
                    rent.to_account_info(),
                    &[],
                )?;
            }

            assert_is_ata(
                &treasury_withdrawal_destination.to_account_info(),
                &treasury_withdrawal_destination_owner.key(),
                &treasury_mint.key(),
            )?;
        } else {
            assert_keys_equal(
                treasury_withdrawal_destination.key(),
                treasury_withdrawal_destination_owner.key(),
            )?;
        }

        Ok(())
    }

    /// Create a private buy bid by creating a `buyer_trade_state` account and an `escrow_payment` account and funding the escrow with the necessary SOL or SPL token amount.
    pub fn buy<'info>(
        ctx: Context<'_, '_, '_, 'info, Buy<'info>>,
        trade_state_bump: u8,
        escrow_payment_bump: u8,
        buyer_price: u64,
        token_size: u64,
    ) -> Result<()> {
        private_bid(
            ctx,
            trade_state_bump,
            escrow_payment_bump,
            buyer_price,
            token_size,
        )
    }

    pub fn auctioneer_buy<'info>(
        ctx: Context<'_, '_, '_, 'info, AuctioneerBuy<'info>>,
        trade_state_bump: u8,
        escrow_payment_bump: u8,
        buyer_price: u64,
        token_size: u64,
    ) -> Result<()> {
        bid::auctioneer_private_bid(
            ctx,
            trade_state_bump,
            escrow_payment_bump,
            buyer_price,
            token_size,
        )
    }

    /// Create a public buy bid by creating a `public_buyer_trade_state` account and an `escrow_payment` account and funding the escrow with the necessary SOL or SPL token amount.
    pub fn public_buy<'info>(
        ctx: Context<'_, '_, '_, 'info, PublicBuy<'info>>,
        trade_state_bump: u8,
        escrow_payment_bump: u8,
        buyer_price: u64,
        token_size: u64,
    ) -> Result<()> {
        public_bid(
            ctx,
            trade_state_bump,
            escrow_payment_bump,
            buyer_price,
            token_size,
        )
    }

    /// Create a public buy bid by creating a `public_buyer_trade_state` account and an `escrow_payment` account and funding the escrow with the necessary SOL or SPL token amount.
    pub fn auctioneer_public_buy<'info>(
        ctx: Context<'_, '_, '_, 'info, AuctioneerPublicBuy<'info>>,
        trade_state_bump: u8,
        escrow_payment_bump: u8,
        buyer_price: u64,
        token_size: u64,
    ) -> Result<()> {
        bid::auctioneer_public_bid(
            ctx,
            trade_state_bump,
            escrow_payment_bump,
            buyer_price,
            token_size,
        )
    }

    /// Cancel a bid or ask by revoking the token delegate, transferring all lamports from the trade state account to the fee payer, and setting the trade state account data to zero so it can be garbage collected.
    pub fn cancel<'info>(
        ctx: Context<'_, '_, '_, 'info, Cancel<'info>>,
        buyer_price: u64,
        token_size: u64,
    ) -> Result<()> {
        cancel::cancel(ctx, buyer_price, token_size)
    }

    /// Cancel, but with an auctioneer
    pub fn auctioneer_cancel<'info>(
        ctx: Context<'_, '_, '_, 'info, AuctioneerCancel<'info>>,
        buyer_price: u64,
        token_size: u64,
    ) -> Result<()> {
        cancel::auctioneer_cancel(ctx, buyer_price, token_size)
    }

    /// Deposit `amount` into the escrow payment account for your specific wallet.
    pub fn deposit<'info>(
        ctx: Context<'_, '_, '_, 'info, Deposit<'info>>,
        escrow_payment_bump: u8,
        amount: u64,
    ) -> Result<()> {
        deposit::deposit(ctx, escrow_payment_bump, amount)
    }

    /// Deposit `amount` into the escrow payment account for your specific wallet.
    pub fn auctioneer_deposit<'info>(
        ctx: Context<'_, '_, '_, 'info, AuctioneerDeposit<'info>>,
        escrow_payment_bump: u8,
        amount: u64,
    ) -> Result<()> {
        deposit::auctioneer_deposit(ctx, escrow_payment_bump, amount)
    }

    pub fn execute_sale<'info>(
        ctx: Context<'_, '_, '_, 'info, ExecuteSale<'info>>,
        escrow_payment_bump: u8,
        _free_trade_state_bump: u8,
        program_as_signer_bump: u8,
        buyer_price: u64,
        token_size: u64,
    ) -> Result<()> {
        execute_sale::execute_sale(
            ctx,
            escrow_payment_bump,
            _free_trade_state_bump,
            program_as_signer_bump,
            buyer_price,
            token_size,
        )
    }

    pub fn execute_partial_sale<'info>(
        ctx: Context<'_, '_, '_, 'info, ExecutePartialSale<'info>>,
        escrow_payment_bump: u8,
        _free_trade_state_bump: u8,
        program_as_signer_bump: u8,
        buyer_price: u64,
        token_size: u64,
        partial_order_size: Option<u64>,
        partial_order_price: Option<u64>,
    ) -> Result<()> {
        execute_sale::execute_partial_sale(
            ctx,
            escrow_payment_bump,
            _free_trade_state_bump,
            program_as_signer_bump,
            buyer_price,
            token_size,
            partial_order_size,
            partial_order_price,
        )
    }

    pub fn auctioneer_execute_sale<'info>(
        ctx: Context<'_, '_, '_, 'info, AuctioneerExecuteSale<'info>>,
        escrow_payment_bump: u8,
        _free_trade_state_bump: u8,
        program_as_signer_bump: u8,
        buyer_price: u64,
        token_size: u64,
    ) -> Result<()> {
        execute_sale::auctioneer_execute_sale(
            ctx,
            escrow_payment_bump,
            _free_trade_state_bump,
            program_as_signer_bump,
            buyer_price,
            token_size,
        )
    }

    pub fn auctioneer_execute_partial_sale<'info>(
        ctx: Context<'_, '_, '_, 'info, AuctioneerExecutePartialSale<'info>>,
        escrow_payment_bump: u8,
        _free_trade_state_bump: u8,
        program_as_signer_bump: u8,
        buyer_price: u64,
        token_size: u64,
        partial_order_size: Option<u64>,
        partial_order_price: Option<u64>,
    ) -> Result<()> {
        execute_sale::auctioneer_execute_partial_sale(
            ctx,
            escrow_payment_bump,
            _free_trade_state_bump,
            program_as_signer_bump,
            buyer_price,
            token_size,
            partial_order_size,
            partial_order_price,
        )
    }

    pub fn sell<'info>(
        ctx: Context<'_, '_, '_, 'info, Sell<'info>>,
        trade_state_bump: u8,
        free_trade_state_bump: u8,
        program_as_signer_bump: u8,
        buyer_price: u64,
        token_size: u64,
    ) -> Result<()> {
        sell::sell(
            ctx,
            trade_state_bump,
            free_trade_state_bump,
            program_as_signer_bump,
            buyer_price,
            token_size,
        )
    }

    pub fn auctioneer_sell<'info>(
        ctx: Context<'_, '_, '_, 'info, AuctioneerSell<'info>>,
        trade_state_bump: u8,
        free_trade_state_bump: u8,
        program_as_signer_bump: u8,
        token_size: u64,
    ) -> Result<()> {
        sell::auctioneer_sell(
            ctx,
            trade_state_bump,
            free_trade_state_bump,
            program_as_signer_bump,
            token_size,
        )
    }

    /// Withdraw `amount` from the escrow payment account for your specific wallet.
    pub fn withdraw<'info>(
        ctx: Context<'_, '_, '_, 'info, Withdraw<'info>>,
        escrow_payment_bump: u8,
        amount: u64,
    ) -> Result<()> {
        withdraw::withdraw(ctx, escrow_payment_bump, amount)
    }

    /// Withdraw `amount` from the escrow payment account for your specific wallet.
    pub fn auctioneer_withdraw<'info>(
        ctx: Context<'_, '_, '_, 'info, AuctioneerWithdraw<'info>>,
        escrow_payment_bump: u8,
        amount: u64,
    ) -> Result<()> {
        withdraw::auctioneer_withdraw(ctx, escrow_payment_bump, amount)
    }

    /// Close the escrow account of the user.
    pub fn close_escrow_account<'info>(
        ctx: Context<'_, '_, '_, 'info, CloseEscrowAccount<'info>>,
        escrow_payment_bump: u8,
    ) -> Result<()> {
        let auction_house_key = ctx.accounts.auction_house.key();
        let wallet_key = ctx.accounts.wallet.key();

        let escrow_signer_seeds = [
            PREFIX.as_bytes(),
            auction_house_key.as_ref(),
            wallet_key.as_ref(),
            &[escrow_payment_bump],
        ];

        invoke_signed(
            &system_instruction::transfer(
                &ctx.accounts.escrow_payment_account.key(),
                &ctx.accounts.wallet.key(),
                ctx.accounts.escrow_payment_account.lamports(),
            ),
            &[
                ctx.accounts.escrow_payment_account.to_account_info(),
                ctx.accounts.wallet.to_account_info(),
                ctx.accounts.system_program.to_account_info(),
            ],
            &[&escrow_signer_seeds],
        )?;
        Ok(())
    }

    pub fn delegate_auctioneer<'info>(
        ctx: Context<'_, '_, '_, 'info, DelegateAuctioneer<'info>>,
        scopes: Vec<AuthorityScope>,
    ) -> Result<()> {
        auctioneer::delegate_auctioneer(ctx, scopes)
    }

    pub fn update_auctioneer<'info>(
        ctx: Context<'_, '_, '_, 'info, UpdateAuctioneer<'info>>,
        scopes: Vec<AuthorityScope>,
    ) -> Result<()> {
        auctioneer::update_auctioneer(ctx, scopes)
    }

    /// Create a listing receipt by creating a `listing_receipt` account.
    pub fn print_listing_receipt<'info>(
        ctx: Context<'_, '_, '_, 'info, PrintListingReceipt<'info>>,
        receipt_bump: u8,
    ) -> Result<()> {
        receipt::print_listing_receipt(ctx, receipt_bump)
    }

    /// Cancel an active listing receipt by setting the `canceled_at` field to the current time.
    pub fn cancel_listing_receipt<'info>(
        ctx: Context<'_, '_, '_, 'info, CancelListingReceipt<'info>>,
    ) -> Result<()> {
        receipt::cancel_listing_receipt(ctx)
    }

    /// Create a bid receipt by creating a `bid_receipt` account.
    pub fn print_bid_receipt<'info>(
        ctx: Context<'_, '_, '_, 'info, PrintBidReceipt<'info>>,
        receipt_bump: u8,
    ) -> Result<()> {
        receipt::print_bid_receipt(ctx, receipt_bump)
    }

    /// Cancel an active bid receipt by setting the `canceled_at` field to the current time.
    pub fn cancel_bid_receipt<'info>(
        ctx: Context<'_, '_, '_, 'info, CancelBidReceipt<'info>>,
    ) -> Result<()> {
        receipt::cancel_bid_receipt(ctx)
    }

    /// Create a purchase receipt by creating a `purchase_receipt` account.
    pub fn print_purchase_receipt<'info>(
        ctx: Context<'_, '_, '_, 'info, PrintPurchaseReceipt<'info>>,
        purchase_receipt_bump: u8,
    ) -> Result<()> {
        receipt::print_purchase_receipt(ctx, purchase_receipt_bump)
    }

    #[doc(hidden)]
    pub fn sell_remaining_accounts<'info>(
        _ctx: Context<'_, '_, '_, 'info, SellRemainingAccounts<'info>>,
    ) -> Result<()> {
        unreachable!();
    }

    #[doc(hidden)]
    pub fn cancel_remaining_accounts<'info>(
        _ctx: Context<'_, '_, '_, 'info, CancelRemainingAccounts<'info>>,
    ) -> Result<()> {
        unreachable!();
    }

    #[doc(hidden)]
    pub fn execute_sale_remaining_accounts<'info>(
        _ctx: Context<'_, '_, '_, 'info, ExecuteSaleRemainingAccounts<'info>>,
    ) -> Result<()> {
        unreachable!();
    }
}

/// Accounts for the [`create_auction_house` handler](auction_house/fn.create_auction_house.html).
#[derive(Accounts)]
#[instruction(bump: u8, fee_payer_bump: u8, treasury_bump: u8)]
pub struct CreateAuctionHouse<'info> {
    /// Treasury mint account, either native SOL mint or a SPL token mint.
    pub treasury_mint: Account<'info, Mint>,

    /// Key paying SOL fees for setting up the Auction House.
    #[account(mut)]
    pub payer: Signer<'info>,

    /// CHECK: User can use whatever they want for initialization.
    // Authority key for the Auction House.
    pub authority: UncheckedAccount<'info>,

    /// CHECK: User can use whatever they want for initialization.
    /// Account that pays for fees if the marketplace executes sales.
    #[account(mut)]
    pub fee_withdrawal_destination: UncheckedAccount<'info>,

    /// CHECK: User can use whatever they want for initialization.
    /// SOL or SPL token account to receive Auction House fees. If treasury mint is native this will be the same as the `treasury_withdrawl_destination_owner`.
    #[account(mut)]
    pub treasury_withdrawal_destination: UncheckedAccount<'info>,

    /// CHECK: User can use whatever they want for initialization.
    /// Owner of the `treasury_withdrawal_destination` account or the same address if the `treasury_mint` is native.
    pub treasury_withdrawal_destination_owner: UncheckedAccount<'info>,

    /// Auction House instance PDA account.
    #[account(init, seeds=[PREFIX.as_bytes(), authority.key().as_ref(), treasury_mint.key().as_ref()], bump, space=AUCTION_HOUSE_SIZE, payer=payer)]
    pub auction_house: Account<'info, AuctionHouse>,

    /// Auction House instance fee account.
    /// CHECK: Not dangerous. Account seeds checked in constraint.
    #[account(mut, seeds=[PREFIX.as_bytes(), auction_house.key().as_ref(), FEE_PAYER.as_bytes()], bump)]
    pub auction_house_fee_account: UncheckedAccount<'info>,

    /// Auction House instance treasury PDA account.
    /// CHECK: Not dangerous. Account seeds checked in constraint.
    #[account(mut, seeds=[PREFIX.as_bytes(), auction_house.key().as_ref(), TREASURY.as_bytes()], bump)]
    pub auction_house_treasury: UncheckedAccount<'info>,

    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
    pub ata_program: Program<'info, AssociatedToken>,
    pub rent: Sysvar<'info, Rent>,
}

/// Accounts for the [`update_auction_house` handler](auction_house/fn.update_auction_house.html).
#[derive(Accounts)]
pub struct UpdateAuctionHouse<'info> {
    /// Treasury mint account, either native SOL mint or a SPL token mint.
    pub treasury_mint: Account<'info, Mint>,

    /// Key paying SOL fees for setting up the Auction House.
    pub payer: Signer<'info>,

    /// Authority key for the Auction House.
    pub authority: Signer<'info>,

    /// CHECK: User can use whatever they want for updating this.
    /// New authority key for the Auction House.
    pub new_authority: UncheckedAccount<'info>,

    /// CHECK: User can use whatever they want for updating this.
    /// Account that pays for fees if the marketplace executes sales.
    #[account(mut)]
    pub fee_withdrawal_destination: UncheckedAccount<'info>,

    /// CHECK: User can use whatever they want for updating this.
    /// SOL or SPL token account to receive Auction House fees. If treasury mint is native this will be the same as the `treasury_withdrawl_destination_owner`.
    #[account(mut)]
    pub treasury_withdrawal_destination: UncheckedAccount<'info>,

    /// CHECK: User can use whatever they want for updating this.
    /// Owner of the `treasury_withdrawal_destination` account or the same address if the `treasury_mint` is native.
    pub treasury_withdrawal_destination_owner: UncheckedAccount<'info>,

    /// Auction House instance PDA account.
    #[account(mut, seeds=[PREFIX.as_bytes(), auction_house.creator.as_ref(), treasury_mint.key().as_ref()], bump=auction_house.bump, has_one=authority, has_one=treasury_mint)]
    pub auction_house: Account<'info, AuctionHouse>,

    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
    pub ata_program: Program<'info, AssociatedToken>,
    pub rent: Sysvar<'info, Rent>,
}

/// Accounts for the [`withdraw_from_treasury` handler](auction_house/fn.withdraw_from_treasury.html).
#[derive(Accounts)]
pub struct WithdrawFromTreasury<'info> {
    /// Treasury mint account, either native SOL mint or a SPL token mint.
    pub treasury_mint: Account<'info, Mint>,

    /// Authority key for the Auction House.
    pub authority: Signer<'info>,

    /// SOL or SPL token account to receive Auction House fees. If treasury mint is native this will be the same as the `treasury_withdrawl_destination_owner`.
    /// CHECK: User can withdraw wherever they want as long as they sign as authority.
    #[account(mut)]
    pub treasury_withdrawal_destination: UncheckedAccount<'info>,

    /// Auction House treasury PDA account.
    /// CHECK: Not dangerous. Account seeds checked in constraint.
    #[account(mut, seeds=[PREFIX.as_bytes(), auction_house.key().as_ref(), TREASURY.as_bytes()], bump=auction_house.treasury_bump)]
    pub auction_house_treasury: UncheckedAccount<'info>,

    /// Auction House instance PDA account.
    #[account(mut, seeds=[PREFIX.as_bytes(), auction_house.creator.as_ref(), treasury_mint.key().as_ref()], bump=auction_house.bump, has_one=authority, has_one=treasury_mint, has_one=treasury_withdrawal_destination, has_one=auction_house_treasury)]
    pub auction_house: Account<'info, AuctionHouse>,

    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

/// Accounts for the [`withdraw_from_fee` handler](auction_house/fn.withdraw_from_fee.html).
#[derive(Accounts)]
pub struct WithdrawFromFee<'info> {
    /// Authority key for the Auction House.
    pub authority: Signer<'info>,

    /// Account that pays for fees if the marketplace executes sales.
    /// CHECK: User can withdraw wherever as long as they sign as authority.
    #[account(mut)]
    pub fee_withdrawal_destination: UncheckedAccount<'info>,

    /// Auction House instance fee account.
    /// CHECK: Not dangerous. Account seeds checked in constraint.
    #[account(mut, seeds=[PREFIX.as_bytes(), auction_house.key().as_ref(), FEE_PAYER.as_bytes()], bump=auction_house.fee_payer_bump)]
    pub auction_house_fee_account: UncheckedAccount<'info>,

    /// Auction House instance PDA account.
    #[account(mut, seeds=[PREFIX.as_bytes(), auction_house.creator.as_ref(), auction_house.treasury_mint.key().as_ref()], bump=auction_house.bump, has_one=authority, has_one=fee_withdrawal_destination, has_one=auction_house_fee_account)]
    pub auction_house: Account<'info, AuctionHouse>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(escrow_payment_bump: u8)]
pub struct CloseEscrowAccount<'info> {
    /// User wallet account.
    pub wallet: Signer<'info>,

    /// CHECK: Account seeds checked in constraint.
    /// Buyer escrow payment account PDA.
    #[account(mut, seeds=[PREFIX.as_bytes(), auction_house.key().as_ref(), wallet.key().as_ref()], bump=escrow_payment_bump)]
    pub escrow_payment_account: UncheckedAccount<'info>,

    /// Auction House instance PDA account.
    #[account(seeds=[PREFIX.as_bytes(), auction_house.creator.as_ref(), auction_house.treasury_mint.as_ref()], bump=auction_house.bump)]
    pub auction_house: Account<'info, AuctionHouse>,
    pub system_program: Program<'info, System>,
}
