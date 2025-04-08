use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct PlayerWallet {
    pub player: Pubkey,
    pub balance: u64,
    pub authority: Pubkey,
    pub bump: u8,
}



