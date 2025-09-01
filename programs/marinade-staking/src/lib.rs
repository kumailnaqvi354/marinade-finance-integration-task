pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

pub use instructions::*;
pub use state::*;

declare_id!("B3hvpW8zM64Kb8Ud5eoGA2UBQgf4nZ18EEfbqZNXYD1b");

#[program]
pub mod marinadestaking {
    use super::*;

    pub fn marinade_deposit(ctx: Context<MarinadeDeposit>, amount: u64) -> Result<()> {
        marinade_deposit::handler(ctx, amount)
    }

    pub fn marinade_unstake(ctx: Context<LiquidUnstake>, msol_amount: u64) -> Result<()> {
        marinade_liquid_unstake::handler(ctx, msol_amount)
    }

}
