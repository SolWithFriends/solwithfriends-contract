use anchor_lang::prelude::*;
use crate::states::Table;
use crate::states::{Hand, Card, PlayerHandResult, GameResult, ProgramState, ShuffleAction};
use crate::errors::ErrorCode::*;
use sha2::{Sha256, Digest};

pub fn deal_cards(ctx: Context<DealCardsCtx>, client_entropy: [u8; 32]) -> Result<()> {
    let table = &mut ctx.accounts.table;

    if ctx.accounts.program_state.site_authority != ctx.accounts.authority.key() {
        return Err(Unauthorized.into());
    }

    if table.game_status != 1 { return Err(InvalidGameState.into()); }
    if table.seats.iter().all(|seat| seat.bet == 0) { return Err(NoBetsPlaced.into()); }
    let table_key = table.key();
  

    table.check_and_reshuffle(
        ShuffleAction::Deal,
        &ctx.accounts.recent_slothashes,
        client_entropy,
        table_key
    )?;

    let deck = table.deck.clone();
    let mut used_card_indexes = table.used_card_indexes.clone();
    
    // Deal cards to players
    for seat in table.seats.iter_mut().filter(|seat| seat.player.is_some() && seat.bet > 0) {
        let mut player_hand = Hand::new();
       
        for _ in 0..2 {
           let next_card = get_random_unused_card(&ctx.accounts.recent_slothashes,  table_key, &mut used_card_indexes, client_entropy)?;
            player_hand.add_card(deck[next_card as usize]);
        }
        player_hand.calculate_value();
        seat.hand = Some(player_hand);
        seat.in_action = true;
    }
    

    // Deal dealer's hand
    let mut dealer_hand = Hand::new();
    let next_card = get_random_unused_card(&ctx.accounts.recent_slothashes,  table_key, &mut used_card_indexes, client_entropy)?;
    dealer_hand.add_card(deck[next_card as usize]);
    dealer_hand.add_card(Card::new(0, 0));
    dealer_hand.calculate_value();
    table.dealer_hand = Some(dealer_hand);
    table.used_card_indexes = used_card_indexes;

    

    // Handle blackjacks
    let mut all_done = true;
    for seat in table.seats.iter_mut().filter(|seat| seat.hand.is_some() && seat.bet > 0) {
        if let Some(hand) = seat.hand.as_ref() {
            if hand.is_blackjack {
                seat.in_action = false;
            } else {
                all_done = false;
            }
        }
    }

    if all_done {
        dealer_play(table, client_entropy, &ctx.accounts.recent_slothashes, table_key)?;
        resolve_game(table, &ctx.remaining_accounts, &ctx.accounts.authority, &mut ctx.accounts.program_state)?;
    } else {
        table.advance_turn().ok_or(NoActivePlayers)?;
        table.game_status = 2;
    }
    msg!("SOLWFR::BLACKJACK::DEAL_CARDS::SUCCESS:{},", table.key());

    Ok(())
}

#[derive(Accounts)]
pub struct DealCardsCtx<'info> {
    #[account(mut)]
    pub program_state: Account<'info, ProgramState>,
    #[account(mut)]
    pub table: Account<'info, Table>,
    pub system_program: Program<'info, System>, // Keep for consistency, though unused now
    pub authority: Signer<'info>,
    // remaining_accounts will contain the player_wallets
    #[account(address = anchor_lang::solana_program::sysvar::slot_hashes::ID)]
    /// CHECK: slot_hashes is needed
    pub recent_slothashes: AccountInfo<'info>, 
}

