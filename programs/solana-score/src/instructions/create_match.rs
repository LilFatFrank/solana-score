use anchor_lang::prelude::*;
use anchor_spl::token::TokenAccount;
use anchor_lang::solana_program::keccak::hash;
use crate::states::*;
use crate::error::GameError;
#[derive(Accounts)]
#[instruction(match_id: String)]
pub struct CreateMatchPool<'info> {
    #[account(
        init,
        payer = authority,
        space = MatchPool::LEN,
        seeds = [b"match_pool", hash(match_id.as_bytes()).to_bytes().as_ref()],
        bump
    )]
    pub match_pool: Account<'info, MatchPool>,
    #[account(mut)]
    pub authority: Signer<'info>,
    pub vault: Account<'info, TokenAccount>,
    pub system_program: Program<'info, System>,
} 

impl<'info> CreateMatchPool<'info> {
    pub fn process(
        &mut self,
        match_id: String,
    ) -> Result<()> {
        let match_pool = &mut self.match_pool;
        match_pool.match_id = hash(match_id.as_bytes()).to_bytes();
        match_pool.authority = ctx.accounts.authority.key();
        match_pool.vault = ctx.accounts.vault.key();
        match_pool.locked = false;
        match_pool.total_entries = 0;
        match_pool.total_stake = 0;
        match_pool.claimed_count = 0;
        match_pool.bump = ctx.bumps.match_pool;
        Ok(())
    }
}
