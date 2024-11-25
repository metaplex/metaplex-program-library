#![cfg(feature = "test-bpf")]
pub mod common;
pub mod utils;

use common::*;
use mpl_auctioneer::pda::*;
use solana_sdk::signature::Keypair;
use std::time::SystemTime;
use utils::setup_functions::*;

#[tokio::test]
async fn cancel_listing() {
    let mut context = auctioneer_program_test().start_with_context().await;
    // Payer Wallet
    let (ah, ahkey, _) = existing_auction_house_test_context(&mut context)
        .await
        .unwrap();
    let test_metadata = Metadata::new();
    airdrop(
        &mut context,
        &test_metadata.token.pubkey(),
        100_000_000_000_000,
    )
    .await
    .unwrap();
    test_metadata
        .create(
            &mut context,
            "Tests".to_string(),
            "TST".to_string(),
            "uri".to_string(),
            None,
            10,
            false,
            1,
        )
        .await
        .unwrap();
    context.warp_to_slot(100).unwrap();
    // Derive Auction House Key
    let ((acc, listing_config_address), sell_tx) = sell(
        &mut context,
        &ahkey,
        &ah,
        &test_metadata,
        (SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .expect("Time went backwards")
            .as_secs()) as i64,
        (SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .expect("Time went backwards")
            .as_secs()
            + 60) as i64,
        None,
        None,
        None,
        None,
        None,
    );
    context
        .banks_client
        .process_transaction(sell_tx)
        .await
        .unwrap();
    let token =
        get_associated_token_address(&test_metadata.token.pubkey(), &test_metadata.mint.pubkey());
    let (auctioneer_authority, aa_bump) = find_auctioneer_authority_seeds(&ahkey);
    let (auctioneer_pda, _) = find_auctioneer_pda(&ahkey, &auctioneer_authority);
    let accounts = mpl_auctioneer::accounts::AuctioneerCancel {
        auction_house_program: mpl_auction_house::id(),
        listing_config: listing_config_address,
        seller: acc.wallet,
        auction_house: ahkey,
        wallet: test_metadata.token.pubkey(),
        token_account: token,
        authority: ah.authority,
        trade_state: acc.seller_trade_state,
        token_program: spl_token::id(),
        token_mint: test_metadata.mint.pubkey(),
        auction_house_fee_account: ah.auction_house_fee_account,
        auctioneer_authority,
        ah_auctioneer_pda: auctioneer_pda,
    }
    .to_account_metas(None);
    let instruction = Instruction {
        program_id: mpl_auctioneer::id(),
        data: mpl_auctioneer::instruction::Cancel {
            auctioneer_authority_bump: aa_bump,
            buyer_price: u64::MAX,
            token_size: 1,
        }
        .data(),
        accounts,
    };

    let tx = Transaction::new_signed_with_payer(
        &[instruction],
        Some(&test_metadata.token.pubkey()),
        &[&test_metadata.token],
        context.last_blockhash,
    );

    context.banks_client.process_transaction(tx).await.unwrap();

    let listing_config_closed = context
        .banks_client
        .get_account(listing_config_address)
        .await
        .unwrap();

    assert!(listing_config_closed.is_none());
}

