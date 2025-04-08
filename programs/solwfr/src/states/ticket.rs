use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Ticket {
    pub id: u64,  // start_id of range
    pub size: u64, // size of range
    pub buyer: Pubkey,
    pub lottery_id: u64,
}