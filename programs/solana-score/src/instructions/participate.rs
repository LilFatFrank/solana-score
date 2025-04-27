use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount, Transfer};
use crate::states::*;
use crate::error::GameError;
use crate::constants::*;

#[derive(Accounts)]
pub struct Participate<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        mut,
        constraint = user_token_account.mint == USDC_MINT
    )]
    pub user_token_account: Account<'info, TokenAccount>,

    #[account(
        mut,
        seeds = [b"match_pool", match_pool.match_id.as_ref()],
        bump = match_pool.bump
    )]
    pub match_pool: Account<'info, MatchPool>,

    #[account(
        mut,
        constraint = vault.key() == match_pool.vault,
        constraint = vault.mint == USDC_MINT
    )]
    pub vault: Account<'info, TokenAccount>,

    pub token_program: Program<'info, Token>,
}

impl<'info> Participate<'info> {
    pub fn process(
        &mut self,
        amount: u64
    ) -> Result<()> {
        require!(!self.match_pool.locked, GameError::MatchPoolLocked);
        
        // Update match pool state
        self.match_pool.total_entries += 1;
        self.match_pool.total_stake += amount;

        // Transfer stake to pool vault
        let cpi_accounts = Transfer {
            from: self.user_token_account.to_account_info(),
            to: self.vault.to_account_info(),
            authority: self.user.to_account_info(),
        };
        let cpi_program = self.token_program.to_account_info();
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
        token::transfer(cpi_ctx, amount)?;

        Ok(())
    }
}