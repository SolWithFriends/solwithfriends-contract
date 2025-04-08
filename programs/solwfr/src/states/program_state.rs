use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct ProgramState {
    pub initialized: bool,
    pub table_count: u64,
    pub player_count: u64,
    pub platform_fee: u64,
    pub fees_collected: u64,
    pub platform_address: Pubkey,
    pub site_authority: Pubkey,
    pub lottery_count: u64,
}