#[tokio::test]
async fn cancel_bid() {
    let mut context = auctioneer_program_test().start_with_context().await;
    // Payer Wallet
    let (ah, ahkey, _) = existing_auction_house_test_context(&mut context)
        .await
        .unwrap();
    let test_metadata = Metadata::new();
    airdrop(&mut context, &test_metadata.token.pubkey(), 1000000000)
        .await
        .unwrap();
    test_metadata
        .create(
            &mut context,
            "Tests".to_string(),
            "TST".to_string(),
            "uri".to_string(),
            None,
            10,
            false,
            1,
        )
        .await
        .unwrap();

    let price = 1000000000;

    let ((sell_acc, listing_config_address), sell_tx) = sell(
        &mut context,
        &ahkey,
        &ah,
        &test_metadata,
        (SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .expect("Time went backwards")
            .as_secs()
            - 60) as i64,
        (SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .expect("Time went backwards")
            .as_secs()
            + 60) as i64,
        None,
        None,
        None,
        None,
        Some(true),
    );
    context
        .banks_client
        .process_transaction(sell_tx)
        .await
        .unwrap();

    context.warp_to_slot(100).unwrap();
    let buyer = Keypair::new();
    // Derive Auction House Key
    airdrop(&mut context, &buyer.pubkey(), 2000000000)
        .await
        .unwrap();
    let (acc, buy_tx) = buy(
        &mut context,
        &ahkey,
        &ah,
        &test_metadata,
        &test_metadata.token.pubkey(),
        &buyer,
        &sell_acc.wallet,
        &listing_config_address,
        price,
    );

    context
        .banks_client
        .process_transaction(buy_tx)
        .await
        .unwrap();
    let (auctioneer_authority, aa_bump) = find_auctioneer_authority_seeds(&ahkey);
    let (auctioneer_pda, _) = find_auctioneer_pda(&ahkey, &auctioneer_authority);
    let accounts = mpl_auctioneer::accounts::AuctioneerCancel {
        auction_house_program: mpl_auction_house::id(),
        listing_config: listing_config_address,
        seller: sell_acc.wallet,
        auction_house: ahkey,
        wallet: buyer.pubkey(),
        token_account: acc.token_account,
        authority: ah.authority,
        trade_state: acc.buyer_trade_state,
        token_program: spl_token::id(),
        token_mint: test_metadata.mint.pubkey(),
        auction_house_fee_account: ah.auction_house_fee_account,
        auctioneer_authority,
        ah_auctioneer_pda: auctioneer_pda,
    }
    .to_account_metas(None);
    let instruction = Instruction {
        program_id: mpl_auctioneer::id(),
        data: mpl_auctioneer::instruction::Cancel {
            auctioneer_authority_bump: aa_bump,
            buyer_price: price,
            token_size: 1,
        }
        .data(),
        accounts,
    };

    let tx = Transaction::new_signed_with_payer(
        &[instruction],
        Some(&buyer.pubkey()),
        &[&buyer],
        context.last_blockhash,
    );
    context.banks_client.process_transaction(tx).await.unwrap();

    // Make sure the trade state wasn't erroneously closed.
    let listing_config_closed = context
        .banks_client
        .get_account(listing_config_address)
        .await
        .unwrap();

    assert!(listing_config_closed.is_some());
}

