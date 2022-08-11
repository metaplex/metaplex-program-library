use anchor_lang::{context::Context, prelude::*, AnchorDeserialize, InstructionData};
use anchor_spl::token::{Token, TokenAccount};
use solana_program::{instruction::Instruction, program::invoke_signed};

use crate::{
    assertions::assert_belongs_to_rewardable_collection,
    constants::{LISTING, REWARDABLE_COLLECTION, REWARD_CENTER},
    errors::ListingRewardsError,
    state::{Listing, RewardCenter, RewardableCollection},
    MetadataAccount,
};
use mpl_auction_house::{
    constants::{AUCTIONEER, FEE_PAYER, PREFIX, SIGNER},
    cpi::accounts::AuctioneerSell,
    instruction::AuctioneerSell as AuctioneerSellParams,
    program::AuctionHouse as AuctionHouseProgram,
    AuctionHouse,
};

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct CreateListingParams {
    pub price: u64,
    pub token_size: u64,
    pub trade_state_bump: u8,
    pub free_trade_state_bump: u8,
    pub program_as_signer_bump: u8,
}

/// Accounts for the [`sell` handler](listing_rewards/fn.sell.html).
#[derive(Accounts, Clone)]
#[instruction(sell_params: CreateListingParams)]
pub struct CreateListing<'info> {
    /// Auction House Program used for CPI call
    pub auction_house_program: Program<'info, AuctionHouseProgram>,

    // Accounts used for Auctioneer
    /// The Listing Config used for listing settings
    #[account(
        init,
        payer = wallet,
        space = Listing::size(),
        seeds = [
            LISTING.as_bytes(),
            wallet.key().as_ref(),
            metadata.key().as_ref(),
            rewardable_collection.key().as_ref(),
        ],
        bump,
    )]
    pub listing: Account<'info, Listing>,

    /// The auctioneer program PDA running this auction.
    #[account(seeds = [REWARD_CENTER.as_bytes(), auction_house.key().as_ref()], bump = reward_center.bump)]
    pub reward_center: Box<Account<'info, RewardCenter>>,

    /// The collection eligable for rewards
    #[account(
        seeds = [
            REWARDABLE_COLLECTION.as_bytes(),
            reward_center.key().as_ref(),
            metadata.collection.as_ref().ok_or(ListingRewardsError::NFTMissingCollection)?.key.as_ref()
        ],
        bump = rewardable_collection.bump
    )]
    pub rewardable_collection: Box<Account<'info, RewardableCollection>>,

    // Accounts passed into Auction House CPI call
    /// User wallet account.
    #[account(mut)]
    pub wallet: Signer<'info>,

    /// SPL token account containing token for sale.
    #[account(
        mut,
        constraint = token_account.owner == wallet.key(),
        constraint = token_account.amount == 1
    )]
    pub token_account: Box<Account<'info, TokenAccount>>,

    /// Metaplex metadata account decorating SPL mint account.
    pub metadata: Box<Account<'info, MetadataAccount>>,

    /// CHECK: Verified through CPI
    /// Auction House authority account.
    pub authority: UncheckedAccount<'info>,

    /// Auction House instance PDA account.
    #[account(
        seeds = [
            PREFIX.as_bytes(),
            auction_house.creator.as_ref(),
            auction_house.treasury_mint.as_ref()
        ],
        seeds::program = auction_house_program,
        bump = auction_house.bump,
        has_one = auction_house_fee_account
    )]
    pub auction_house: Box<Account<'info, AuctionHouse>>,

    /// CHECK: Not dangerous. Account seeds checked in constraint.
    /// Auction House instance fee account.
    #[account(
        mut,
        seeds = [
            PREFIX.as_bytes(),
            auction_house.key().as_ref(),
            FEE_PAYER.as_bytes()
        ],
        seeds::program = auction_house_program,
        bump = auction_house.fee_payer_bump
    )]
    pub auction_house_fee_account: UncheckedAccount<'info>,

    /// CHECK: Not dangerous. Account seeds checked in constraint.
    /// Seller trade state PDA account encoding the sell order.
    #[account(
        mut, 
        seeds = [
            PREFIX.as_bytes(),
            wallet.key().as_ref(),
            auction_house.key().as_ref(),
            token_account.key().as_ref(),
            auction_house.treasury_mint.as_ref(),
            token_account.mint.as_ref(),
            &u64::MAX.to_le_bytes(),
            &sell_params.token_size.to_le_bytes()
        ],
        seeds::program = auction_house_program,
        bump = sell_params.trade_state_bump
    )]
    pub seller_trade_state: UncheckedAccount<'info>,

    /// CHECK: Not dangerous. Account seeds checked in constraint.
    /// Free seller trade state PDA account encoding a free sell order.
    #[account(
        mut, 
        seeds = [
            PREFIX.as_bytes(),
            wallet.key().as_ref(),
            auction_house.key().as_ref(),
            token_account.key().as_ref(),
            auction_house.treasury_mint.as_ref(),
            token_account.mint.as_ref(),
            &0u64.to_le_bytes(),
            &sell_params.token_size.to_le_bytes()
        ],
        seeds::program = auction_house_program,
        bump = sell_params.free_trade_state_bump
    )]
    pub free_seller_trade_state: UncheckedAccount<'info>,

    /// CHECK: Not dangerous. Account seeds checked in constraint.
    /// The auctioneer PDA owned by Auction House storing scopes.
    #[account(
        seeds = [
            AUCTIONEER.as_bytes(),
            auction_house.key().as_ref(),
            reward_center.key().as_ref()
        ],
        seeds::program = auction_house_program,
        bump = auction_house.auctioneer_pda_bump
    )]
    pub ah_auctioneer_pda: UncheckedAccount<'info>,

    /// CHECK: Not dangerous. Account seeds checked in constraint.
    #[account(
        seeds=[
            PREFIX.as_bytes(),
            SIGNER.as_bytes()
        ],
        seeds::program = auction_house_program,
        bump = sell_params.program_as_signer_bump
    )]
    pub program_as_signer: UncheckedAccount<'info>,

    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

