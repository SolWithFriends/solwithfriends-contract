use crate::constants::ANCHOR_DISCRIMINATOR_SIZE;
use crate::errors::ErrorCode::*;
use crate::states::{Table, ProgramState, Seat};
use anchor_lang::prelude::*;

pub fn create_table(
    ctx: Context<CreateTableCtx>,
    title: String,
    max_bet: u64,
    min_bet: u64,
    split_allowed: bool,
    double_allowed: bool,
    dealer_liquidity: u64,
    max_players: u8,
    time_between_hands: i64,
    time_between_actions: i64,
    inactivity_timeout: i64,
    table_fee: u8,
    huddle_room_id: String,
    num_decks: u8,
    deck_penetration: u8,
) -> Result<()> {
    let table = &mut ctx.accounts.table;
    let state = &mut ctx.accounts.program_state;
    let dealer = &mut ctx.accounts.dealer;
    if title.len() > 64 {
        return Err(TitleTooLong.into());
    }

    let required_liquidity = max_bet * max_players as u64 * 2;

    if dealer_liquidity < required_liquidity {
        return Err(InsufficientLiquidity.into());
    }

    if dealer_liquidity < min_bet * max_players as u64 * 2 {
        return Err(InsufficientLiquidity.into());
    }

    if dealer_liquidity > 0 {
        let tx_instruction = anchor_lang::solana_program::system_instruction::transfer(
            &dealer.key(),
            &table.key(),
            dealer_liquidity,
        );
    
        let result = anchor_lang::solana_program::program::invoke(
            &tx_instruction,
            &[dealer.to_account_info(), table.to_account_info()],
        );
        if let Err(e) = result {
            msg!("SOLWFR::BLACKJACK::CREATE_TABLE::DEALER_DEPOSIT::FAILED");
            return Err(e.into());
        }
        table.dealer_liquidity = dealer_liquidity;
        table.active = true;
    }

    if num_decks > 6 {
        return Err(InvalidNumDecks.into());
    }

    if deck_penetration > 100 {
        return Err(InvalidDeckPenetration.into());
    }
    
    state.table_count += 1;
    let last_hand_played_timestamp = Clock::get()?.unix_timestamp as i64;

    // Initialize seats vector with empty seats
    table.seats = (0..max_players).map(|_| Seat {
        player: None,
        player_wallet: None,
        bet: 0,
        in_action: false,
        hand: None,
        is_turn: false,
        previous_hand: None,
        last_action_timestamp: 0,
        last_hand_played_timestamp: last_hand_played_timestamp,
    }).collect();

    table.max_bet = max_bet;
    table.min_bet = min_bet;
    table.split_allowed = split_allowed;        
    table.double_allowed = double_allowed;
    table.game_status = 0;
    table.id = state.table_count;
    table.dealer = ctx.accounts.dealer.key();
    table.title = title;
    table.timestamp = Clock::get()?.unix_timestamp as u64;
    table.max_players = max_players;
    table.last_hand_timestamp = 0;
    table.time_between_hands = time_between_hands;
    table.time_between_actions = time_between_actions;
    table.inactive_player_timeout = inactivity_timeout;
    table.table_fee = table_fee;
    table.huddle_room_id = huddle_room_id;
    table.num_decks = num_decks;
    table.deck_penetration = deck_penetration;
    msg!("SOLWFR::BLACKJACK::CREATE_TABLE::SUCCESS:{}, dealer:{}, liquidity:{}, table_fee:{}", table.key(), table.dealer, table.dealer_liquidity, table.table_fee );


    Ok(())
}

pub fn close_table(ctx: Context<CloseTableCtx>, tid: u64) -> Result<()> {
    let table = &mut ctx.accounts.table;
    let dealer = &mut ctx.accounts.dealer;


    if tid != table.id {
        return Err(TableNotFound.into());
    }

    if dealer.key() != table.dealer {
        return Err(InvalidDealer.into());
    }

    // Check if there's an active bet
    if table.seats.iter().any(|seat| seat.bet > 0) {    
        msg!("Cannot close table with active bets");
        return Err(ActiveBetExists.into());
    }

    if table.game_status == 2 {
        msg!("Cannot close table with game in progress");
        return Err(GameInProgress.into());
    }

   
    msg!("SOLWFR::BLACKJACK::CLOSE_TABLE::SUCCESS:{}, dealer:{},", table.key(), dealer.key());

    Ok(())
}

pub fn update_table(
    ctx: Context<TableActionCtx>,
    title: String,
    max_bet: u64,
    min_bet: u64,
    max_players: u8,
) -> Result<()> {
    let table = &mut ctx.accounts.table;
    let dealer = &mut ctx.accounts.dealer;

    if table.game_status == 2 {
        return Err(GameInProgress.into());
    }

    if table.dealer != dealer.key() {
        return Err(InvalidDealer.into());
    }

    if title.len() > 64 {
        return Err(TitleTooLong.into());
    }

    if max_bet < min_bet {
        return Err(InvalidBetAmount.into());
    }

    if table.dealer_liquidity < max_bet * max_players as u64 * 2 {
        return Err(InsufficientLiquidity.into());
    }

    if table.dealer_liquidity < min_bet * max_players as u64 * 2 {
        return Err(InsufficientLiquidity.into());
    }

    let last_hand_played_timestamp = Clock::get()?.unix_timestamp as i64;

     // Handle seat changes
     if max_players > table.max_players {
        // Adding new seats
        let additional_seats = (table.max_players..max_players).map(|_| Seat {
            player: None,
            player_wallet: None,
            bet: 0,
            in_action: false,
            hand: None,
            is_turn: false,
            previous_hand: None,
            last_action_timestamp: 0,
            last_hand_played_timestamp,
        });
        table.seats.extend(additional_seats);
    } else if max_players < table.max_players {
        // Count active players and find empty seats
        let mut active_players = 0;
        let mut empty_seats: Vec<usize> = Vec::new();
        
        for (i, seat) in table.seats.iter().enumerate().take(max_players as usize) {
            if seat.player.is_some() {
                active_players += 1;
            } else {
                empty_seats.push(i);
            }
        }

        // Check players that need to be moved (in seats that will be removed)
        let mut players_to_move: Vec<Seat> = Vec::new();
        for seat in table.seats.iter().skip(max_players as usize) {
            if seat.player.is_some() {
                active_players += 1;
                players_to_move.push(seat.clone());
            }
        }

        // Verify we have enough space
        if active_players > max_players {
            return Err(InsufficientSeats.into());
        }

        // Move players to empty seats
        for (player_idx, player) in players_to_move.iter().enumerate() {
            if let Some(empty_seat_idx) = empty_seats.get(player_idx) {
                table.seats[*empty_seat_idx] = player.clone();
            }
        }

        // Truncate the seats vector to the new size
        table.seats.truncate(max_players as usize);
    }

    table.max_bet = max_bet;
    table.min_bet = min_bet;
    table.title = title;
    table.max_players = max_players;
    Ok(())
}

