use anchor_lang::prelude::*;

pub mod error;
pub mod states;
pub mod instructions;
pub mod constants;

use states::*;
use instructions::*;

declare_id!("GfkGaCZx3QZFKGEPARZWShsA33j9q4TGxTjdMWfBEtbh");

#[program]
pub mod game_program {
    use super::*;

    pub fn initialize_global_config(ctx: Context<configure::Configure>, config: GlobalConfig) -> Result<()> {
        ctx.accounts.process(config)
    }

    pub fn create_match_pool(ctx: Context<create_match::CreateMatchPool>, match_id: String) -> Result<()> {
        ctx.accounts.process(match_id)
    }

    pub fn participate(ctx: Context<participate::Participate>, amount: u64) -> Result<()> {
        ctx.accounts.process(amount)
    }

    pub fn distribute_prizes(ctx: Context<DistributePrizes>, winners: Vec<distribute_prizes::Winner>) -> Result<()> {
        instructions::distribute_prizes::distribute_prizes(ctx, winners)
    }
}