pub fn hit(ctx: Context<GameActionCtx>, seat_index: u8, client_entropy: [u8; 32]) -> Result<()> {
    let table = &mut ctx.accounts.table;
    let player_wallet = &ctx.accounts.player_wallet;

    if ctx.accounts.program_state.site_authority != ctx.accounts.authority.key() {
        return Err(Unauthorized.into());
    }

    table.validate_player_action(
        seat_index, 
        &ctx.accounts.player_wallet.key(),
        table.game_status
    )?;

    let table_key = table.key();
    table.check_and_reshuffle(
        ShuffleAction::Hit,
        &ctx.accounts.recent_slothashes,
        client_entropy,
        table_key
    )?;
    let deck = table.deck.clone();
    let mut used_card_indexes = table.used_card_indexes.clone();
    

    let seat = &mut table.seats[seat_index as usize];
  
    let next_card_index = get_random_unused_card(&ctx.accounts.recent_slothashes,  table_key, &mut used_card_indexes, client_entropy)?;
    let next_card = deck[next_card_index as usize];
    
    let hand = seat.hand.as_mut().ok_or(NoHandFound)?;
    hand.add_card(next_card);
    hand.calculate_value();
    let mut need_dealer_play = false;
   
    if hand.is_bust {
        seat.in_action = false;
        if table.advance_turn().is_none() {
            need_dealer_play = true;
        }
    }
    
    // Update card indexes OUTSIDE any mutable borrows
    table.used_card_indexes = used_card_indexes;
    
    if need_dealer_play {
        dealer_play(table, client_entropy, &ctx.accounts.recent_slothashes, table_key)?;
        resolve_game(table, &ctx.remaining_accounts, &ctx.accounts.authority, &mut ctx.accounts.program_state)?;
    }
    
    msg!("SOLWFR::BLACKJACK::HIT::SUCCESS:{}, seat_index:{}, player:{},", table.key(), seat_index, player_wallet.key());

    Ok(())
}

pub fn stand(ctx: Context<GameActionCtx>, seat_index: u8, client_entropy: [u8; 32]) -> Result<()> {
    let table = &mut ctx.accounts.table;
    let player_wallet = &ctx.accounts.player_wallet;
    let table_key = table.key();
    if ctx.accounts.program_state.site_authority != ctx.accounts.authority.key() {
        return Err(Unauthorized.into());
    }

    table.validate_player_action(
        seat_index, 
        &ctx.accounts.player_wallet.key(),
        table.game_status
    )?;
   
    let seat = &mut table.seats[seat_index as usize];

    seat.in_action = false;
    if table.advance_turn().is_none() {
        dealer_play(table, client_entropy, &ctx.accounts.recent_slothashes, table_key)?;
        resolve_game(table, &ctx.remaining_accounts, &ctx.accounts.authority, &mut ctx.accounts.program_state)?;
    }
    msg!("SOLWFR::BLACKJACK::STAND::SUCCESS:{}, seat_index:{}, player:{},", table.key(), seat_index, player_wallet.key());
    Ok(())
}

pub fn double_down(ctx: Context<GameActionCtx>, seat_index: u8, client_entropy: [u8; 32]) -> Result<()> {
    let table = &mut ctx.accounts.table;
    let player_wallet = &ctx.accounts.player_wallet;

    if ctx.accounts.program_state.site_authority != ctx.accounts.authority.key() {
        return Err(Unauthorized.into());
    }

    table.validate_player_action(
        seat_index, 
        &ctx.accounts.player_wallet.key(),
        table.game_status
    )?;

    if !table.double_allowed {
        return Err(DoubleDownNotAllowed.into());
    }

    let table_key = table.key();
    table.check_and_reshuffle(
        ShuffleAction::Double,
        &ctx.accounts.recent_slothashes,
        client_entropy,
        table_key
    )?;

    let deck = table.deck.clone();
    let mut used_card_indexes = table.used_card_indexes.clone();
   
    let seat_ref =  &table.seats[seat_index as usize];

    if seat_ref.hand.as_ref().unwrap().cards.len() != 2 {
        return Err(InvalidDoubleDown.into());
    }

    let additional_bet = seat_ref.bet;
    let total_bet = additional_bet.checked_mul(2).unwrap();

    let player_lamports = **player_wallet.try_borrow_mut_lamports()?;
    if player_lamports < additional_bet {
        return Err(InsufficientFunds.into());
    }

    **player_wallet.try_borrow_mut_lamports()? -= additional_bet;
    **table.to_account_info().try_borrow_mut_lamports()? += additional_bet;

    let seat = &mut table.seats[seat_index as usize];
    seat.bet = total_bet;
    let next_card_index = get_random_unused_card(&ctx.accounts.recent_slothashes,  table_key, &mut used_card_indexes, client_entropy)?;
    let next_card = deck[next_card_index as usize];
   

    let hand = seat.hand.as_mut().unwrap();
    hand.add_card(next_card);
    hand.calculate_value();
    
    seat.in_action = false;
    table.used_card_indexes = used_card_indexes;
    if table.advance_turn().is_none() {
        dealer_play(table, client_entropy, &ctx.accounts.recent_slothashes, table_key)?;
        resolve_game(table, &ctx.remaining_accounts, &ctx.accounts.authority, &mut ctx.accounts.program_state)?;
    }
    msg!("SOLWFR::BLACKJACK::DOUBLE_DOWN::SUCCESS:{}, seat_index:{}, player:{},", table.key(), seat_index, player_wallet.key());
    Ok(())
}

