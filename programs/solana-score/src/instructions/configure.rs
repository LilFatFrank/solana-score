use anchor_lang::{prelude::*, system_program};
use crate::states::GlobalConfig;
use crate::error::GameError;

#[derive(Accounts)]
pub struct Configure<'info> {
    #[account(mut)]
    admin: Signer<'info>,

    #[account(init_if_needed,
        payer = admin,
        seeds = [GlobalConfig::SEED_PREFIX.as_bytes()],
        space = 8 + GlobalConfig::LEN,
        bump)]
    global_config: Account<'info, GlobalConfig>,

    #[account(address = system_program::ID)]
    system_program: Program<'info, System>,
}

impl<'info> Configure<'info> {
    pub fn process(
        &mut self,
        config: GlobalConfig,
    ) -> Result<()> {
        // If config is not initialized, set the admin as the authority
        if self.global_config.global_authority.eq(&Pubkey::default()) {
            self.global_config.global_authority = self.admin.key();
        } else {
            require!(self.global_config.global_authority == self.admin.key(), GameError::InvalidAuthority);
        }

        self.global_config.protocol_fee_bps = config.protocol_fee_bps;
        self.global_config.leaderboard_cut_bps = config.leaderboard_cut_bps;
        self.global_config.treasury_wallet = config.treasury_wallet;
        self.global_config.leaderboard_vault = config.leaderboard_vault;
        
        Ok(())
    }
}