#[tokio::test]
async fn cancel_highest_bid() {
    let mut context = auctioneer_program_test().start_with_context().await;
    // Payer Wallet
    let (ah, ahkey, _) = existing_auction_house_test_context(&mut context)
        .await
        .unwrap();
    let test_metadata = Metadata::new();
    airdrop(&mut context, &test_metadata.token.pubkey(), 1000000000)
        .await
        .unwrap();
    test_metadata
        .create(
            &mut context,
            "Tests".to_string(),
            "TST".to_string(),
            "uri".to_string(),
            None,
            10,
            false,
            1,
        )
        .await
        .unwrap();

    let price = 1000000000;

    let ((sell_acc, listing_config_address), sell_tx) = sell(
        &mut context,
        &ahkey,
        &ah,
        &test_metadata,
        (SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .expect("Time went backwards")
            .as_secs()
            - 60) as i64,
        (SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .expect("Time went backwards")
            .as_secs()
            + 60) as i64,
        None,
        None,
        None,
        None,
        Some(false),
    );
    context
        .banks_client
        .process_transaction(sell_tx)
        .await
        .unwrap();

    context.warp_to_slot(100).unwrap();
    let buyer0 = Keypair::new();
    // Derive Auction House Key
    airdrop(&mut context, &buyer0.pubkey(), 2000000000)
        .await
        .unwrap();
    let (acc0, buy_tx0) = buy(
        &mut context,
        &ahkey,
        &ah,
        &test_metadata,
        &test_metadata.token.pubkey(),
        &buyer0,
        &sell_acc.wallet,
        &listing_config_address,
        price,
    );

    context
        .banks_client
        .process_transaction(buy_tx0)
        .await
        .unwrap();

    context.warp_to_slot(200).unwrap();

    let (auctioneer_authority, aa_bump) = find_auctioneer_authority_seeds(&ahkey);
    let (auctioneer_pda, _) = find_auctioneer_pda(&ahkey, &auctioneer_authority);
    let accounts0 = mpl_auctioneer::accounts::AuctioneerCancel {
        auction_house_program: mpl_auction_house::id(),
        listing_config: listing_config_address,
        seller: sell_acc.wallet,
        auction_house: ahkey,
        wallet: buyer0.pubkey(),
        token_account: acc0.token_account,
        authority: ah.authority,
        trade_state: acc0.buyer_trade_state,
        token_program: spl_token::id(),
        token_mint: test_metadata.mint.pubkey(),
        auction_house_fee_account: ah.auction_house_fee_account,
        auctioneer_authority,
        ah_auctioneer_pda: auctioneer_pda,
    }
    .to_account_metas(None);
    let instruction0 = Instruction {
        program_id: mpl_auctioneer::id(),
        data: mpl_auctioneer::instruction::Cancel {
            auctioneer_authority_bump: aa_bump,
            buyer_price: price,
            token_size: 1,
        }
        .data(),
        accounts: accounts0,
    };

    let tx0 = Transaction::new_signed_with_payer(
        &[instruction0],
        Some(&buyer0.pubkey()),
        &[&buyer0],
        context.last_blockhash,
    );
    let result0 = context
        .banks_client
        .process_transaction(tx0)
        .await
        .unwrap_err();
    assert_error!(result0, CANNOT_CANCEL_HIGHEST_BID);

    context.warp_to_slot(300).unwrap();

    // Buyer 1 bids higher and should now be the highest bidder.
    let buyer1 = Keypair::new();
    // Derive Auction House Key
    airdrop(&mut context, &buyer1.pubkey(), 2000000000)
        .await
        .unwrap();
    let (acc1, buy_tx1) = buy(
        &mut context,
        &ahkey,
        &ah,
        &test_metadata,
        &test_metadata.token.pubkey(),
        &buyer1,
        &sell_acc.wallet,
        &listing_config_address,
        price + 1,
    );

    context
        .banks_client
        .process_transaction(buy_tx1)
        .await
        .unwrap();
    context.warp_to_slot(400).unwrap();

    let (auctioneer_authority, aa_bump) = find_auctioneer_authority_seeds(&ahkey);
    let (auctioneer_pda, _) = find_auctioneer_pda(&ahkey, &auctioneer_authority);
    let accounts1 = mpl_auctioneer::accounts::AuctioneerCancel {
        auction_house_program: mpl_auction_house::id(),
        listing_config: listing_config_address,
        seller: sell_acc.wallet,
        auction_house: ahkey,
        wallet: buyer1.pubkey(),
        token_account: acc1.token_account,
        authority: ah.authority,
        trade_state: acc1.buyer_trade_state,
        token_program: spl_token::id(),
        token_mint: test_metadata.mint.pubkey(),
        auction_house_fee_account: ah.auction_house_fee_account,
        auctioneer_authority,
        ah_auctioneer_pda: auctioneer_pda,
    }
    .to_account_metas(None);
    let instruction1 = Instruction {
        program_id: mpl_auctioneer::id(),
        data: mpl_auctioneer::instruction::Cancel {
            auctioneer_authority_bump: aa_bump,
            buyer_price: price + 1,
            token_size: 1,
        }
        .data(),
        accounts: accounts1,
    };

    let tx1 = Transaction::new_signed_with_payer(
        &[instruction1],
        Some(&buyer1.pubkey()),
        &[&buyer1],
        context.last_blockhash,
    );

    let result1 = context
        .banks_client
        .process_transaction(tx1)
        .await
        .unwrap_err();
    assert_error!(result1, CANNOT_CANCEL_HIGHEST_BID);
    context.warp_to_slot(500).unwrap();

    // Rerun the cancel on the lower bid to verify it now succeeds.
    let (auctioneer_authority, aa_bump) = find_auctioneer_authority_seeds(&ahkey);
    let (auctioneer_pda, _) = find_auctioneer_pda(&ahkey, &auctioneer_authority);
    let accounts2 = mpl_auctioneer::accounts::AuctioneerCancel {
        auction_house_program: mpl_auction_house::id(),
        listing_config: listing_config_address,
        seller: sell_acc.wallet,
        auction_house: ahkey,
        wallet: buyer0.pubkey(),
        token_account: acc0.token_account,
        authority: ah.authority,
        trade_state: acc0.buyer_trade_state,
        token_program: spl_token::id(),
        token_mint: test_metadata.mint.pubkey(),
        auction_house_fee_account: ah.auction_house_fee_account,
        auctioneer_authority,
        ah_auctioneer_pda: auctioneer_pda,
    }
    .to_account_metas(None);
    let instruction2 = Instruction {
        program_id: mpl_auctioneer::id(),
        data: mpl_auctioneer::instruction::Cancel {
            auctioneer_authority_bump: aa_bump,
            buyer_price: price,
            token_size: 1,
        }
        .data(),
        accounts: accounts2,
    };

    let tx2 = Transaction::new_signed_with_payer(
        &[instruction2],
        Some(&buyer0.pubkey()),
        &[&buyer0],
        context.last_blockhash,
    );
    context.banks_client.process_transaction(tx2).await.unwrap();
}

