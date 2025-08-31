pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

pub use instructions::*;
pub use state::*;

declare_id!("GpXBBAHsQegCigS26qWS6m7PbiycAGggrjtnbNHTyeb8");

#[program]
pub mod drox_task_marinade_staking {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }

    // Wire up instructions
    pub fn create_pool(ctx: Context<CreatePool>, args: CreatePoolArgs) -> Result<()> {
        instructions::create_pool::handler(ctx, args)
    }

    pub fn marinade_deposit(ctx: Context<MarinadeDeposit>, lamports: u64) -> Result<()> {
        instructions::marinade_deposit::handler(ctx, lamports)
    }

    pub fn marinade_liquid_unstake(ctx: Context<MarinadeLiquidUnstake>, msol_amount: u64) -> Result<()> {
        instructions::marinade_liquid_unstake::handler(ctx, msol_amount)
    }

    pub fn swap_grass_for_sol(ctx: Context<SwapGrassForSol>, args: SwapArgs) -> Result<()> {
        instructions::swap_grass_for_sol::handler(ctx, args)
    }
}

// Base context
#[derive(Accounts)]
pub struct Initialize {}