#[derive(Accounts)]
pub struct GameActionCtx<'info> {
    #[account(mut)]
    pub program_state: Account<'info, ProgramState>,
    #[account(mut)]
    pub table: Account<'info, Table>,
    #[account(mut)]
    /// CHECK: player_wallet is needed
    pub player_wallet: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
    pub authority: Signer<'info>,
    // remaining_accounts will contain the player_wallets
    #[account(address = anchor_lang::solana_program::sysvar::slot_hashes::ID)]
    /// CHECK: recent_slothashes is needed
    pub recent_slothashes: AccountInfo<'info>, 
}



fn dealer_play(table: &mut Table, client_entropy: [u8; 32],recent_slothashes: &AccountInfo,table_key: Pubkey,
    ) -> Result<()> {
    table.check_and_reshuffle(
        ShuffleAction::DealerPlay,
        &recent_slothashes,
        client_entropy,
        table_key
    )?;
    let dealer_hand = table.dealer_hand.as_mut().ok_or(NoHandFound)?;
    let mut used_card_indexes = table.used_card_indexes.clone();
    let deck = table.deck.clone();
    if let Some(pos) = dealer_hand.cards.iter().position(|card| card.rank == 0) {
        let next_card_index = get_random_unused_card(&recent_slothashes,  table_key, &mut used_card_indexes, client_entropy)?;
        let next_card = deck[next_card_index as usize];
        dealer_hand.cards[pos] = next_card;
    }

    dealer_hand.calculate_value();
    while dealer_hand.value < 17 {
        let next_card_index = get_random_unused_card(&recent_slothashes,  table_key, &mut used_card_indexes, client_entropy)?;
        let next_card = deck[next_card_index as usize];
        dealer_hand.add_card(next_card);
        dealer_hand.calculate_value();
    }
    table.used_card_indexes = used_card_indexes;
    Ok(())
}


fn determine_winner(dealer: &Hand, player: &Hand) -> GameResult {
    if player.is_bust {
        return GameResult::Lost;
    }
    if dealer.is_bust {
        return GameResult::Won;
    }
    if player.is_blackjack {
        return GameResult::Won;
    }
    if dealer.is_blackjack && !player.is_blackjack {
        return GameResult::Lost;
    }
    if player.value == dealer.value {
        return GameResult::Push;
    }
    if player.value > dealer.value {
        GameResult::Won
    } else {
        GameResult::Lost
    }
}

fn calculate_payout(bet: u64, result: GameResult, is_blackjack: bool) -> u64 {
    match result {
        GameResult::Won => {
            if is_blackjack {
                // For blackjack: return original bet + 1.5x bet (3:2 payout)
                bet.checked_add(bet.checked_mul(3).unwrap().checked_div(2).unwrap()).unwrap()
            } else {
                bet.checked_mul(2).unwrap() // 1:1 payout (original bet + 1x bet)
            }
        }
        GameResult::Push => bet,
        GameResult::Lost => 0,
    }
}