pub fn dealer_deposit(ctx: Context<TableActionCtx>, amount: u64) -> Result<()> {
    let table = &mut ctx.accounts.table;
    let dealer = &mut ctx.accounts.dealer;

    if table.dealer != dealer.key() {
        return Err(InvalidDealer.into());
    }

    let tx_instruction = anchor_lang::solana_program::system_instruction::transfer(
        &dealer.key(),
        &table.key(),
        amount,
    );

    let result = anchor_lang::solana_program::program::invoke(
        &tx_instruction,
        &[dealer.to_account_info(), table.to_account_info()],
    );

    if let Err(e) = result {
        msg!("Dealer deposit transfer failed: {:?}", e);
        return Err(e.into());
    }

    table.dealer_liquidity += amount;
    table.active = table.dealer_liquidity > 0;

    let required_liquidity = table.max_bet * table.max_players as u64 * 2;

    if table.dealer_liquidity < required_liquidity {
        table.max_bet = table.dealer_liquidity / (table.max_players as u64 * 2);
        table.min_bet = table.max_bet / 10;
        table.active = table.dealer_liquidity > 0;
    }

    if table.max_bet == 0{
        table.max_bet = table.dealer_liquidity / 2;
    }

    if table.min_bet == 0{
        table.min_bet = table.max_bet / 10;
    }

    msg!("SOLWFR::BLACKJACK::DEALER_DEPOSIT::SUCCESS:{}, dealer:{}, liquidity:{}, amount:{}", table.key(), table.dealer, table.dealer_liquidity, amount );

    Ok(())
}

pub fn dealer_withdraw(ctx: Context<TableActionCtx>, amount: u64) -> Result<()> {
    let table = &mut ctx.accounts.table;
    let dealer = &ctx.accounts.dealer;

    if table.game_status == 2 {
        return Err(GameInProgress.into());
    }

    if table.dealer != dealer.key() {
        return Err(InvalidDealer.into());
    }


    if amount > table.dealer_liquidity {
        return Err(InsufficientFunds.into());
    }



    // Comparing amount against usable balance
    let rent_balance = Rent::get()?.minimum_balance(table.to_account_info().data_len());
    if amount > **table.to_account_info().lamports.borrow() - rent_balance {
        return Err(InsufficientFunds.into());
    }

    // Transferring money to dealer
    **table.to_account_info().try_borrow_mut_lamports()? -= amount;
    **dealer.to_account_info().try_borrow_mut_lamports()? += amount;


     // Calculate required liquidity for max bets from all players
     let required_liquidity = table.max_bet * table.max_players as u64 * 2;

     // If current liquidity is less than required, adjust limits
     if table.dealer_liquidity < required_liquidity {
         table.max_bet = table.dealer_liquidity / (table.max_players as u64 * 2);
         table.min_bet = table.max_bet / 10;
         table.active = table.dealer_liquidity > 0;
     }


    table.dealer_liquidity -= amount;
    table.active = table.dealer_liquidity > 0;




    msg!("SOLWFR::BLACKJACK::DEALER_WITHDRAW::SUCCESS:{}, dealer:{}, liquidity:{}, amount:{}", table.key(), table.dealer, table.dealer_liquidity, amount );

    Ok(())
}


#[derive(Accounts)]
pub struct TableActionCtx<'info> {
    #[account(mut)]
    pub table: Account<'info, Table>,
    #[account(mut)]
    pub dealer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct CreateTableCtx<'info> {
    #[account(mut)]
    pub program_state: Account<'info, ProgramState>,

    #[account(
        init,
        payer = dealer,
        space = ANCHOR_DISCRIMINATOR_SIZE + Table::INIT_SPACE,
        seeds = [
            b"table",
            (program_state.table_count + 1).to_le_bytes().as_ref()
        ],
        bump
    )]
    pub table: Account<'info, Table>,

    #[account(mut)]
    pub dealer: Signer<'info>,
    pub system_program: Program<'info, System>,
}



#[derive(Accounts)]
#[instruction(tid: u64)]
pub struct CloseTableCtx<'info> {
    #[account(mut)]
    pub program_state: Account<'info, ProgramState>,
    #[account(
        mut,
        seeds = [
            b"table",
            tid.to_le_bytes().as_ref()
        ],
        bump,
        close = dealer
    )]
    pub table: Account<'info, Table>,

    #[account(mut)]
    pub dealer: Signer<'info>,

    pub system_program: Program<'info, System>,
}

