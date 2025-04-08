use anchor_lang::prelude::*;
use crate::states::{Table, ProgramState, PlayerWallet};
use crate::errors::ErrorCode::*;

// Join Table: Bot joins player to table
pub fn join_seat(ctx: Context<PlayerWalletCtx>, seat_index: u8) -> Result<()> {
    let table = &mut ctx.accounts.table;
    let player_wallet = &ctx.accounts.player_wallet;
    let program_state = &mut ctx.accounts.program_state;
    let authority = &ctx.accounts.authority;
    if authority.key() != program_state.site_authority {
        return Err(InvalidAuthority.into());
    }

    if table.game_status == 2 {
        return Err(InvalidGameState.into());
    }

    if table.get_active_seat_count() >= table.max_players as usize {
        return Err(TableFull.into());
    }

    if seat_index >= table.seats.len() as u8 {
        return Err(InvalidSeatIndex.into());
    }

    let seat = &mut table.seats[seat_index as usize];
    if seat.player.is_some() {
        return Err(SeatTaken.into());
    }

    let current_time = Clock::get()?.unix_timestamp;
    seat.player = Some(player_wallet.player);
    seat.player_wallet = Some(player_wallet.key());
    seat.last_hand_played_timestamp = current_time;

    if table.get_active_seat_count() == 1 {
        table.game_status = 1;
    }
    if table.has_players == false {
        table.has_players = true;
    }
    msg!("SOLWFR::BLACKJACK::JOIN_SEAT::SUCCESS:{}, player:{}, seat_index:{},", table.key(), player_wallet.key(), seat_index);

    Ok(())
}

// Leave Table: Bot removes player from table
pub fn leave_seat(ctx: Context<PlayerWalletCtx>, seat_index: u8) -> Result<()> {
    let table = &mut ctx.accounts.table;
    let player_wallet = &ctx.accounts.player_wallet;
    let program_state = &mut ctx.accounts.program_state;
    let authority = &ctx.accounts.authority;
    if authority.key() != program_state.site_authority {
        return Err(InvalidAuthority.into());
    }

    // Check game state first
    if table.game_status == 2 {
        return Err(InvalidGameState.into());
    }

    if table.seats[seat_index as usize].player.is_none() {
        return Err(PlayerNotJoined.into());
    }

    if table.seats[seat_index as usize].player.unwrap() != player_wallet.player {
        return Err(Unauthorized.into());
    }
    
    // Check bet before clearing
    if table.seats[seat_index as usize].bet > 0 {
        return Err(ActiveBetExists.into());
    }

    // Clear the seat
    table.seats[seat_index as usize].full_clear();

    // Update game state if needed
    if table.get_active_seat_count() == 0 {
        table.game_status = 0;
        table.has_players = false;
    }

    msg!("SOLWFR::BLACKJACK::LEAVE_SEAT::SUCCESS:{},  player:{}, seat_index:{},", table.key(), player_wallet.key(), seat_index);

    Ok(())
}

