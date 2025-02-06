use anchor_lang::prelude::*;

declare_id!("9AtqAo7BUPvB96Vx4L1yv8We32WGVHxj7p8LFRpCAM3w");

pub mod contexts;
pub mod errors;
pub mod state;

use contexts::*;

#[program]
pub mod nft_staking {

    use super::*;

    pub fn initialize_config(
        ctx: Context<InitConfig>,
        points_per_stake: u8,
        max_stake: u8,
        freeze_period: u64,
    ) -> Result<()> {
        ctx.accounts
            .init(points_per_stake, max_stake, freeze_period, &ctx.bumps)
    }

    pub fn initialize_user(ctx: Context<Initialize>) -> Result<()> {
        ctx.accounts.initialize_user(&ctx.bumps)
    }

    pub fn stake(ctx: Context<Stake>) -> Result<()> {
        ctx.accounts.stake(&ctx.bumps)
    }

    pub fn unstake(ctx: Context<Unstake>) -> Result<()> {
        ctx.accounts.unstake()
    }

    pub fn claim(ctx: Context<Claim>) -> Result<()> {
        ctx.accounts.claim()
    }
}
