use anchor_lang::prelude::*;

#[constant]
pub const SEED: [u8; 6] = *b"anchor";
pub const PROPORTION: u64 = 1280;      //  800M token is sold on 500SOL ===> (500 * 2 / 800) = 1.25 ===> 800 : 1.25 = 640 ====> 640 * 2 = 1280