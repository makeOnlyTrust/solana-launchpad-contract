use anchor_lang::prelude::*;
use anchor_spl::token_interface;

#[derive(Accounts)]
pub struct CreateAccounts<'info> {
  #[account(mut)]
  pub creator: Signer<'info>,

  #[account(
    seeds = [b"pool", token_b.key().as_ref()],
    bump,
  )]
  pub pda: AccountInfo<'info>,

  pub token_b: Box<InterfaceAccount<'info, token_interface::Mint>>,

  #[account(
    init,
    payer = creator,
    token::mint = token_b,
    token::authority = pda,
    token::token_program = token_program,
  )]
  pub token_b_for_pda: Box<InterfaceAccount<'info, token_interface::TokenAccount>>,

  pub token_program: Interface<'info, token_interface::TokenInterface>,
  pub system_program: Program<'info, System>,
}

pub fn create_accounts_handler(_ctx: Context<CreateAccounts>) -> Result<()> {
  msg!("Starting Create account");
  Ok(())
}
