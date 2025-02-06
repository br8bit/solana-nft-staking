use anchor_lang::prelude::*;

declare_id!("9AtqAo7BUPvB96Vx4L1yv8We32WGVHxj7p8LFRpCAM3w");

pub mod contexts;
pub mod state;

#[program]
pub mod nft_staking {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}
