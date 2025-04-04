use anchor_lang::prelude::*;

#[derive(
    AnchorSerialize, AnchorDeserialize,
    Clone, Copy, Debug, PartialEq,
)]
#[derive(InitSpace)]
pub struct Card {
    pub rank: u8,  // 1-13 (Ace = 1, Jack = 11, etc.)
    pub suit: u8,  // 0-3 (Hearts, Diamonds, Spades, Clubs)
}

#[derive(
    AnchorSerialize, AnchorDeserialize,
    Clone, Debug,
)]
#[derive(InitSpace)]
pub struct Hand {
    #[max_len(52)]
    pub cards: Vec<Card>,
    pub is_soft: bool,    // Has an Ace counted as 11
    pub value: u8,        // Current hand value
    pub is_blackjack: bool,
    pub is_bust: bool,
}

impl Card {
    pub fn new(rank: u8, suit: u8) -> Self {
        Self { rank, suit }
    }
}

impl Hand {
    pub fn new() -> Self {
        Self {
            cards: Vec::new(),
            is_soft: false,
            value: 0,
            is_blackjack: false,
            is_bust: false,
        }
    }

    pub fn add_card(&mut self, card: Card) {
        self.cards.push(card);
        self.calculate_value();
    }

    pub fn calculate_value(&mut self) {
        let mut value = 0;
        let mut aces = 0;
        
        for card in &self.cards {
            match card.rank {
                1 => { aces += 1; }  // Ace
                11..=13 => { value += 10; }  // Face cards
                _ => { value += card.rank; }
            }
        }

        // Handle aces
        self.is_soft = false;
        for i in 0..aces {
            // For first ace, try 11 if possible
            if i == 0 && value + 11 <= 21 {
                value += 11;
                self.is_soft = true;
            } else {
                // All subsequent aces are 1
                value += 1;
            }
        }

        self.value = value;
        self.is_bust = value > 21;
        self.is_blackjack = self.cards.len() == 2 && value == 21;
    }
}

pub fn create_shuffled_deck(seed: u64, num_decks: u8) -> Vec<Card> {
    let mut deck = Vec::with_capacity(52 * num_decks as usize);
    
    // Create ordered deck
    for _ in 0..num_decks {
        for suit in 0..4 {
            for rank in 1..14 {
                deck.push(Card::new(rank, suit));
            }
        }
    }
    
    // Fisher-Yates shuffle using seed
    let mut rng = seed;
    for i in (1..deck.len()).rev() {
        // Simple RNG using seed
        rng = rng.wrapping_mul(1103515245).wrapping_add(12345);
        let j = (rng % (i as u64 + 1)) as usize;
        deck.swap(i, j);
    }
    
    deck
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug)]
#[derive(InitSpace)]
pub struct HandResult {
    pub player_pubkey: Pubkey,
    pub player_hand: Hand,
    pub dealer_hand: Hand,
    pub player_won: bool,
    pub bet_amount: u64,
    pub payout_amount: u64,
    pub timestamp: i64,
} 