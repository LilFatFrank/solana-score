use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount, Transfer};
use crate::states::*;
use crate::error::GameError;

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct Winner {
    pub user: Pubkey,  // Just the public key
    pub amount: u64,
}

#[derive(Accounts)]
#[instruction(winners: Vec<Winner>)]
pub struct DistributePrizes<'info> {
    #[account(mut)]
    pub match_pool: Account<'info, MatchPool>,
    #[account(mut)]
    pub vault: Account<'info, TokenAccount>,
    pub authority: Signer<'info>,
    pub token_program: Program<'info, Token>,
    #[account(mut)]
    pub winner_token_accounts: UncheckedAccount<'info>,
} 

pub fn distribute_prizes<'a, 'b, 'c, 'info>(
    ctx: Context<'a, 'b, 'c, 'info, DistributePrizes<'info>>,
    winners: Vec<Winner>
) -> Result<()> {
    let match_pool = &mut ctx.accounts.match_pool;
    match_pool.verify_authority(&ctx.accounts.authority.key())?;
    
    let total_distribution: u64 = winners.iter().map(|w| w.amount).sum();
    require!(
        match_pool.can_distribute(total_distribution),
        GameError::InsufficientFunds
    );

    require!(
        winners.len() == ctx.remaining_accounts.len(),
        GameError::InvalidWinnerAccounts
    );

    for (i, winner) in winners.iter().enumerate() {
        let winner_account = &ctx.remaining_accounts[i];
        require!(
            winner_account.key() == winner.user,
            GameError::InvalidWinnerAccount
        );

        let cpi_accounts = Transfer {
            from: ctx.accounts.vault.to_account_info(),
            to: winner_account.to_account_info(),
            authority: match_pool.to_account_info(),
        };
        let seeds = [
            MatchPool::SEED_PREFIX.as_bytes(),
            &match_pool.match_id,
            &[match_pool.bump]
        ];
        let signer = &[&seeds[..]];
        let cpi_ctx = CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            cpi_accounts,
            signer
        );
        token::transfer(cpi_ctx, winner.amount)?;
    }

    match_pool.lock();
    Ok(())
}