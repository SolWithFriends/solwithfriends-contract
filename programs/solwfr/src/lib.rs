#![allow(unexpected_cfgs)]
use anchor_lang::prelude::*;

pub mod instructions;
pub mod states; 
pub mod constants;
pub mod errors;

use instructions::*;
#[allow(unused_imports)]
use states::*;

//anchor upgrade target/deploy/basic.so --program-id 66WaYihanAvpcmnN9zYLSi6g8eipjBb8PfFeNUEzUHPS

// Program ID declaration (replace with your own ID when deploying)
declare_id!("GyjVtacGa7irP3s4EMKt49FU769e5SRrW2kDVnd7AeSp");

#[program]
pub mod solwfr {
    use super::*;

    pub fn initialize(ctx: Context<InitializeCtx>) -> Result<()> {
        instructions::initialize(ctx)
    }

    //lottery
    pub fn create_lottery(ctx: Context<CreateLotteryCtx>, ticket_price: u64, title: String, time_limit: u64, max_tickets: u64, is_automatic: bool) -> Result<()> {
        instructions::create_lottery(ctx, ticket_price, title, time_limit, max_tickets, is_automatic)
    }

    pub fn buy_ticket(ctx: Context<BuyTicketCtx>, ticket_amount: u64) -> Result<()> {
        instructions::buy_ticket(ctx, ticket_amount)
    }

    pub fn pick_winner(ctx: Context<PickWinnerCtx>, lottery_id:u64) -> Result<()> {
        instructions::pick_winner(ctx, lottery_id)
    }

    pub fn claim_prize(ctx: Context<ClaimPrizeCtx>) -> Result<()> {
        instructions::claim_prize(ctx,)
    }
    // Blackjack
    // Blackjack Dealer
    pub fn create_table(ctx: Context<CreateTableCtx>, title: String, max_bet: u64, min_bet: u64, split_allowed: bool, double_allowed: bool, dealer_liquidity: u64, max_players: u8, time_between_hands: i64, time_between_actions: i64, inactivity_timeout: i64, table_fee: u8, huddle_room_id: String, num_decks: u8, deck_penetration: u8, ) -> Result<()> {
        instructions::create_table(ctx, title, max_bet, min_bet, split_allowed, double_allowed, dealer_liquidity, max_players, time_between_hands, time_between_actions, inactivity_timeout, table_fee, huddle_room_id, num_decks, deck_penetration)
    }

    pub fn close_table(ctx: Context<CloseTableCtx>, tid: u64) -> Result<()> {
        instructions::close_table(ctx, tid)
    }

    pub fn update_table(ctx: Context<TableActionCtx>, title: String, max_bet: u64, min_bet: u64, max_players: u8) -> Result<()> {
        instructions::update_table(ctx, title, max_bet, min_bet, max_players)
    }

    pub fn dealer_deposit(ctx: Context<TableActionCtx>, amount: u64) -> Result<()> {
        instructions::dealer_deposit(ctx, amount)
    }

    pub fn dealer_withdraw(ctx: Context<TableActionCtx>, amount: u64) -> Result<()> {
        instructions::dealer_withdraw(ctx, amount)
    }

    // Blackjack Player
    pub fn join_seat(ctx: Context<PlayerWalletCtx>, seat_index: u8) -> Result<()> {
        instructions::join_seat(ctx, seat_index)
    }

    pub fn leave_seat(ctx: Context<PlayerWalletCtx>, seat_index: u8) -> Result<()> {
        instructions::leave_seat(ctx, seat_index)
    }

    pub fn place_bet(ctx: Context<PlayerWalletCtx>, seat_index: u8, bet_amount: u64) -> Result<()> {
        instructions::place_bet(ctx, seat_index, bet_amount)
    }

    pub fn remove_bet(ctx: Context<PlayerWalletCtx>, seat_index: u8) -> Result<()> {
        instructions::remove_bet(ctx, seat_index)
    }

    // Blackjack Game Actions

    pub fn deal_cards(ctx: Context<DealCardsCtx>, client_entropy: [u8; 32]) -> Result<()> {
        instructions::deal_cards(ctx, client_entropy)
    }

    pub fn hit(ctx: Context<GameActionCtx>, seat_index: u8, client_entropy: [u8; 32]) -> Result<()> {
        instructions::hit(ctx, seat_index, client_entropy)
    }

    pub fn stand(ctx: Context<GameActionCtx>, seat_index: u8, client_entropy: [u8; 32]) -> Result<()> {
        instructions::stand(ctx, seat_index, client_entropy)
    }

    pub fn double_down(ctx: Context<GameActionCtx>, seat_index: u8, client_entropy: [u8; 32]) -> Result<()> {
        instructions::double_down(ctx, seat_index, client_entropy)
    }

    
    // Player Wallet
    pub fn initialize_player_wallet(ctx: Context<InitializePlayerWallet>) -> Result<()> {
        instructions::initialize_player_wallet(ctx)
    }

    pub fn deposit(ctx: Context<Deposit>, amount: u64) -> Result<()> {
        instructions::deposit(ctx, amount)
    }

    pub fn withdraw(ctx: Context<Withdraw>, amount: u64) -> Result<()> {
        instructions::withdraw(ctx, amount)
    }

    pub fn withdraw_fees(ctx: Context<WithdrawFeesCtx>) -> Result<()> {
        instructions::withdraw_fees(ctx)
    }

    pub fn emergency_close_table(ctx: Context<EmergencyCloseTableCtx>) -> Result<()> {
        instructions::emergency_close_table(ctx)
    }

    pub fn emergency_close_wallet(ctx: Context<EmergencyCloseWalletCtx>) -> Result<()> {
        instructions::emergency_close_wallet(ctx)
    }


    
}

