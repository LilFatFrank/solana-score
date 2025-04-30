use anchor_lang::prelude::*;

pub mod error;
pub mod states;
pub mod instructions;
pub mod constants;

use states::*;
use instructions::{
    configure::*,
    create_match::*,
    participate::*,
    distribute_prizes::*,
    withdraw::*,
    update_status::*
};

declare_id!("GfkGaCZx3QZFKGEPARZWShsA33j9q4TGxTjdMWfBEtbh");

#[program]
pub mod game_program {
    use super::*;

    pub fn initialize_global_config(ctx: Context<Configure>, config: GlobalConfig) -> Result<()> {
        ctx.accounts.process(config)
    }

    pub fn create_match_pool(ctx: Context<CreateMatchPool>, match_id: String) -> Result<()> {
        ctx.accounts.process(ctx.bumps.match_pool, match_id)
    }

    pub fn participate(ctx: Context<Participate>, amount: u64) -> Result<()> {
        ctx.accounts.process(amount)
    }

    pub fn distribute_prizes(ctx: Context<DistributePrizes>, winner: Winner) -> Result<()> {
        ctx.accounts.process(winner)
    }

    pub fn withdraw(ctx: Context<Withdraw>, amount: u64) -> Result<()> {
        ctx.accounts.process(amount)
    }

    pub fn update_match_status(ctx: Context<UpdateMatchStatus>, new_status: MatchStatus) -> Result<()> {
        ctx.accounts.process(new_status)
    }
}

