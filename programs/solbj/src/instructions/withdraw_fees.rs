use anchor_lang::prelude::*;
use crate::errors::ErrorCode::*;
use crate::states::ProgramState;
pub fn withdraw_fees(ctx: Context<WithdrawFeesCtx>) -> Result<()> {
    let program_state = &mut ctx.accounts.program_state;
    
    // Verify authority
    if ctx.accounts.site_authority.key() != program_state.site_authority {
        return Err(Unauthorized.into());
    }
    
    // Get fees to withdraw
    let fees_to_withdraw = program_state.fees_collected / 2;
    
    // Transfer fees to treasury account
    let treasury = &mut ctx.accounts.treasury;
    let transfer_ix = anchor_lang::solana_program::system_instruction::transfer(
        &ctx.accounts.site_authority.key(),
        &treasury.key(),
        fees_to_withdraw
    );
    anchor_lang::solana_program::program::invoke(
        &transfer_ix,
        &[
            ctx.accounts.site_authority.to_account_info(),
            treasury.to_account_info(),
            ctx.accounts.system_program.to_account_info(),
        ]
    )?;

    // Reset fees collected
    program_state.fees_collected = 0;
    
    msg!("SOLWFR::FEES::WITHDRAW::SUCCESS: amount:{}", fees_to_withdraw);
    
    Ok(())
}

#[derive(Accounts)]
pub struct WithdrawFeesCtx<'info> {
    #[account(mut)]
    pub program_state: Account<'info, ProgramState>,
    #[account(mut)]
    /// CHECK: treasury is needed to send fees
    pub treasury: AccountInfo<'info>,
    #[account(mut)]
    pub site_authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}
