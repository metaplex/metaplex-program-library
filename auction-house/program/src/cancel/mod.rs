use anchor_lang::{prelude::*, solana_program::program::invoke, AnchorDeserialize};
use solana_program::program_memory::sol_memset;

use crate::{constants::*, errors::*, utils::*, AuctionHouse, AuthorityScope, *};

use mpl_token_metadata::instruction::{builders::RevokeBuilder, InstructionBuilder, RevokeArgs};

/// Accounts for the [`cancel` handler](auction_house/fn.cancel.html).
#[derive(Accounts)]
#[instruction(buyer_price: u64, token_size: u64)]
pub struct Cancel<'info> {
    /// CHECK: Verified in cancel_logic.
    /// User wallet account.
    #[account(mut)]
    pub wallet: UncheckedAccount<'info>,

    /// SPL token account containing the token of the sale to be canceled.
    #[account(mut)]
    pub token_account: Box<Account<'info, TokenAccount>>,

    /// Token mint account of SPL token.
    pub token_mint: Box<Account<'info, Mint>>,

    /// CHECK: Validated as a signer in cancel_logic.
    /// Auction House instance authority account.
    pub authority: UncheckedAccount<'info>,

    /// Auction House instance PDA account.
    #[account(
        seeds = [
            PREFIX.as_bytes(),
            auction_house.creator.as_ref(),
            auction_house.treasury_mint.as_ref()
        ],
        bump=auction_house.bump,
        has_one=authority,
        has_one=auction_house_fee_account
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
        bump=auction_house.fee_payer_bump
    )]
    pub auction_house_fee_account: UncheckedAccount<'info>,

    /// CHECK: Validated in cancel_logic.
    /// Trade state PDA account representing the bid or ask to be canceled.
    #[account(mut)]
    pub trade_state: UncheckedAccount<'info>,

    pub token_program: Program<'info, Token>,
}

// this isn't for an ix, only here to help gather accounts
#[derive(Accounts)]
pub struct CancelRemainingAccounts<'info> {
    ///CHECK: checked in cancel function
    pub metadata_program: UncheckedAccount<'info>,
    ///CHECK: checked in cpi
    #[account(mut)]
    pub delegate_record: UncheckedAccount<'info>,
    ///CHECK: checked in cpi
    pub program_as_signer: UncheckedAccount<'info>,
    ///CHECK: checked in cpi
    #[account(mut)]
    pub metadata: UncheckedAccount<'info>,
    ///CHECK: checked in cpi
    pub edition: UncheckedAccount<'info>,
    ///CHECK: checked in cpi
    #[account(mut)]
    pub token_record: UncheckedAccount<'info>,
    ///CHECK: checked in cpi
    pub token_mint: UncheckedAccount<'info>,
    ///CHECK: checked in cpi
    pub auth_rules_program: UncheckedAccount<'info>,
    ///CHECK: checked in cpi
    pub auth_rules: UncheckedAccount<'info>,
    ///CHECK: checked in cpi
    pub sysvar_instructions: UncheckedAccount<'info>,
    ///CHECK: checked in cpi
    pub system_program: UncheckedAccount<'info>,
}

impl<'info> From<AuctioneerCancel<'info>> for Cancel<'info> {
    fn from(a: AuctioneerCancel<'info>) -> Cancel<'info> {
        Cancel {
            wallet: a.wallet,
            token_account: a.token_account,
            token_mint: a.token_mint,
            authority: a.authority,
            auction_house: a.auction_house,
            auction_house_fee_account: a.auction_house_fee_account,
            trade_state: a.trade_state,
            token_program: a.token_program,
        }
    }
}