fn resolve_game(table: &mut Account<Table>, remaining_accounts: &[AccountInfo], authority_wallet: &AccountInfo, program_state: &mut Account<ProgramState>) -> Result<()> {
    // Validate remaining_accounts length
    let active_players = table
        .seats
        .iter()
        .filter(|seat| seat.player.is_some() && seat.bet > 0)
        .count();
    if remaining_accounts.len() < active_players {
        return Err(InvalidRemainingAccounts.into());
    }
    

    // Clone dealer_hand early to avoid borrow issues
    let dealer_hand = table
        .dealer_hand
        .as_ref()
        .ok_or(NoHandFound)?
        .clone();

    let mut account_index = 0;
    let mut dealer_liquidity_change: i64 = 0; // Track total payout to dealer

    let mut total_dealer_payouts: u64 = 0; // Track total payout to dealer
    let mut total_prize_pool_payouts: u64 = 0; // Track total payout to prize pool
    let mut total_bets: u64 = 0; // Track total bets
    let table_fee = table.table_fee;
    let mut authority_fee_to_be_paid: u64 = 0;
    // Clear previous hands
    for seat in table.seats.iter_mut() {
        if seat.previous_hand.is_some() {
            seat.previous_hand = None;
        }
    }

    // Process each player's hand
    for seat in table.seats.iter_mut().filter(|seat| seat.player.is_some() && seat.bet > 0) {
        let player_wallet = &remaining_accounts[account_index];


        if seat.player_wallet != Some(player_wallet.key()) {
            return Err(Unauthorized.into());
        }

        let player_hand = seat.hand.as_ref().ok_or(NoHandFound)?.clone();
        let bet_amount = seat.bet;

        total_bets += bet_amount;
        //total_bets = total_bets.checked_add(bet_amount).ok_or(MathOverflow)?;

        let game_result = determine_winner(&dealer_hand, &player_hand);
        let payout_amount = calculate_payout(bet_amount, game_result, player_hand.is_blackjack);

        // Update player wallet and track total payout
        if game_result == GameResult::Won {
            let winning_amount = payout_amount - bet_amount;

            let table_fee_amount = (winning_amount * table_fee as u64) / 100;
            let authority_fee = std::cmp::min(
                (winning_amount * table_fee as u64) / 200, // table_fee_amount / 2 as a percentage
                winning_amount / 100, // 1% cap
            );
            authority_fee_to_be_paid += authority_fee;
            let dealer_fee = table_fee_amount - authority_fee;

            dealer_liquidity_change -= (winning_amount - dealer_fee) as i64;
            total_prize_pool_payouts += bet_amount;
            total_dealer_payouts += winning_amount - dealer_fee;
            **player_wallet.try_borrow_mut_lamports()? += payout_amount - table_fee_amount;
        } else if game_result == GameResult::Push {
            total_prize_pool_payouts += bet_amount;
            **player_wallet.try_borrow_mut_lamports()? += bet_amount;
        } else if game_result == GameResult::Lost {
            dealer_liquidity_change += bet_amount as i64;
        }

        // Record the result for logging/debugging
        seat.previous_hand = Some(PlayerHandResult {
            player_hand: player_hand.clone(),
            hand_result: game_result,
            bet_amount,
            payout_amount,
        });
        seat.last_hand_played_timestamp = Clock::get()?.unix_timestamp;
        seat.clear();
        account_index += 1;
    }

    // Skip state updates if there are no active players
    if account_index == 0 {
        table.last_hand_timestamp = Clock::get()?.unix_timestamp;
        table.previous_dealer_hand = Some(dealer_hand);
        table.game_status = 1;
        table.dealer_hand = None;
        table.current_turn_index = None;
        return Ok(());
    }

    if total_prize_pool_payouts > total_bets {
        return Err(InvalidPrizePoolPayout.into());
    }

    let total_payout_to_players = total_prize_pool_payouts + total_dealer_payouts;
    let rent = Rent::get()?.minimum_balance(table.to_account_info().data_len());

    // Update table's lamports after the loop
    if total_payout_to_players > 0 {
        **authority_wallet.try_borrow_mut_lamports()? += authority_fee_to_be_paid;
        program_state.fees_collected = program_state.fees_collected.checked_add(authority_fee_to_be_paid).ok_or(MathOverflow)?;
        if table.dealer_liquidity < total_dealer_payouts {
            return Err(InsufficientFunds.into());
        }
        let table_lamports = **table.to_account_info().try_borrow_mut_lamports()?;
        if table_lamports < total_payout_to_players {
            return Err(InsufficientFunds.into());
        }
        **table.to_account_info().try_borrow_mut_lamports()? -= total_payout_to_players;
    }

    // Update dealer liquidity
    if dealer_liquidity_change < 0 {
        let deduction = (-dealer_liquidity_change) as u64;
        if table.dealer_liquidity < deduction {
            return Err(InsufficientFunds.into());
        }
        table.dealer_liquidity = table.dealer_liquidity.checked_sub(deduction).ok_or(MathOverflow)?;
    }else if dealer_liquidity_change > 0 {
        table.dealer_liquidity = table.dealer_liquidity.checked_add(dealer_liquidity_change as u64).ok_or(MathOverflow)?;
    }
    let table_balance = **table.to_account_info().try_borrow_mut_lamports()? - rent;
    if table_balance != table.dealer_liquidity {
        return Err(InsufficientFunds.into());
    }
 
    // Adjust table settings based on liquidity
    let required_liquidity = table
    .max_bet
    .checked_mul(table.max_players as u64)
    .ok_or(MathOverflow)?
    .checked_mul(2)
    .ok_or(MathOverflow)?;
   if table.dealer_liquidity < required_liquidity {
    // First calculate raw max_bet
    let raw_max_bet = table.dealer_liquidity / (table.max_players as u64 * 2);
    
    // Round down to 4 decimal places (100 lamports)
    // 100 lamports = 0.0001 SOL = 1e5 lamports
    let decimal_places = 1_000_000; // 1e6 lamports
    table.max_bet = (raw_max_bet / decimal_places) * decimal_places;
    
    // Do the same for min_bet
    let raw_min_bet = table.max_bet / 10;
    table.min_bet = (raw_min_bet / decimal_places) * decimal_places;
    
    table.active = table.dealer_liquidity > 0;
}

    // Update table state
    table.last_hand_timestamp = Clock::get()?.unix_timestamp;
    table.show_hand_timestamp = Clock::get()?.unix_timestamp;
    table.previous_dealer_hand = Some(dealer_hand);
    table.game_status = 1;
    table.dealer_hand = None;
    table.current_turn_index = None;
    msg!("SOLWFR::BLACKJACK::RESOLVE_GAME::SUCCESS:{}, dealer_liquidity:{},  authority_fee:{}, total_prize_pool_payouts:{}, total_dealer_payouts:{}, total_bets:{}, total_payout_to_players:{},", table.key(), table.dealer_liquidity, authority_fee_to_be_paid, total_prize_pool_payouts, total_dealer_payouts, total_bets, total_payout_to_players);

    Ok(())
}


