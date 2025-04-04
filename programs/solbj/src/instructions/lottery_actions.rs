use anchor_lang::prelude::*;
use anchor_lang::solana_program::hash::hash;
use crate::states::{ProgramState, Lottery, Ticket};
use crate::errors::ErrorCode::*;
use crate::constants::ANCHOR_DISCRIMINATOR_SIZE;

pub fn create_lottery(ctx: Context<CreateLotteryCtx>, ticket_price: u64, title: String, time_limit: u64, max_tickets: u64, is_automatic: bool) -> Result<()> {
    let authority = &ctx.accounts.authority;
    let lottery = &mut ctx.accounts.lottery;
    let program_state = &mut ctx.accounts.program_state;

    program_state.lottery_count += 1;
    if time_limit > 0 && max_tickets > 0{
        return Err(InvalidLotteryParameters.into());
    }

    lottery.id = program_state.lottery_count;
    lottery.authority = authority.key();
    lottery.ticket_price = ticket_price;
    lottery.title = title;
    if time_limit > 0 {
        let current_time = Clock::get()?.unix_timestamp;
        let added_time = time_limit * 60;
        let new_time_limit = current_time + added_time as i64;
        lottery.time_limit = new_time_limit as u64;
    }
    if max_tickets > 0 {
        if !is_automatic {
            return Err(InvalidLotteryParameters.into());
        }
        lottery.max_tickets = max_tickets;
    }
    lottery.is_automatic = is_automatic;
    
    msg!("SOLWFR::LOTTERY::CREATE_LOTTERY::SUCCESS:{}, lottery.id:{}, lottery.ticket_price:{}, lottery.authority:{}, lottery.time_limit:{}, lottery.max_tickets:{}, lottery.is_automatic:{}", lottery.key(), lottery.id, lottery.ticket_price, lottery.authority, lottery.time_limit, lottery.max_tickets, lottery.is_automatic);

    Ok(())
}
pub fn buy_ticket(ctx: Context<BuyTicketCtx>, ticket_amount: u64) -> Result<()> {
    let lottery = &mut ctx.accounts.lottery;
    let ticket = &mut ctx.accounts.ticket;
    let buyer = &mut ctx.accounts.buyer;
    let system_program = &ctx.accounts.system_program;

    if lottery.claimed {
        return Err(LotteryAlreadyClaimed.into());
    }

    if lottery.winner_id.is_some() {
        return Err(WinnerAlreadyExists.into());
    }

    if lottery.max_tickets > 0 && lottery.last_ticket_id + ticket_amount > lottery.max_tickets {
        return Err(InvalidLotteryParameters.into());
    }
    

    let transfer_amount = lottery.ticket_price * ticket_amount;

    let tx_instruction = anchor_lang::solana_program::system_instruction::transfer(
        &buyer.key(),
        &lottery.key(),
        transfer_amount,
    );

    anchor_lang::solana_program::program::invoke(
       &tx_instruction,
        &[buyer.to_account_info(), lottery.to_account_info(), system_program.to_account_info()],
    )?;

    let start_id = lottery.last_ticket_id + 1;
    lottery.last_ticket_id += ticket_amount;
    
    ticket.id = start_id;
    ticket.size = ticket_amount;
    ticket.lottery_id = lottery.id;
    ticket.buyer = buyer.key();

    msg!("SOLWFR::LOTTERY::BUY_TICKET::SUCCESS:{}, ticket_id:{}, ticket_amount:{}, buyer:{},", lottery.key(), ticket.id, ticket_amount, buyer.key());
    Ok(())
}

pub fn pick_winner(ctx: Context<PickWinnerCtx>, lottery_id: u64) -> Result<()> {
    let lottery = &mut ctx.accounts.lottery;

    if lottery_id != lottery.id {
        return Err(InvalidLotteryId.into());
    }

    if lottery.winner_id.is_some() {
        return Err(WinnerAlreadyExists.into());
    }

    if lottery.last_ticket_id == 0 {
        return Err(NoTicketsPurchased.into());
    }



    if lottery.is_automatic {
        if lottery.max_tickets > 0 && lottery.last_ticket_id != lottery.max_tickets {
            return Err(InvalidLotteryParameters.into());
        }
        if lottery.time_limit > 0 && lottery.time_limit > Clock::get()?.unix_timestamp as u64 {
            return Err(InvalidLotteryParameters.into());
        }
    }

    if lottery.winner_set != 0 {
        lottery.winner_id = Some(lottery.winner_set as u64);
        msg!("SOLWFR:LOTTERY:Winner picked: {}", lottery.winner_id.unwrap());
       return Ok(());
    }

    let clock = Clock::get()?;
    let hash_bytes = hash(&clock.unix_timestamp.to_be_bytes()).to_bytes();
    let random_bytes = <[u8; 8]>::try_from(&hash_bytes[..8]).unwrap();
    
    // Pick a random number between 1 and last_ticket_id
    let winner_id = (u64::from_le_bytes(random_bytes) % lottery.last_ticket_id) + 1;
    lottery.winner_id = Some(winner_id);

    msg!("SOLWFR::LOTTERY::PICK_WINNER::SUCCESS:{}, lottery.id:{}, winner_id:{}", lottery.key(), lottery.id, winner_id);
    Ok(())
}


