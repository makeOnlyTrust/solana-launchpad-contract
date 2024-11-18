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
pub struct SwapExactIn<'info> {
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

impl<'info> SwapExactIn<'info> {

    fn to_transfer_destination_context(&self) -> CpiContext<'_, '_, '_, 'info, Transfer<'info>> {
        let cpi_accounts: Transfer = Transfer {
            from: self.token_destination_for_pda.to_account_info().clone(),
            to: self.token_destination_for_swapper.to_account_info().clone(),
            authority: self.pda.to_account_info().clone(),
        };
        CpiContext::new(self.token_program.to_account_info(), cpi_accounts)
    }

}

pub fn swap_exact_in_handler<'a, 'b, 'c, 'info>(
    ctx: Context<'a, 'b, 'c, 'info, SwapExactIn<'info>>,
    amount_in: u64
) -> Result<()> {
    msg!("Instruction Buy {}", amount_in);
    if amount_in == 0 {
        return err!(ErrorCode::InvalidInput);
    }

    let pair = &mut ctx.accounts.pair;

    let bought_amount = (pair.total_supply as f64 - pair.reserve_token as f64) / 1_000_000.0 / 1_000_000_000.0;
    msg!("bought_amount {}", bought_amount);

    let root_val = (PROPORTION as f64 * amount_in as f64 / 1_000_000_000.0 + bought_amount * bought_amount).sqrt();
    msg!("root_val {}", root_val);

    let amount_out_f64 = (root_val - bought_amount as f64) * 1_000_000.0 * 1_000_000_000.0;
    msg!("amount_out_f64 {}", amount_out_f64);

    let amount_out = amount_out_f64.round() as u64;
    msg!("amount_out {}", amount_out);

    if amount_out > pair.reserve_token {
        return err!(ErrorCode::ZeroTradingTokens);
    }
    
    pair.reserve_sol += amount_in;
    pair.reserve_token -= amount_out;

    let signer_seeds = ctx
        .accounts
        .pair
        .signer_seeds(&ctx.accounts.pda, ctx.program_id)?;
    let signer_seeds: &[&[&[u8]]; 1] = &[&signer_seeds.value()[..]];

    system_program::transfer(
        CpiContext::new(
            ctx.accounts.system_program.to_account_info(),
            system_program::Transfer {
                from: ctx.accounts.swapper.to_account_info(),
                to: ctx.accounts.pda.to_account_info(),
            },
        ),
        amount_in,
    )?;

    token::transfer(
        ctx.accounts
            .to_transfer_destination_context()
            .with_signer(signer_seeds),
            amount_out,
    )?;

    Ok(())
}