#[tokio::test]
async fn cancel_seller_bid() {
    let mut context = auctioneer_program_test().start_with_context().await;
    // Payer Wallet
    let (ah, ahkey, _) = existing_auction_house_test_context(&mut context)
        .await
        .unwrap();
    let test_metadata = Metadata::new();
    airdrop(&mut context, &test_metadata.token.pubkey(), 1000000000)
        .await
        .unwrap();
    test_metadata
        .create(
            &mut context,
            "Tests".to_string(),
            "TST".to_string(),
            "uri".to_string(),
            None,
            10,
            false,
            1,
        )
        .await
        .unwrap();

    let price = 1000000000;

    let ((sell_acc, listing_config_address), sell_tx) = sell(
        &mut context,
        &ahkey,
        &ah,
        &test_metadata,
        (SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .expect("Time went backwards")
            .as_secs()
            - 60) as i64,
        (SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .expect("Time went backwards")
            .as_secs()
            + 60) as i64,
        None,
        None,
        None,
        None,
        Some(true),
    );
    context
        .banks_client
        .process_transaction(sell_tx)
        .await
        .unwrap();

    context.warp_to_slot(100).unwrap();
    // Buyer = Seller : The seller bids on its own listing.
    // Derive Auction House Key
    airdrop(&mut context, &test_metadata.token.pubkey(), 2000000000)
        .await
        .unwrap();
    let (acc, buy_tx) = buy(
        &mut context,
        &ahkey,
        &ah,
        &test_metadata,
        &test_metadata.token.pubkey(),
        &test_metadata.token,
        &sell_acc.wallet,
        &listing_config_address,
        price,
    );

    context
        .banks_client
        .process_transaction(buy_tx)
        .await
        .unwrap();
    let (auctioneer_authority, aa_bump) = find_auctioneer_authority_seeds(&ahkey);
    let (auctioneer_pda, _) = find_auctioneer_pda(&ahkey, &auctioneer_authority);
    let accounts = mpl_auctioneer::accounts::AuctioneerCancel {
        auction_house_program: mpl_auction_house::id(),
        listing_config: listing_config_address,
        seller: sell_acc.wallet,
        auction_house: ahkey,
        wallet: test_metadata.token.pubkey(),
        token_account: acc.token_account,
        authority: ah.authority,
        trade_state: acc.buyer_trade_state,
        token_program: spl_token::id(),
        token_mint: test_metadata.mint.pubkey(),
        auction_house_fee_account: ah.auction_house_fee_account,
        auctioneer_authority,
        ah_auctioneer_pda: auctioneer_pda,
    }
    .to_account_metas(None);
    let instruction = Instruction {
        program_id: mpl_auctioneer::id(),
        data: mpl_auctioneer::instruction::Cancel {
            auctioneer_authority_bump: aa_bump,
            buyer_price: price,
            token_size: 1,
        }
        .data(),
        accounts,
    };

    let tx = Transaction::new_signed_with_payer(
        &[instruction],
        Some(&test_metadata.token.pubkey()),
        &[&test_metadata.token],
        context.last_blockhash,
    );
    context.banks_client.process_transaction(tx).await.unwrap();

    // Make sure the trade state wasn't erroneously closed.
    let listing_config_closed = context
        .banks_client
        .get_account(listing_config_address)
        .await
        .unwrap();

    assert!(listing_config_closed.is_some());
}