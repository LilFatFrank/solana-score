use anchor_lang::prelude::*;

#[account]
pub struct GlobalConfig {
    pub global_authority: Pubkey,      
    pub protocol_fee_bps: u16,         
    pub leaderboard_cut_bps: u16,      
    pub treasury_wallet: Pubkey,
    pub leaderboard_vault: Pubkey,
    pub bump: u8,
    pub reserved: [u8; 64],  // Reserved space for future upgrades
}

impl GlobalConfig {
    pub const SEED_PREFIX: &'static str = "global_config";
    pub const LEN: usize = 8 + // discriminator
                          32 + // global_authority
                          2 +  // protocol_fee_bps
                          2 +  // leaderboard_cut_bps
                          32 + // treasury_wallet
                          32 + // leaderboard_vault
                          1 +  // bump
                          64;  // reserved space
}