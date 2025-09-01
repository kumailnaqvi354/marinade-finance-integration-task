use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    metadata::{ mpl_token_metadata, Metadata as Metaplex },
    token::{ Mint, SyncNative, Token, TokenAccount },
};
use spl_token::native_mint::ID as NATIVE_MINT;

use raydium_cp_swap::{ cpi::accounts::Initialize, program::RaydiumCpSwap, states::AmmConfig };
use crate::error::ErrorCode;
#[derive(Accounts)]
pub struct CreateTokenPool<'info> {
    #[account(
        constraint = token_0_mint.key() < token_1_mint.key(),
        mint::token_program = token_program
    )]
    pub token_0_mint: Account<'info, Mint>,

    #[account(mint::token_program = token_program)]
    pub token_1_mint: Account<'info, Mint>,

    #[account(mut)]
    pub payer: Signer<'info>,
    #[account(
        mut,
        associated_token::mint = token_0_mint,
        associated_token::authority = payer,
    )]
    pub user_token_0_account: Box<Account<'info, TokenAccount>>,
    #[account(
        mut,
        associated_token::mint = token_1_mint,
        associated_token::authority = payer
    )]
    pub user_token_1_account: Box<Account<'info, TokenAccount>>,
    pub rent: Sysvar<'info, Rent>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    #[account(address = mpl_token_metadata::ID)]
    pub token_metadata_program: Program<'info, Metaplex>,
    pub associated_token_program: Program<'info, AssociatedToken>,

    // RaydiumCpSwap program
    pub cp_swap_program: Program<'info, RaydiumCpSwap>,

    pub amm_config: Account<'info, AmmConfig>,

    ///CHECK:
    #[account(
        mut,
        seeds = [
            "pool".as_bytes(),
            amm_config.key().as_ref(),
            &token_0_mint.key().as_ref(),
            &token_1_mint.key().as_ref(),
        ],
        seeds::program = cp_swap_program,
        bump,
    )]
    pub pool_state: UncheckedAccount<'info>,
    /// CHECK: pool vault and lp mint authority
    #[account(
        seeds = ["vault_and_lp_mint_auth_seed".as_bytes()],
        seeds::program = cp_swap_program,
        bump
    )]
    pub authority: UncheckedAccount<'info>,

    ///CHECK:
    #[account(
        mut,
        seeds = [
           "pool_lp_mint".as_bytes(),
            pool_state.key().as_ref(),
        ],
        seeds::program = cp_swap_program,
        bump,
    )]
    pub lp_mint: UncheckedAccount<'info>,
    ///CHECK:
    #[account( 
        mut,
        seeds=  [
        payer.key().as_ref(),
        token_program.key().as_ref(),
        lp_mint.key().as_ref(),
      ],
      bump,
      seeds::program = associated_token_program
    )]
    pub user_lp_token_vault: UncheckedAccount<'info>,

    /// CHECK
    #[account(
            mut,
            seeds = [
                "pool_vault".as_bytes(),
                pool_state.key().as_ref(),
                token_0_mint.key().as_ref(),
                ],
            seeds::program = cp_swap_program,
            bump,
        )]
    pub token_0_vault: UncheckedAccount<'info>,

    /// CHECK:
    #[account(
    mut,
    seeds = [
        "pool_vault".as_bytes(),
        pool_state.key().as_ref(),
        token_1_mint.key().as_ref(),
        ],
    seeds::program = cp_swap_program,
    bump,
   )]
    pub token_1_vault: UncheckedAccount<'info>,

    /// create pool fee account
    #[account(
          mut,
          address= raydium_cp_swap::create_pool_fee_reveiver::id(),
      )]
    pub create_pool_fee: Box<Account<'info, TokenAccount>>,
    /// CHECK: an account to store oracle observations, init by cp-swap
    #[account(
        mut,
        seeds = [
            "observation".as_bytes(),
            pool_state.key().as_ref(),
        ],
        seeds::program = cp_swap_program,
        bump,
    )]
    pub observation_state: UncheckedAccount<'info>,
}

pub fn handler(
    mut ctx: Context<CreateTokenPool>,
    liq_grass_amount: u64,
    liq_sol_amount: u64
) -> Result<()> {
    let accounts = &mut ctx.accounts;

    let (liq_0_amount, liq_1_amount) = if accounts.token_0_mint.key() == NATIVE_MINT {
        (liq_sol_amount, liq_grass_amount)
    } else if accounts.token_1_mint.key() == NATIVE_MINT {
        (liq_grass_amount, liq_sol_amount)
    } else {
        return err!(ErrorCode::InvalidTokenPair);
    };

    let (
        token_0_mint,
        creator_token_0,
        token_0_vault,
        token_0_supply,
        token_1_mint,
        creator_token_1,
        token_1_vault,
        token_1_supply,
    ) = if accounts.token_0_mint.key() < accounts.token_1_mint.key() {
        (
            accounts.token_0_mint.to_account_info(),
            accounts.user_token_0_account.to_account_info(),
            accounts.token_0_vault.to_account_info(),
            liq_0_amount,
            accounts.token_1_mint.to_account_info(),
            accounts.user_token_1_account.to_account_info(),
            accounts.token_1_vault.to_account_info(),
            liq_1_amount,
        )
    } else {
        (
            accounts.token_1_mint.to_account_info(),
            accounts.user_token_1_account.to_account_info(),
            accounts.token_1_vault.to_account_info(),
            liq_1_amount,
            accounts.token_0_mint.to_account_info(),
            accounts.user_token_0_account.to_account_info(),
            accounts.token_0_vault.to_account_info(),
            liq_0_amount,
        )
    };

    let cpi_accounts = Initialize {
        creator: accounts.payer.to_account_info(),
        amm_config: accounts.amm_config.to_account_info(),
        authority: accounts.authority.to_account_info(),
        pool_state: accounts.pool_state.to_account_info(),
        token_0_mint,
        token_1_mint,
        lp_mint: accounts.lp_mint.to_account_info(),
        creator_token_0,
        creator_token_1,
        creator_lp_token: accounts.user_lp_token_vault.to_account_info(),
        token_0_vault,
        token_1_vault,
        create_pool_fee: accounts.create_pool_fee.to_account_info(),
        observation_state: accounts.observation_state.to_account_info(),
        token_program: accounts.token_program.to_account_info(),
        token_0_program: accounts.token_program.to_account_info(),
        token_1_program: accounts.token_program.to_account_info(),
        associated_token_program: accounts.associated_token_program.to_account_info(),
        system_program: accounts.system_program.to_account_info(),
        rent: accounts.rent.to_account_info(),
    };

    raydium_cp_swap::cpi::initialize(
        CpiContext::new(accounts.cp_swap_program.to_account_info(), cpi_accounts),
        token_0_supply,
        token_1_supply,
        0
    )?;
    msg!("<====POOL_CREATED_FOR_GRASS/SOL====>");

    Ok(())
}
