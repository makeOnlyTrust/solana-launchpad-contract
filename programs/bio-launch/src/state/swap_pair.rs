use anchor_lang::prelude::*;
use crate::error::ErrorCode;

pub struct SwapResult {
    pub source_amount_swapped: u128,
    pub destination_amount_swapped: u128,
}

#[account]
pub struct SwapPair {
    pub token_a_account: Pubkey,
    pub token_b_account: Pubkey,
    pub token_a_mint: Pubkey,
    pub token_b_mint: Pubkey,
    pub total_supply: u64,
    pub reserve_token: u64,
    pub reserve_sol: u64,
}

impl SwapPair {
  pub fn signer_seeds<'a>(&'a self, pda: &AccountInfo, program_id: &Pubkey) -> Result<SignerSeeds> {
      let seeds: [&[u8]; 2] = [
          b"pool".as_ref(),
          self.token_b_mint.as_ref(),
      ];
      let (pubkey, bump_seed) = Pubkey::find_program_address(&seeds, program_id);
      if pubkey != pda.key() {
          return err!(ErrorCode::InvalidArgument);
      }
      Ok(SignerSeeds(seeds, [bump_seed]))
  }

}

pub struct SignerSeeds<'a>([&'a [u8]; 2], [u8; 1]);

impl<'a> SignerSeeds<'a> {
    pub fn value(&self) -> [&[u8]; 3] {
        [self.0[0], self.0[1], &self.1]
    }
}
