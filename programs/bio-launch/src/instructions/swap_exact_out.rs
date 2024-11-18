use anchor_lang::{
    prelude::*,
    system_program
};
use anchor_spl::{
    token::{self, Transfer},
    token_interface,
};
use crate::{
    error::ErrorCode,
    state::{
        SwapPair,
        constants::PROPORTION
    },
};

#[derive(Accounts)]
#[instruction(amount_in: u64)]
pub struct SwapExactOut<'info> {
    pub swapper: Signer<'info>,

    #[account(
        mut,
        seeds = [
            b"pool",
            pair.token_b_mint.key().as_ref()],
        bump,
    )]
    pub pda: AccountInfo<'info>,

    #[account(token::token_program = token_program)]
    pub token_destination: Box<InterfaceAccount<'info, token_interface::Mint>>,

    #[account(
        mut,
        constraint = pair.token_b_account == token_destination_for_pda.key(),
        constraint = pair.token_b_mint == token_destination_for_pda.mint,
    )]
    pub pair: Box<Account<'info, SwapPair>>,

    #[account(
        mut,
        token::mint = token_destination,
        token::authority = swapper,
        token::token_program = token_program,
    )]
    pub token_destination_for_swapper: Box<InterfaceAccount<'info, token_interface::TokenAccount>>,

    #[account(
        mut,
        token::mint = token_destination,
        token::authority = pda,
        token::token_program = token_program,
    )]
    pub token_destination_for_pda: Box<InterfaceAccount<'info, token_interface::TokenAccount>>,

    pub token_program: Interface<'info, token_interface::TokenInterface>,
    pub system_program: Program<'info, System>,
}

impl<'info> SwapExactOut<'info> {

    fn to_transfer_source_context(&self) -> CpiContext<'_, '_, '_, 'info, Transfer<'info>> {
        let cpi_accounts: Transfer = Transfer {
            from: self.token_destination_for_swapper.to_account_info().clone(),
            to: self.token_destination_for_pda.to_account_info().clone(),
            authority: self.swapper.to_account_info().clone(),
        };
        CpiContext::new(self.token_program.to_account_info(), cpi_accounts)
    }

}

pub fn swap_exact_out_handler<'a, 'b, 'c, 'info>(
    ctx: Context<'a, 'b, 'c, 'info, SwapExactOut<'info>>,
    amount_in: u64
) -> Result<()> {
    msg!("Instruction Sell {}", amount_in);
    if amount_in == 0 {
        return err!(ErrorCode::InvalidInput);
    }

    let pair = &mut ctx.accounts.pair;

    if amount_in > pair.reserve_token {
        return err!(ErrorCode::TokenAmountTooBig);
    }

    let bought_amount = (pair.total_supply as f64 - pair.reserve_token as f64) / 1_000_000.0 / 1_000_000_000.0;
    msg!("bought_amount: {}", bought_amount);

    let result_amount =
        (pair.total_supply as f64 - pair.reserve_token as f64 - amount_in as f64) / 1_000_000.0 / 1_000_000_000.0;
    msg!("result_amount: {}", result_amount);

    let amount_out_f64 =
        (bought_amount * bought_amount - result_amount * result_amount) / PROPORTION as f64 * 1_000_000_000.0;
    msg!("amount_out_f64: {}", amount_out_f64);

    let amount_out = amount_out_f64.round() as u64;
    msg!("amount_out: {}", amount_out);

    msg!("reserve_sol: {}", pair.reserve_sol);

    if amount_out > pair.reserve_sol {
        return err!(ErrorCode::SolInsufficient);
    }

    pair.reserve_token += amount_in;
    pair.reserve_sol -= amount_out;
    
    let signer_seeds = ctx
        .accounts
        .pair
        .signer_seeds(&ctx.accounts.pda, ctx.program_id)?;
    let signer_seeds: &[&[&[u8]]; 1] = &[&signer_seeds.value()[..]];

    token::transfer(
        ctx.accounts
            .to_transfer_source_context(),
            amount_in,
    )?;

    system_program::transfer(
        CpiContext::new(
            ctx.accounts.system_program.to_account_info(),
            system_program::Transfer {
                from: ctx.accounts.pda.to_account_info(),
                to: ctx.accounts.swapper.to_account_info(),
            },
        ).with_signer(signer_seeds),
        amount_out,
    )?;

    Ok(())
}
