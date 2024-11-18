use crate::state::SwapPair;
use anchor_lang::prelude::*;
use anchor_spl::token_interface;
use std::mem::size_of;

#[derive(Accounts)]
pub struct CreatePool<'info> {
    #[account(mut)]
    pub creator: Signer<'info>,
    #[account(
        init,
        payer = creator,
        space = size_of::<SwapPair>() + 8,
        seeds = [b"swap-pair", token_b_for_pda.mint.as_ref()],
        bump,
    )]
    pub pair: Box<Account<'info, SwapPair>>,

    #[account(
        mint::token_program = token_program,
    )]
    pub token_b: Box<InterfaceAccount<'info, token_interface::Mint>>,

    #[account(
        seeds = [b"pool", token_b.key().as_ref()],
        bump,
    )]
    pub pda: AccountInfo<'info>,

    #[account(
        mut,
        token::mint = token_b,
        token::authority = pda,
        token::token_program = token_program,
    )]
    pub token_b_for_pda: Box<InterfaceAccount<'info, token_interface::TokenAccount>>,
    
    pub token_program: Interface<'info, token_interface::TokenInterface>,
    pub system_program: Program<'info, System>,
}

pub fn create_pool_handler(ctx: Context<CreatePool>) -> Result<()> {

    ctx.accounts.pair.token_b_account = ctx.accounts.token_b_for_pda.key();
    ctx.accounts.pair.token_b_mint = ctx.accounts.token_b.key();
    ctx.accounts.pair.total_supply = ctx.accounts.token_b_for_pda.amount;
    ctx.accounts.pair.reserve_token = ctx.accounts.token_b_for_pda.amount;
    ctx.accounts.pair.reserve_sol = 0;

    Ok(())
}
