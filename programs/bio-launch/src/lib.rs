pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

pub use instructions::*;

declare_id!("BDm9yRZ4NnSAEg8dc7o5Wugm8xguCS3XZETEpyKHBko6");

#[program]
pub mod bio_launch {
    use super::*;

    pub fn create_accounts(ctx: Context<CreateAccounts>) -> Result<()> {
        create_accounts_handler(ctx)
    }

    pub fn create_pool(ctx: Context<CreatePool>) -> Result<()> {
        create_pool_handler(ctx)
    }

    pub fn withdraw_all<'a, 'b, 'c, 'info>(
        ctx: Context<'a, 'b, 'c, 'info, WithdrawAll<'info>>,
    ) -> Result<()> {
        withdraw_all_handler(ctx)
    }

    pub fn swap_exact_in<'a, 'b, 'c, 'info>(
        ctx: Context<'a, 'b, 'c, 'info, SwapExactIn<'info>>,
        amount_in: u64
    ) -> Result<()> {
        swap_exact_in_handler(ctx, amount_in)
    }

    pub fn swap_exact_out<'a, 'b, 'c, 'info>(
        ctx: Context<'a, 'b, 'c, 'info, SwapExactOut<'info>>,
        amount_out: u64
    ) -> Result<()> {
        swap_exact_out_handler(ctx, amount_out)
    }
    
}
