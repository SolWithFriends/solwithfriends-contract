use anchor_lang::prelude::*;
use crate::states::{PlayerWallet, ProgramState};
use crate::errors::ErrorCode::*;
use crate::constants::ANCHOR_DISCRIMINATOR_SIZE;

pub fn initialize_player_wallet(ctx: Context<InitializePlayerWallet>) -> Result<()> {
    let player_wallet = &mut ctx.accounts.player_wallet;
    if ctx.accounts.program_state.site_authority != ctx.accounts.site_authority.key() {
        return Err(InvalidAuthority.into());
    }
    player_wallet.player = ctx.accounts.player.key();
    player_wallet.balance = 0;
    player_wallet.authority = ctx.accounts.site_authority.key();
    player_wallet.bump = ctx.bumps.player_wallet;
    ctx.accounts.program_state.player_count = ctx.accounts.program_state.player_count.checked_add(1).unwrap();
    msg!("SOLWFR::PLAYER_WALLET::INITIALIZE::SUCCESS:{}, player:{},", player_wallet.key(), player_wallet.player);
    Ok(())
}

pub fn deposit(ctx: Context<Deposit>, amount: u64) -> Result<()> {
    let player_wallet = &mut ctx.accounts.player_wallet;
    let transfer_ix = anchor_lang::solana_program::system_instruction::transfer(
        &ctx.accounts.player.key(),
        &player_wallet.key(),
        amount
    );
    anchor_lang::solana_program::program::invoke(
        &transfer_ix,
        &[
            ctx.accounts.player.to_account_info(),
            player_wallet.to_account_info(),
            ctx.accounts.system_program.to_account_info(),
        ]
    )?;
    player_wallet.balance = player_wallet.balance.checked_add(amount).unwrap();
    msg!("SOLWFR::PLAYER_WALLET::DEPOSIT::SUCCESS:{}, player:{}, amount:{}", player_wallet.key(), player_wallet.player, amount);
    Ok(())
}

pub fn withdraw(ctx: Context<Withdraw>, amount: u64) -> Result<()> {
    let player_wallet = &mut ctx.accounts.player_wallet;
    if player_wallet.player != ctx.accounts.player.key() {
        return Err(InvalidPlayer.into());
    }
    let rent = Rent::get()?;
    let min_balance = rent.minimum_balance(player_wallet.to_account_info().data_len());
    if player_wallet.to_account_info().lamports() - amount < min_balance {
        return Err(InsufficientFunds.into());
    }

    **player_wallet.to_account_info().try_borrow_mut_lamports()? -= amount;
    **ctx.accounts.player.to_account_info().try_borrow_mut_lamports()? += amount;
    player_wallet.balance = player_wallet.balance.checked_sub(amount).unwrap();
    msg!("SOLWFR::PLAYER_WALLET::WITHDRAW::SUCCESS:{}, player:{}, amount:{}", player_wallet.key(), player_wallet.player, amount);
    Ok(())
}



#[derive(Accounts)]
pub struct InitializePlayerWallet<'info> {
    #[account(
        init,
        payer = player, // User pays rent
        space = ANCHOR_DISCRIMINATOR_SIZE + PlayerWallet::INIT_SPACE,
        seeds = [b"player_wallet", player.key().as_ref()],
        bump
    )]
    pub player_wallet: Account<'info, PlayerWallet>,
    #[account(mut)]
    pub player: Signer<'info>, // User signs
    /// CHECK: Just used to set authority field, not a signer
    pub site_authority: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
    pub program_state: Account<'info, ProgramState>,
}

#[derive(Accounts)]
pub struct Deposit<'info> {
    #[account(mut, has_one = player)]
    pub player_wallet: Account<'info, PlayerWallet>,
    #[account(mut)]
    pub player: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Withdraw<'info> {
    #[account(mut, has_one = player)]
    pub player_wallet: Account<'info, PlayerWallet>,
    #[account(mut)]
    pub player: Signer<'info>,
    pub system_program: Program<'info, System>,
}