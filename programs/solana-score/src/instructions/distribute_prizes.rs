use crate::error::GameError;
use crate::states::*;
use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount, Transfer};

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct Winner {
    pub user: Pubkey, // Just the public key
    pub amount: u64,
}

#[derive(Accounts)]
pub struct DistributePrizes<'info> {
    #[account(mut)]
    pub match_pool: Account<'info, MatchPool>,
    #[account(mut)]
    pub vault: Account<'info, TokenAccount>,
    pub authority: Signer<'info>,
    /// CHECK: Validated in instruction logic
    #[account(mut)]
    pub winner_token_account: AccountInfo<'info>,
    pub token_program: Program<'info, Token>,
}

impl<'info> DistributePrizes<'info> {
    pub fn process(&mut self, winner: Winner) -> Result<()> {
        self.match_pool.verify_authority(&self.authority.key())?;

        require!(
            self.match_pool.can_distribute(winner.amount),
            GameError::InsufficientFunds
        );

        require!(
            self.winner_token_account.key() == winner.user,
            GameError::InvalidWinnerAccount
        );

        let cpi_accounts = Transfer {
            from: self.vault.to_account_info(),
            to: self.winner_token_account.to_account_info(),
            authority: self.match_pool.to_account_info(),
        };
        let seeds = [
            MatchPool::SEED_PREFIX.as_bytes(),
            &self.match_pool.match_id,
            &[self.match_pool.bump],
        ];
        let signer = &[&seeds[..]];
        let cpi_ctx = CpiContext::new_with_signer(
            self.token_program.to_account_info(),
            cpi_accounts,
            signer
        );
        token::transfer(cpi_ctx, winner.amount)?;

        self.match_pool.lock();
        Ok(())
    }
}
