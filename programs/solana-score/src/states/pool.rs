use anchor_lang::prelude::*;
use crate::error::GameError;

#[account]
pub struct MatchPool {
    pub match_id: [u8; 32],            
    pub authority: Pubkey,             
    pub vault: Pubkey,                 
    pub locked: bool,
    pub total_entries: u32,
    pub total_stake: u64,              
    pub claimed_count: u32,
    pub bump: u8,
}

impl MatchPool {
    pub const SEED_PREFIX: &'static str = "match_pool";
    
    pub const LEN: usize = 8 +  // discriminator
                          32 +  // match_id
                          32 +  // authority
                          32 +  // vault
                          1 +   // locked
                          4 +   // total_entries
                          8 +   // total_stake
                          4 +   // claimed_count
                          1;    // bump

    pub fn is_active(&self) -> bool {
        !self.locked
    }

    pub fn can_distribute(&self, total_distribution: u64) -> bool {
        !self.locked && total_distribution <= self.total_stake
    }

    pub fn verify_authority(&self, authority: &Pubkey) -> Result<()> {
        require!(self.authority == *authority, GameError::UnauthorizedAccess);
        Ok(())
    }

    pub fn lock(&mut self) {
        self.locked = true;
    }

    pub fn participate(&mut self, stake_amount: u64) -> Result<()> {
        require!(!self.locked, GameError::MatchPoolLocked);
        
        self.total_entries += 1;
        self.total_stake = self.total_stake.checked_add(stake_amount)
            .ok_or(GameError::InvalidPrizeDistribution)?;
        
        Ok(())
    }

    pub fn verify_config(&mut self, authority: &Pubkey) -> Result<()> {
        self.verify_authority(authority)?;
        require!(!self.locked, GameError::MatchPoolLocked);
        Ok(())
    }

    pub fn validate_match_id(
        &self,
        match_id: String,
    ) -> Result<()> {
        if self.match_id != hash(match_id.as_bytes()).to_bytes() {
            return Err(GameError::InvalidMatchId.into());
        }
        Ok(())
    }

    pub fn update_status(
        &mut self,
        status: bool,
    ) -> Result<()> {
        self.locked = status;
        Ok(())
    }
}
