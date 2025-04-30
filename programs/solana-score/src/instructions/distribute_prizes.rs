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
    pub global_config: Account<'info, GlobalConfig>,
    #[account(mut)]
    pub vault: Account<'info, TokenAccount>,
    #[account(mut)]
    pub treasury_wallet: Account<'info, TokenAccount>,
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

        // Calculate protocol fee
        let protocol_fee = (winner.amount as u128)
            .checked_mul(self.global_config.protocol_fee_bps as u128)
            .ok_or(GameError::Overflow)?
            .checked_div(10000)
            .ok_or(GameError::Overflow)? as u64;

        let winner_amount = winner.amount.checked_sub(protocol_fee)
            .ok_or(GameError::Overflow)?;

        // Transfer protocol fee to treasury
        if protocol_fee > 0 {
            let fee_accounts = Transfer {
                from: self.vault.to_account_info(),
                to: self.treasury_wallet.to_account_info(),
                authority: self.match_pool.to_account_info(),
            };
            let seeds = [
                MatchPool::SEED_PREFIX.as_bytes(),
                &self.match_pool.match_id,
                &[self.match_pool.bump],
            ];
            let signer = &[&seeds[..]];
            let fee_ctx = CpiContext::new_with_signer(
                self.token_program.to_account_info(),
                fee_accounts,
                signer
            );
            token::transfer(fee_ctx, protocol_fee)?;
        }

        // Transfer remaining amount to winner
        let winner_account = Transfer {
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
        let winner_ctx = CpiContext::new_with_signer(
            self.token_program.to_account_info(),
            winner_account,
            signer
        );
        token::transfer(winner_ctx, winner_amount)?;

        self.match_pool.lock();
        Ok(())
    }
}