// Place Bet: Bot transfers bet from PlayerWallet to table
pub fn place_bet(ctx: Context<PlayerWalletCtx>, seat_index: u8, bet_amount: u64) -> Result<()> {
    let table = &mut ctx.accounts.table;
    let player_wallet = &mut ctx.accounts.player_wallet;
    let program_state = &mut ctx.accounts.program_state;
    let authority = &ctx.accounts.authority;
    if authority.key() != program_state.site_authority {
        return Err(InvalidAuthority.into());
    }

    if bet_amount < table.min_bet || bet_amount > table.max_bet {
        return Err(InvalidBetAmount.into());
    }

    if table.game_status != 1 {
        return Err(InvalidGameState.into());
    }

    if table.seats[seat_index as usize].player.is_none() {
        return Err(PlayerNotJoined.into());
    }

    if table.seats[seat_index as usize].player.unwrap() != player_wallet.player {
        return Err(Unauthorized.into());
    }

    let player_wallet_lamports = **player_wallet.to_account_info().try_borrow_mut_lamports()?;
    if player_wallet_lamports < bet_amount {
        return Err(InsufficientFunds.into());
    }

    

    let current_time = Clock::get()?.unix_timestamp;
    if table.last_hand_timestamp == 0 {
        table.last_hand_timestamp = current_time;
    } else {
        let time_since_last_hand = current_time - table.last_hand_timestamp;
        if time_since_last_hand > table.time_between_hands {
            // Check if there are other bets on the table (excluding the current seat)
            let other_seats_have_bets = table.seats.iter().enumerate().any(|(i, seat)| {
                i != seat_index as usize && seat.bet > 0
            });

            // Reset lastHandTimestamp only if there are no other bets
            if !other_seats_have_bets {
                table.last_hand_timestamp = current_time;
            }
        }
    }

    // Handle lamport transfers first
    **player_wallet.to_account_info().try_borrow_mut_lamports()? -= bet_amount;
    **table.to_account_info().try_borrow_mut_lamports()? += bet_amount;

    table.seats[seat_index as usize].last_hand_played_timestamp = current_time;
    // Update bet amount
    let seat = &mut table.seats[seat_index as usize];
    if seat.bet > 0 {
        seat.bet = seat.bet.checked_add(bet_amount).unwrap();
        player_wallet.balance = player_wallet.balance.checked_sub(bet_amount).unwrap();
    } else {
        seat.bet = bet_amount;
    }

    msg!("SOLWFR::BLACKJACK::PLACE_BET::SUCCESS:{}, player:{}, seat_index:{}, bet_amount:{},", table.key(), player_wallet.key(), seat_index, bet_amount);

    Ok(())
}

// Remove Bet: Bot transfers bet back from table to PlayerWallet
pub fn remove_bet(ctx: Context<PlayerWalletCtx>, seat_index: u8) -> Result<()> {
    let table = &mut ctx.accounts.table;
    let player_wallet = &mut ctx.accounts.player_wallet;
    let program_state = &mut ctx.accounts.program_state;
    let authority = &ctx.accounts.authority;
    if authority.key() != program_state.site_authority {
        return Err(InvalidAuthority.into());
    }

    if player_wallet.authority != ctx.accounts.authority.key() {
        return Err(Unauthorized.into());
    }

    if table.game_status != 1 {
        return Err(InvalidGameState.into());
    }

    if table.seats[seat_index as usize].player.is_none() {
        return Err(PlayerNotJoined.into());
    }

    if table.seats[seat_index as usize].player.unwrap() != player_wallet.player {
        return Err(Unauthorized.into());
    }
    let bet_amount = table.seats[seat_index as usize].bet;
    if bet_amount == 0 {
        return Err(NoBet.into());
    }

    let table_lamports = **table.to_account_info().try_borrow_mut_lamports()?;
    if table_lamports < bet_amount {
        return Err(InsufficientFunds.into());
    }

    // Handle lamport transfers
    **player_wallet.to_account_info().try_borrow_mut_lamports()? += bet_amount;
    **table.to_account_info().try_borrow_mut_lamports()? -= bet_amount;

    // Update balances
    player_wallet.balance = player_wallet.balance.checked_add(bet_amount).unwrap();
    table.seats[seat_index as usize].bet = 0;

    msg!("SOLWFR::BLACKJACK::REMOVE_BET::SUCCESS:{}, player:{}, seat_index:{}, bet_amount:{},", table.key(), player_wallet.key(), seat_index, bet_amount);

    Ok(())
}

#[derive(Accounts)]
pub struct PlayerWalletCtx<'info> {
    #[account(mut)]
    pub table: Account<'info, Table>,
    #[account(mut, has_one = authority)]
    pub player_wallet: Account<'info, PlayerWallet>,
    pub authority: Signer<'info>, // Site authority bot
    pub program_state: Account<'info, ProgramState>,
}





