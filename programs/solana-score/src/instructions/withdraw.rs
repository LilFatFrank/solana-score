use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount, Transfer};
use crate::states::*;
use crate::error::GameError;

#[derive(Accounts)]
pub struct Withdraw<'info> {
    #[account(mut)]
    pub global_config: Account<'info, GlobalConfig>,
    
    #[account(mut)]
    pub vault: Account<'info, TokenAccount>,
    
    #[account(
        mut,
        address = global_config.global_authority
    )]
    pub authority: Signer<'info>,

    #[account(mut)]
    pub authority_token_account: Account<'info, TokenAccount>,
    
    pub token_program: Program<'info, Token>,
}

impl<'info> Withdraw<'info> {
    pub fn process(&mut self, amount: u64) -> Result<()> {
        let vault_balance = self.vault.amount;
        require!(amount <= vault_balance, GameError::InsufficientFunds);

        // Transfer tokens from vault to authority
        let transfer_accounts = Transfer {
            from: self.vault.to_account_info(),
            to: self.authority_token_account.to_account_info(),
            authority: self.global_config.to_account_info(),
        };

        let cpi_ctx = CpiContext::new(
            self.token_program.to_account_info(),
            transfer_accounts,
        );

        token::transfer(cpi_ctx, amount)?;

        Ok(())
    }
}
