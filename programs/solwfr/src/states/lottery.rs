use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Lottery {
    pub id: u64,
    pub authority: Pubkey,
    pub ticket_price: u64,
    pub last_ticket_id: u64,
    pub winner_set: u32,
    pub winner_id: Option<u64>,
    pub claimed: bool,
    pub claimed_by: Pubkey,

    #[max_len(32)]
    pub title: String,
    pub time_limit: u64,
    pub max_tickets: u64,
    pub is_automatic: bool,

}
