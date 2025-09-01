use anchor_lang::prelude::*;
use solana_program::{
    instruction::{ AccountMeta, Instruction },
    program::invoke_signed,
};
#[derive(Accounts)]
pub struct MarinadeDeposit<'info> {
    /// CHECK: Not accessed directly
    #[account(mut)]
    pub state: AccountInfo<'info>,
    /// CHECK: Not accessed directly
    #[account(mut)]
    pub msol_mint: AccountInfo<'info>,
    /// CHECK: Not accessed directly
    #[account(mut)]
    pub liq_pool_sol_leg_pda: AccountInfo<'info>,
    /// CHECK: Not accessed directly
    #[account(mut)]
    pub liq_pool_msol_leg: AccountInfo<'info>,
    /// CHECK: Not accessed directly
    // #[account(mut)]
    pub liq_pool_msol_leg_authority: AccountInfo<'info>,
    /// CHECK: Not accessed directly
    #[account(mut)]
    pub reserve_pda: AccountInfo<'info>,
    #[account(mut)]
    pub transfer_from: Signer<'info>,
    /// CHECK: Not accessed directly
    #[account(mut)]
    pub mint_to: AccountInfo<'info>,
    /// CHECK: Not accessed directly
    // #[account(mut)]
    pub mint_authority: AccountInfo<'info>,
    /// CHECK: System program
    pub system_program: AccountInfo<'info>,
    /// CHECK: Token program
    pub token_program: AccountInfo<'info>,
    /// CHECK: Marinade main program
    pub marinade_program: AccountInfo<'info>,
}


pub fn handler(ctx: Context<MarinadeDeposit>, lamports: u64) -> Result<()> {
    msg!("Marinade deposit");
    
    let discriminator = [242, 35, 198, 137, 82, 225, 242, 182];

    let mut data = discriminator.to_vec();
    // let mut data = vec![6];
    data.extend_from_slice(&lamports.to_le_bytes());

    msg!("Instruction created");

    let accounts = vec![
        AccountMeta::new(*ctx.accounts.state.key, false),
        AccountMeta::new(*ctx.accounts.msol_mint.key, false),
        AccountMeta::new(*ctx.accounts.liq_pool_sol_leg_pda.key, false),
        AccountMeta::new(*ctx.accounts.liq_pool_msol_leg.key, false),
        AccountMeta::new_readonly(*ctx.accounts.liq_pool_msol_leg_authority.key, false),
        AccountMeta::new(*ctx.accounts.reserve_pda.key, false),
        AccountMeta::new(ctx.accounts.transfer_from.key(), true),
        AccountMeta::new(*ctx.accounts.mint_to.key, false),
        AccountMeta::new_readonly(*ctx.accounts.mint_authority.key, false),
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
            ctx.accounts.liq_pool_msol_leg_authority.clone(),
            ctx.accounts.reserve_pda.clone(),
            ctx.accounts.transfer_from.to_account_info(),
            ctx.accounts.mint_to.clone(),
            ctx.accounts.mint_authority.clone(),
            ctx.accounts.system_program.clone(),
            ctx.accounts.token_program.clone(),
            ctx.accounts.token_program.to_account_info(),
        ],
        &[]
    )?;

    msg!("Instruction invoked");

    Ok(())
}
