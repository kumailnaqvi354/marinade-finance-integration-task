use anchor_lang::prelude::*;

declare_id!("GpXBBAHsQegCigS26qWS6m7PbiycAGggrjtnbNHTyeb8");

#[program]
pub mod drox_task_marinade_staking {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
