use anchor_lang::prelude::*;
use anchor_spl::token::Token;
use solana_program::{instruction::Instruction, program::invoke_signed};

#[derive(Accounts)]
pub struct LiquidUnstake<'info> {
    ///CHECK:
    #[account(
        mut)]
    pub state: AccountInfo<'info>,
    ///CHECK:
    #[account(mut)]
    pub msol_mint: AccountInfo<'info>,
    ///CHECK:
    #[account(
        mut
)]
    pub liq_pool_sol_leg_pda: AccountInfo<'info>,
    ///CHECK:
    #[account(
        mut
    )]
    pub liq_pool_msol_leg: AccountInfo<'info>,

    /// CHECK: deserialized in code, must be the one in State (State has_one treasury_msol_account)
    #[account(mut)]
    pub treasury_msol_account: AccountInfo<'info>,
    ///CHECK: 
    #[account(
        mut,
    )]
    pub get_msol_from: AccountInfo<'info>,

    pub get_msol_from_authority: Signer<'info>, //burn_msol_from owner or delegate_authority
    ///CHECK: 
    #[account(mut)]
    pub transfer_sol_to: AccountInfo<'info>,
    ///CHECK:
    pub system_program: AccountInfo<'info>,
    ///CHECK:
    pub token_program: AccountInfo<'info>,
    /// CHECK: Marinade main program
    pub marinade_program: AccountInfo<'info>,
}
// Function to handle msol unstake by giving sol
pub fn handler(ctx: Context<LiquidUnstake>, msol_amount: u64) -> Result<()> {
    msg!("Marinade Liquid Unstake");

    let discriminator = [
        30,  30, 119, 240,
       191, 227,  12,  16
     ];

    let mut data = discriminator.to_vec();
    data.extend_from_slice(&msol_amount.to_le_bytes());

    msg!("Instruction created");

    // Call the marinade program to liquid unstake
    msg!("Instruction created");

    let accounts = vec![
        AccountMeta::new(*ctx.accounts.state.key, false),
        AccountMeta::new(*ctx.accounts.msol_mint.key, false),
        AccountMeta::new(*ctx.accounts.liq_pool_sol_leg_pda.key, false),
        AccountMeta::new(*ctx.accounts.liq_pool_msol_leg.key, false),
        AccountMeta::new(*ctx.accounts.treasury_msol_account.key, false),
        AccountMeta::new(*ctx.accounts.get_msol_from.key, false),
        AccountMeta::new(*ctx.accounts.get_msol_from_authority.key, true),
        AccountMeta::new(*ctx.accounts.transfer_sol_to.key, true),
        AccountMeta::new_readonly(solana_program::system_program::ID, false),
        AccountMeta::new_readonly(spl_token::ID, false)
    ];

    msg!("Accounts created");

    let ix = Instruction {
        program_id: ctx.accounts.marinade_program.key(), // MARINADE_PROGRAM_ID,
        accounts,
        data,
    };

    msg!("Instruction created 2");
    invoke_signed(
        &ix,
        &[
            ctx.accounts.state.clone(),
            ctx.accounts.msol_mint.clone(),
            ctx.accounts.liq_pool_sol_leg_pda.clone(),
            ctx.accounts.liq_pool_msol_leg.clone(),
            ctx.accounts.treasury_msol_account.clone(),
            ctx.accounts.get_msol_from.clone(),
            ctx.accounts.get_msol_from_authority.to_account_info(),
            ctx.accounts.transfer_sol_to.clone(),
            ctx.accounts.system_program.clone(),
            ctx.accounts.token_program.clone(),
            ctx.accounts.token_program.to_account_info(),
        ],
        &[]
    )?;

    msg!("Liquid Unstake instruction invoked");

    Ok(())
}
