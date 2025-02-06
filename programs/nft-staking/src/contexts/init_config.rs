use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token};

use crate::state::StakeConfig;

#[derive(Accounts)]
pub struct InitConfig<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,

    #[account(
        init_if_needed,
        payer = admin,
        seeds = [
            b"rewards_mint",
            config.key().as_ref(),
        ],
        bump,
        mint::decimals = 6,
        mint::authority = config,
    )]
    pub rewards_mint: Box<Account<'info, Mint>>,

    #[account(
        init,
        payer = admin,
        space = 8 + StakeConfig::INIT_SPACE,
        seeds = [
            b"stake_config",
            admin.key().as_ref(),
        ],
        bump,
    )]
    pub config: Box<Account<'info, StakeConfig>>,

    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
}

impl InitConfig<'_> {
    pub fn init(
        &mut self,
        points_per_stake: u8,
        max_stake: u8,
        freeze_period: u64,
        bumps: &InitConfigBumps,
    ) -> Result<()> {
        self.config.set_inner(StakeConfig {
            points_per_stake,
            max_stake,
            freeze_period,
            rewards_bump: bumps.rewards_mint,
            bump: bumps.config,
        });

        Ok(())
    }
}
