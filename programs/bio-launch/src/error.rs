use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {
    #[msg("Invalid arguments")]
    InvalidArgument,

    #[msg("InvalidInput")]
    InvalidInput,

    #[msg("Swap instruction exceeds desired slippage limit")]
    ExceededSlippage,

    #[msg("Given pool token amount results in zero trading tokens")]
    ZeroTradingTokens,

    #[msg("sol amount is small than require")]
    SolInsufficient,

    #[msg("Given Token amount too big")]
    TokenAmountTooBig,
}
