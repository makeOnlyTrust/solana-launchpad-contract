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
pub struct WithdrawAll<'info> {
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

impl<'info> WithdrawAll<'info> {

    fn to_transfer_destination_context(&self) -> CpiContext<'_, '_, '_, 'info, Transfer<'info>> {
        let cpi_accounts: Transfer = Transfer {
            from: self.token_destination_for_pda.to_account_info().clone(),
            to: self.token_destination_for_swapper.to_account_info().clone(),
            authority: self.pda.to_account_info().clone(),
        };
        CpiContext::new(self.token_program.to_account_info(), cpi_accounts)
    }

}

pub fn withdraw_all_handler<'a, 'b, 'c, 'info>(
    ctx: Context<'a, 'b, 'c, 'info, WithdrawAll<'info>>
) -> Result<()> {
    msg!("Instruction Remove");

    let pair = &mut ctx.accounts.pair;

    let signer_seeds = ctx
        .accounts
        .pair
        .signer_seeds(&ctx.accounts.pda, ctx.program_id)?;
    let signer_seeds: &[&[&[u8]]; 1] = &[&signer_seeds.value()[..]];

    // system_program::transfer(
    //     CpiContext::new(
    //         ctx.accounts.system_program.to_account_info(),
    //         system_program::Transfer {
    //             from: ctx.accounts.swapper.to_account_info(),
    //             to: ctx.accounts.pda.to_account_info(),
    //         },
    //     ),
    //     pair.reserve_sol,
    // )?;

    token::transfer(
        ctx.accounts
            .to_transfer_destination_context()
            .with_signer(signer_seeds),
            pair.reserve_token,
    )?;

    // pair.reserve_sol += amount_in;
    // pair.reserve_token -= amount_out;

    Ok(())
}
