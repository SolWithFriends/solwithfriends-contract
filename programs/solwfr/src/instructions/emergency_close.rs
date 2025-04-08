use anchor_lang::prelude::*;
use crate::errors::ErrorCode::*;
use crate::states::{ProgramState, Table, PlayerWallet};

pub fn emergency_close_table(ctx: Context<EmergencyCloseTableCtx>) -> Result<()> {
    let table = &mut ctx.accounts.table;
    let dealer = &mut ctx.accounts.dealer;

    let site_authority = &mut ctx.accounts.site_authority;
    let program_state = &mut ctx.accounts.program_state;

    if site_authority.key() != program_state.site_authority {
        return Err(InvalidAuthority.into());
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

#[derive(Accounts)]
pub struct EmergencyCloseTableCtx<'info> {
    #[account(mut)]
    pub program_state: Account<'info, ProgramState>,
    #[account(
        mut,
        close = dealer,
    )]
    pub table: Account<'info, Table>,
    #[account(mut)]
    /// CHECK: dealer is needed to receive the remaining balance
    pub dealer: AccountInfo<'info>,
    #[account(mut)]
    pub site_authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}


pub fn emergency_close_wallet(ctx: Context<EmergencyCloseWalletCtx>) -> Result<()> {
    let player_wallet = &mut ctx.accounts.player_wallet;
    let player = &mut ctx.accounts.player;

    let site_authority = &mut ctx.accounts.site_authority;
    let program_state = &mut ctx.accounts.program_state;

    if site_authority.key() != program_state.site_authority {
        return Err(InvalidAuthority.into());
    }

    if player_wallet.player != player.key() {
        return Err(InvalidPlayer.into());
    }
    
    msg!("SOLWFR::BLACKJACK::CLOSE_WALLET::SUCCESS:{}, player:{},", player_wallet.key(), player.key());

    Ok(())
}

#[derive(Accounts)]
pub struct EmergencyCloseWalletCtx<'info> {
    #[account(mut)]
    pub program_state: Account<'info, ProgramState>,
    #[account(mut, close = player)]
    pub player_wallet: Account<'info, PlayerWallet>,
    #[account(mut)]
    /// CHECK: player is needed to receive the remaining balance
    pub player: AccountInfo<'info>,
    #[account(mut)]
    pub site_authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}