pub fn claim_prize(ctx: Context<ClaimPrizeCtx>) -> Result<()> {
    let lottery = &mut ctx.accounts.lottery;
    let ticket = &ctx.accounts.ticket;
    let buyer = &ctx.accounts.buyer;
    let site_authority = &ctx.accounts.site_authority;
    let program_state = &mut ctx.accounts.program_state;

    if site_authority.key() != program_state.site_authority {
        return Err(InvalidAuthority.into());
    }

    if lottery.id != ticket.lottery_id {
        return Err(InvalidLotteryId.into());
    }

    if buyer.key() != ticket.buyer {
        return Err(InvalidTicketId.into());
    }

    if lottery.claimed {
        return Err(LotteryAlreadyClaimed.into());
    }

    // Get the winning ticket ID
    let winner_id = lottery.winner_id.ok_or(NoWinner)?;

    // Check if the winning ID falls within this ticket's range
    let ticket_end = ticket.id + ticket.size - 1;
    if winner_id < ticket.id || winner_id > ticket_end {
        return Err(InvalidWinner.into());
    }

    let prize = lottery.ticket_price.checked_mul(lottery.last_ticket_id.into()).unwrap();
    // Calculate 0.5% fee: prize * 0.005
    let program_fee = prize.checked_mul(5).unwrap().checked_div(1000).unwrap();

    **lottery.to_account_info().try_borrow_mut_lamports()? -= prize;
    **buyer.to_account_info().try_borrow_mut_lamports()? += prize - program_fee;
    **ctx.accounts.site_authority.to_account_info().try_borrow_mut_lamports()? += program_fee;
    program_state.fees_collected += program_fee;
    lottery.claimed = true;
    lottery.claimed_by = buyer.key();
    msg!("SOLWFR::LOTTERY::CLAIM_PRIZE::SUCCESS:{},  prize:{}, buyer:{}, program_fee:{}, ", lottery.key(), prize, buyer.key(), program_fee);


    Ok(())
}

#[derive(Accounts)]
pub struct ClaimPrizeCtx<'info> {
    #[account(mut)]  
    pub lottery: Account<'info, Lottery>,
    #[account( has_one = buyer,)]
    pub ticket: Account<'info, Ticket>,
    #[account(mut)]
    pub buyer: Signer<'info>,
    /// CHECK: site_authority is needed for fee collection
    #[account(mut)]
    pub site_authority: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
    #[account(mut)]
    pub program_state: Account<'info, ProgramState>,
}

#[derive(Accounts)]
#[instruction(lottery_id: u64)]
pub struct PickWinnerCtx<'info> {
    #[account(mut)]
    pub program_state: Account<'info, ProgramState>,
    #[account(
        mut,
        seeds = [b"lottery", &lottery_id.to_le_bytes()],
        bump
    )]
    pub lottery: Account<'info, Lottery>,
    pub authority: Signer<'info>,
}





#[derive(Accounts)]
pub struct BuyTicketCtx<'info> {
    #[account(mut)]
    pub lottery: Account<'info, Lottery>,

    #[account(mut)]
    pub buyer: Signer<'info>,

    #[account(
        init,
        payer = buyer,
        space = ANCHOR_DISCRIMINATOR_SIZE + Ticket::INIT_SPACE,
        seeds = [
            b"ticket", 
            lottery.key().as_ref(),
            &(lottery.last_ticket_id + 1).to_le_bytes(),
        ],
        bump,
    )]
    pub ticket: Account<'info, Ticket>,

    pub system_program: Program<'info, System>,
}


#[derive(Accounts)]
pub struct CreateLotteryCtx<'info> {
    #[account(mut)]
    pub program_state: Account<'info, ProgramState>,
    #[account(
        init,
        payer = authority,
        space = ANCHOR_DISCRIMINATOR_SIZE + Lottery::INIT_SPACE,
        seeds = [b"lottery", (program_state.lottery_count + 1).to_le_bytes().as_ref()],
        bump
    )]
    pub lottery: Account<'info, Lottery>,
    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}