/// Accounts for the [`auctioneer_cancel` handler](auction_house/fn.auctioneer_cancel.html).
#[derive(Accounts, Clone)]
#[instruction(buyer_price: u64, token_size: u64)]
pub struct AuctioneerCancel<'info> {
    /// CHECK: Wallet validated as owner in cancel logic.
    /// User wallet account.
    #[account(mut)]
    pub wallet: UncheckedAccount<'info>,

    /// SPL token account containing the token of the sale to be canceled.
    #[account(mut)]
    pub token_account: Box<Account<'info, TokenAccount>>,

    /// Token mint account of SPL token.
    pub token_mint: Box<Account<'info, Mint>>,

    /// CHECK: Validated as a signer in cancel_logic.
    /// Auction House instance authority account.
    pub authority: UncheckedAccount<'info>,

    /// CHECK: Validated in ah_auctioneer_pda seeds and as a signer in cancel_logic.
    /// The auctioneer authority - typically a PDA of the Auctioneer program running this action.
    pub auctioneer_authority: Signer<'info>,

    /// Auction House instance PDA account.
    #[account(
        seeds = [
            PREFIX.as_bytes(),
            auction_house.creator.as_ref(),
            auction_house.treasury_mint.as_ref()
        ],
        bump=auction_house.bump,
        has_one=authority,
        has_one=auction_house_fee_account
    )]
    pub auction_house: Box<Account<'info, AuctionHouse>>,

    /// CHECK: Validated in cancel_logic.
    /// Auction House instance fee account.
    #[account(
        mut,
        seeds = [
            PREFIX.as_bytes(),
            auction_house.key().as_ref(),
            FEE_PAYER.as_bytes()
        ],
        bump=auction_house.fee_payer_bump
    )]
    pub auction_house_fee_account: UncheckedAccount<'info>,

    /// CHECK: Validated in cancel_logic.
    /// Trade state PDA account representing the bid or ask to be canceled.
    #[account(mut)]
    pub trade_state: UncheckedAccount<'info>,

    /// CHECK: Validated in cancel_logic.
    /// The auctioneer PDA owned by Auction House storing scopes.
    #[account(
        seeds = [
            AUCTIONEER.as_bytes(),
            auction_house.key().as_ref(),
            auctioneer_authority.key().as_ref()
        ],
        bump = ah_auctioneer_pda.bump
    )]
    pub ah_auctioneer_pda: Account<'info, Auctioneer>,

    pub token_program: Program<'info, Token>,
}

// Cancel a bid or ask by revoking the token delegate, transferring all lamports from the trade state account to the fee payer, and setting the trade state account data to zero so it can be garbage collected.
pub fn cancel<'info>(
    ctx: Context<'_, '_, '_, 'info, Cancel<'info>>,
    buyer_price: u64,
    token_size: u64,
) -> Result<()> {
    let auction_house = &ctx.accounts.auction_house;

    // If it has an auctioneer authority delegated must use auctioneer_* handler.
    if auction_house.has_auctioneer && auction_house.scopes[AuthorityScope::Cancel as usize] {
        return Err(AuctionHouseError::MustUseAuctioneerHandler.into());
    }

    cancel_logic(
        ctx.accounts,
        ctx.remaining_accounts,
        buyer_price,
        token_size,
    )
}

pub fn auctioneer_cancel<'info>(
    ctx: Context<'_, '_, '_, 'info, AuctioneerCancel<'info>>,
    buyer_price: u64,
    token_size: u64,
) -> Result<()> {
    let auction_house = &ctx.accounts.auction_house;
    let auctioneer_authority = &ctx.accounts.auctioneer_authority;
    let ah_auctioneer_pda = &ctx.accounts.ah_auctioneer_pda;

    if !auction_house.has_auctioneer {
        return Err(AuctionHouseError::NoAuctioneerProgramSet.into());
    }

    assert_valid_auctioneer_and_scope(
        auction_house,
        &auctioneer_authority.key(),
        ah_auctioneer_pda,
        AuthorityScope::Cancel,
    )?;

    let mut accounts: Cancel<'info> = (*ctx.accounts).clone().into();

    cancel_logic(
        &mut accounts,
        ctx.remaining_accounts,
        buyer_price,
        token_size,
    )
}