pub fn handler(
    ctx: Context<CreateListing>,
    CreateListingParams {
        token_size,
        trade_state_bump,
        free_trade_state_bump,
        program_as_signer_bump,
        price,
    }: CreateListingParams,
) -> Result<()> {
    let metadata = &ctx.accounts.metadata;
    let reward_center = &ctx.accounts.reward_center;
    let auction_house = &ctx.accounts.auction_house;
    let rewardable_collection = &ctx.accounts.rewardable_collection;
    let metadata_account = &ctx.accounts.metadata;
    let wallet = &ctx.accounts.wallet;
    let clock = Clock::get()?;
    let auction_house_key = auction_house.key();

    assert_belongs_to_rewardable_collection(metadata, rewardable_collection)?;

    let listing = &mut ctx.accounts.listing;

    listing.reward_center = reward_center.key();
    listing.seller = wallet.key();
    listing.metadata = metadata_account.key();
    listing.rewardable_collection = rewardable_collection.key();
    listing.price = price;
    listing.token_size = token_size;
    listing.bump = *ctx
        .bumps
        .get(LISTING)
        .ok_or(ListingRewardsError::BumpSeedNotInHashMap)?;
    listing.created_at = clock.unix_timestamp;
    listing.canceled_at = None;
    listing.reward_redeemed_at = None;

    let reward_center_signer_seeds: &[&[&[u8]]] = &[&[
        REWARD_CENTER.as_bytes(),
        auction_house_key.as_ref(),
        &[reward_center.bump],
    ]];

    let auction_house_program = ctx.accounts.auction_house_program.to_account_info();

    let create_listing_ctx_accounts = AuctioneerSell {
        wallet: ctx.accounts.wallet.to_account_info(),
        token_account: ctx.accounts.token_account.to_account_info(),
        metadata: ctx.accounts.metadata.to_account_info(),
        auction_house: ctx.accounts.auction_house.to_account_info(),
        auction_house_fee_account: ctx.accounts.auction_house_fee_account.to_account_info(),
        seller_trade_state: ctx.accounts.seller_trade_state.to_account_info(),
        free_seller_trade_state: ctx.accounts.free_seller_trade_state.to_account_info(),
        authority: ctx.accounts.authority.to_account_info(),
        auctioneer_authority: ctx.accounts.reward_center.to_account_info(),
        ah_auctioneer_pda: ctx.accounts.ah_auctioneer_pda.to_account_info(),
        token_program: ctx.accounts.token_program.to_account_info(),
        system_program: ctx.accounts.system_program.to_account_info(),
        program_as_signer: ctx.accounts.program_as_signer.to_account_info(),
        rent: ctx.accounts.rent.to_account_info(),
    };

    let create_listing_params = AuctioneerSellParams {
        trade_state_bump,
        free_trade_state_bump,
        program_as_signer_bump,
        token_size,
    };

    let signer_required_keys = vec![
        ctx.accounts.reward_center.key(),
        ctx.accounts.wallet.key(),
    ];

    let create_listing_ix = Instruction {
        program_id: auction_house_program.key(),
        data: create_listing_params.data(),
        accounts: create_listing_ctx_accounts.to_account_metas(None).into_iter().map(|mut account| {
            if signer_required_keys.contains(&account.pubkey) {
                account.is_signer = true;
            }
            account
        }).collect()
    };

    invoke_signed(&create_listing_ix, &create_listing_ctx_accounts.to_account_infos(), &reward_center_signer_seeds)?;

    Ok(())
}
