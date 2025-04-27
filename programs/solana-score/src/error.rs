use anchor_lang::prelude::*;

#[error_code]
pub enum GameError {
    #[msg("Match pool is already locked")]
    MatchPoolLocked,
    #[msg("Match pool is not locked")]
    MatchPoolNotLocked,
    #[msg("Invalid authority")]
    InvalidAuthority,
    #[msg("Prize already claimed")]
    PrizeAlreadyClaimed,
    #[msg("User is not a winner")]
    NotAWinner,
    #[msg("Invalid prize distribution")]
    InvalidPrizeDistribution,
    #[msg("Insufficient funds")]
    InsufficientFunds,
    #[msg("Unauthorized access")]
    UnauthorizedAccess,
    #[msg("Invalid configuration parameters")]
    InvalidConfiguration,
    #[msg("Invalid match id")]
    InvalidMatchId,
    #[msg("Invalid winner accounts")]
    InvalidWinnerAccounts,
    #[msg("Invalid winner account")]
    InvalidWinnerAccount,
}
