use anchor_lang::prelude::*;
use anchor_spl::token::{Token, TokenAccount, Mint};
use anchor_lang::solana_program::keccak::hash;
use crate::states::*; // Your MatchPool struct, etc.

#[derive(Accounts)]
#[instruction(match_id: String)]
pub struct CreateMatchPool<'info> {
    #[account(
        init,
        payer = authority,
        space = MatchPool::LEN,
        seeds = [b"match_pool", match_id.as_bytes()],
        bump
    )]
    pub match_pool: Account<'info, MatchPool>,

    #[account(
        init,
        payer = authority,
        seeds = [b"vault", match_pool.key().as_ref()],
        bump,
        token::mint = usdc_mint,
        token::authority = vault_authority,
    )]
    pub vault: Account<'info, TokenAccount>,

    /// CHECK: This is safe, we'll derive vault_authority from PDA
    #[account(
        seeds = [b"vault_authority", match_pool.key().as_ref()],
        bump
    )]
    pub vault_authority: UncheckedAccount<'info>,

    #[account(mut)]
    pub authority: Signer<'info>,

    pub usdc_mint: Account<'info, Mint>,

    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub rent: Sysvar<'info, Rent>,
}


impl<'info> CreateMatchPool<'info> {
    pub fn process(&mut self, bump: u8, match_id: String) -> Result<()> {
        
        let match_pool = &mut self.match_pool;
        match_pool.authority = self.authority.key();
        match_pool.match_id = hash(match_id.as_bytes()).to_bytes();
        match_pool.bump = bump;
        match_pool.vault = self.vault.key();
        match_pool.locked = false;
        match_pool.total_entries = 0;
        match_pool.total_stake = 0;
        match_pool.claimed_count = 0;
        Ok(())
    }
}