fn get_random_unused_card(
    recent_slothashes: &AccountInfo,
    table_key: Pubkey,
    used_card_indexes: &mut Vec<bool>,
    client_entropy: [u8; 32],
) -> Result<usize> {
    // Count unused cards
    let unused_count = used_card_indexes.iter().filter(|&&used| !used).count();
    if unused_count == 0 {
        return Err(OutOfCards.into());
    }
    
    // Generate a random index among the unused cards
    let random_position = generate_secure_random(
        &recent_slothashes,
        table_key,
        client_entropy,
        0,
        unused_count as u64 - 1
    )? as usize;
    
    // Find the card at the random position among unused cards
    let mut found_unused = 0;
    for (i, &used) in used_card_indexes.iter().enumerate() {
        if !used {
            if found_unused == random_position {
                used_card_indexes[i] = true;
                return Ok(i);
            }
            found_unused += 1;
        }
    }
    
    // This should never happen, but just in case
    Err(OutOfCards.into())
}

pub fn generate_secure_random(
    recent_slothashes: &AccountInfo,
    table_key: Pubkey,
    client_entropy: [u8; 32],
    min: u64,
    max: u64,
) -> Result<u64> {
    // Get on-chain entropy sources
    let clock = Clock::get()?;
    let slothash_data = recent_slothashes.data.borrow();

    let slot_hash = if slothash_data.len() >= 20 {
        u64::from_le_bytes(slothash_data[12..20].try_into().unwrap())
    } else {
        clock.slot // Fallback if slot hash not available
    };
     
    // Create hash with multiple inputs
    let mut hasher = Sha256::new();
    
    // On-chain sources
    hasher.update(&table_key.to_bytes());
    hasher.update(&clock.slot.to_le_bytes());
    hasher.update(&clock.unix_timestamp.to_le_bytes());
    hasher.update(&slot_hash.to_le_bytes());
    
    // Client inputs
    hasher.update(&client_entropy);
    
    // Get result as bytes
    let result = hasher.finalize();
    
    // Convert first 8 bytes to u64
    let random_u64 = u64::from_le_bytes(result[0..8].try_into().unwrap());
    
    // Scale to range
    let range = max.saturating_sub(min).saturating_add(1);
    let scaled_random = min.saturating_add(random_u64 % range);
    
    Ok(scaled_random)
}
