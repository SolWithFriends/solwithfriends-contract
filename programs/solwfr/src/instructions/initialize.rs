use anchor_lang::prelude::*;

use crate::constants::ANCHOR_DISCRIMINATOR_SIZE;
use crate::errors::ErrorCode::AlreadyInitialized;
use crate::states::ProgramState;

pub fn initialize(ctx: Context<InitializeCtx>) -> Result<()> {
    let state = &mut ctx.accounts.program_state;
    let deployer = &ctx.accounts.deployer;

    if state.initialized {
        return Err(AlreadyInitialized.into());
    }

    state.table_count = 0;
    state.player_count = 0;
    state.lottery_count = 0;
    state.platform_fee = 1;
    state.fees_collected = 0;
    state.platform_address = deployer.key();
    state.initialized = true;
    state.site_authority = ctx.accounts.site_authority.key();

 
    Ok(())
}

#[derive(Accounts)]
pub struct InitializeCtx<'info> {
    #[account(
        init,
        payer = deployer,
        space = ANCHOR_DISCRIMINATOR_SIZE + ProgramState::INIT_SPACE,
        seeds = [b"program_state"],
        bump
    )]
    pub program_state: Account<'info, ProgramState>,
    ///CHECK: Just used to set authority field, not a signer
    pub site_authority: AccountInfo<'info>,
    #[account(mut)]
    pub deployer: Signer<'info>,
    pub system_program: Program<'info, System>,
}