#[allow(clippy::needless_lifetimes)]
fn cancel_logic<'c, 'info>(
    accounts: &mut Cancel<'info>,
    remaining_accounts: &'c [AccountInfo<'info>],
    buyer_price: u64,
    token_size: u64,
) -> Result<()> {
    let wallet = &accounts.wallet;
    let token_account = &accounts.token_account;
    let token_mint = &accounts.token_mint;
    let authority = &accounts.authority;
    let auction_house = &accounts.auction_house;
    let auction_house_fee_account = &accounts.auction_house_fee_account;
    let trade_state = &accounts.trade_state;
    let token_program = &accounts.token_program;

    let ts_bump = trade_state.try_borrow_data()?[0];
    assert_valid_trade_state(
        &wallet.key(),
        auction_house,
        buyer_price,
        token_size,
        &trade_state.to_account_info(),
        &token_account.mint.key(),
        &token_account.key(),
        ts_bump,
    )?;
    assert_keys_equal(token_mint.key(), token_account.mint)?;
    if !wallet.to_account_info().is_signer && !authority.to_account_info().is_signer {
        return Err(AuctionHouseError::NoValidSignerPresent.into());
    }

    let auction_house_key = auction_house.key();
    let seeds = [
        PREFIX.as_bytes(),
        auction_house_key.as_ref(),
        FEE_PAYER.as_bytes(),
        &[auction_house.fee_payer_bump],
    ];

    let (fee_payer, _) = get_fee_payer(
        authority,
        auction_house,
        wallet.to_account_info(),
        auction_house_fee_account.to_account_info(),
        &seeds,
    )?;

    let remaining_accounts = &mut remaining_accounts.iter();

    if token_account.owner == wallet.key() && wallet.is_signer {
        match next_account_info(remaining_accounts) {
            Ok(metadata_program) => {
                require!(
                    metadata_program.key() == mpl_token_metadata::ID,
                    AuctionHouseError::PublicKeyMismatch
                );

                let delegate_record = next_account_info(remaining_accounts)?;
                let program_as_signer = next_account_info(remaining_accounts)?;
                let metadata = next_account_info(remaining_accounts)?;
                let edition = next_account_info(remaining_accounts)?;
                let token_record = next_account_info(remaining_accounts)?;
                let token_mint = next_account_info(remaining_accounts)?;
                let auth_rules_program = next_account_info(remaining_accounts)?;
                let auth_rules = next_account_info(remaining_accounts)?;
                let sysvar_instructions = next_account_info(remaining_accounts)?;
                let system_program = next_account_info(remaining_accounts)?;

                let revoke = RevokeBuilder::new()
                    .delegate_record(delegate_record.key())
                    .delegate(program_as_signer.key())
                    .metadata(metadata.key())
                    .master_edition(edition.key())
                    .token_record(token_record.key())
                    .mint(token_mint.key())
                    .token(token_account.key())
                    .authority(wallet.key())
                    .payer(wallet.key())
                    .system_program(system_program.key())
                    .sysvar_instructions(sysvar_instructions.key())
                    .spl_token_program(token_program.key())
                    .authorization_rules_program(auth_rules_program.key())
                    .authorization_rules(auth_rules.key())
                    .build(RevokeArgs::SaleV1)
                    .unwrap()
                    .instruction();

                let revoke_accounts = [
                    wallet.to_account_info(),
                    program_as_signer.to_account_info(),
                    metadata_program.to_account_info(),
                    delegate_record.to_account_info(),
                    authority.to_account_info(),
                    metadata.to_account_info(),
                    token_record.to_account_info(),
                    edition.to_account_info(),
                    token_account.to_account_info(),
                    wallet.to_account_info(),
                    token_mint.to_account_info(),
                    system_program.to_account_info(),
                    sysvar_instructions.to_account_info(),
                    token_program.to_account_info(),
                    auth_rules_program.to_account_info(),
                    auth_rules.to_account_info(),
                ];

                invoke(&revoke, &revoke_accounts)?;
            }
            Err(_) => {
                invoke(
                    &revoke(
                        &token_program.key(),
                        &token_account.key(),
                        &wallet.key(),
                        &[],
                    )
                    .unwrap(),
                    &[
                        token_program.to_account_info(),
                        token_account.to_account_info(),
                        wallet.to_account_info(),
                    ],
                )?;
            }
        }
    }

    let curr_lamp = trade_state.lamports();
    **trade_state.lamports.borrow_mut() = 0;

    **fee_payer.lamports.borrow_mut() = fee_payer
        .lamports()
        .checked_add(curr_lamp)
        .ok_or(AuctionHouseError::NumericalOverflow)?;

    #[allow(clippy::explicit_auto_deref)]
    sol_memset(*trade_state.try_borrow_mut_data()?, 0, TRADE_STATE_SIZE);

    Ok(())
}
