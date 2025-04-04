use anchor_lang::prelude::*;
use crate::states::bj::{Hand, Card};

#[derive(AnchorSerialize, AnchorDeserialize, Clone, InitSpace)]
pub struct Seat {
    pub player: Option<Pubkey>,
    pub player_wallet: Option<Pubkey>,
    pub bet: u64,
    pub in_action: bool,
    pub hand: Option<Hand>,
    pub is_turn: bool,
    pub previous_hand: Option<PlayerHandResult>,
    pub last_action_timestamp: i64,
    pub last_hand_played_timestamp: i64,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug)]
#[derive(InitSpace)]
pub struct PlayerHandResult {
    pub player_hand: Hand,
    pub bet_amount: u64,
    pub payout_amount: u64,
    pub hand_result: GameResult,
} 
#[derive(Debug, Clone, Copy, PartialEq, AnchorSerialize, AnchorDeserialize)]
#[derive(InitSpace)]
pub enum GameResult {
    Won,
    Lost,
    Push,
}


impl Seat {
    pub fn clear(&mut self) {
        self.bet = 0;
        self.in_action = false;
        self.hand = None;
        self.is_turn = false;
        self.last_action_timestamp = 0;
    }

    pub fn full_clear(&mut self) {
        self.clear();
        self.previous_hand = None;
        self.player = None;
        self.player_wallet = None;
    }
}

#[account]
#[derive(InitSpace)]
pub struct Table {
    pub id: u64,
    pub dealer: Pubkey,
    pub dealer_liquidity: u64,
    pub min_bet: u64,
    pub max_bet: u64,
    pub total_hands: u64,
    #[max_len(32)]
    pub title: String,
    pub timestamp: u64,
    pub current_card_index: u64,
    #[max_len(104)]
    pub deck: Vec<Card>,
    pub num_decks: u8,
    pub deck_seed: u64,
    pub active: bool,
    pub table_status: u8, // 0: no liquidity, 1: active, 2: paused, 3: closed
    pub table_fee: u8, // 0: 0%, 1: 1%, 2: 2%, 3: 3%, 4: 4%, 5: 5%
    #[max_len(32)]
    pub huddle_room_id: String,
    pub has_players: bool,

    #[max_len(6)]
    pub seats: Vec<Seat>,
    pub current_turn_index: Option<u8>,
    pub dealer_hand: Option<Hand>,

    pub game_status: u8, // 0: waiting for players, 1: waiting for bets, 2: hand in progress 
    pub previous_dealer_hand: Option<Hand>,

    pub split_allowed: bool,
    pub double_allowed: bool,
    pub max_players: u8,
    pub last_hand_timestamp: i64,
    pub show_hand_timestamp: i64,
    pub time_between_hands: i64,
    pub time_between_actions: i64,
    pub inactive_player_timeout: i64,
}

impl Table {
    pub fn find_seat_by_player(&self, player: Pubkey) -> Option<usize> {
        self.seats.iter().position(|seat| seat.player == Some(player))
    }

    pub fn get_active_seat_count(&self) -> usize {
        self.seats.iter().filter(|seat| seat.player.is_some()).count()
    }

    pub fn get_current_seat(&self) -> Option<&Seat> {
        self.current_turn_index.and_then(|idx| self.seats.get(idx as usize))
    }

    pub fn get_current_seat_mut(&mut self) -> Option<&mut Seat> {
        self.current_turn_index.and_then(move |idx| self.seats.get_mut(idx as usize))
    }

    pub fn advance_turn(&mut self) -> Option<u8> {
        if self.seats.is_empty() { return None; }
    
        let next_index = match self.current_turn_index {
            Some(current_idx) => {
                let mut next_idx: u8 = (current_idx + 1) % self.seats.len() as u8;
                let start_idx = next_idx;
                loop {
                    if let Some(seat) = self.seats.get(next_idx as usize) {
                        if seat.player.is_some() && seat.bet > 0 && seat.in_action {
                            break; // Find next active player
                        }
                    }
                    next_idx = (next_idx + 1) % self.seats.len() as u8;
                    if next_idx == start_idx { return None; } // No active players
                }
                next_idx
            }
            None => {
                self.seats.iter()
                    .position(|seat| seat.player.is_some() && seat.bet > 0 && seat.in_action)
                    .map(|pos| pos as u8)
                    .unwrap_or(0) // Default to 0 if no active players (will check later)
            }
        };
    
        if let Some(current_idx) = self.current_turn_index {
            if let Some(current_seat) = self.seats.get_mut(current_idx as usize) {
                current_seat.is_turn = false;
            }
        }
        if let Some(next_seat) = self.seats.get_mut(next_index as usize) {
            if next_seat.in_action { // Only set if active
                next_seat.is_turn = true;
                next_seat.last_action_timestamp = Clock::get().unwrap().unix_timestamp;
                self.current_turn_index = Some(next_index);
                return Some(next_index);
            }
        }
    
        self.current_turn_index = None;
        None // No active players found
    }
}

