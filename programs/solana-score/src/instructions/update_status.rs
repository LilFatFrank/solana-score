use anchor_lang::prelude::*;
use crate::states::*;
use crate::error::GameError;

#[derive(AnchorSerialize, AnchorDeserialize)]
pub enum MatchStatus {
    Active,
    Locked
}

#[derive(Accounts)]
pub struct UpdateMatchStatus<'info> {
    #[account(mut)]
    pub match_pool: Account<'info, MatchPool>,

    #[account(
        address = match_pool.authority
    )]
    pub authority: Signer<'info>,
}

impl<'info> UpdateMatchStatus<'info> {
    pub fn process(&mut self, new_status: MatchStatus) -> Result<()> {
        self.match_pool.verify_authority(&self.authority.key())?;

        match new_status {
            MatchStatus::Locked => {
                require!(self.match_pool.is_active(), GameError::MatchPoolLocked);
                self.match_pool.update_status(true)?;
            },
            MatchStatus::Active => {
                require!(!self.match_pool.is_active(), GameError::MatchPoolNotLocked);
                self.match_pool.update_status(false)?;
            }
        }

        Ok(())
    }
} 