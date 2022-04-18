use amm_anchor::PreInitialize;
use anchor_lang::prelude::*;

#[derive(Accounts, Clone)]
pub struct ProxyPreInitialize<'info> {
    /// CHECK: Safe
    pub amm_program: AccountInfo<'info>,
    /// CHECK: Safe
    #[account(
        mut, 
        seeds = [
            amm_program.key.as_ref(),
            serum_market.key.as_ref(),
            b"target_associated_seed"
        ], 
        bump,
        seeds::program = amm_program.key
    )]
    pub amm_target_orders: AccountInfo<'info>,
    /// CHECK: Safe
    #[account(
        mut, 
        seeds = [
            amm_program.key.as_ref(),
            serum_market.key.as_ref(),
            b"withdraw_associated_seed"
        ], 
        bump,
        seeds::program = amm_program.key
    )]
    pub pool_withdraw_queue: AccountInfo<'info>,
    /// CHECK: Safe
    #[account( 
        seeds = [b"amm authority"], 
        bump,
        seeds::program = amm_program.key
    )]
    pub amm_authority: AccountInfo<'info>,
    /// CHECK: Safe
    #[account(
        mut, 
        seeds = [
            amm_program.key.as_ref(),
            serum_market.key.as_ref(),
            b"lp_mint_associated_seed"
        ], 
        bump,
        seeds::program = amm_program.key
    )]
    pub lp_mint: AccountInfo<'info>,
    /// CHECK: Safe
    pub coin_mint: AccountInfo<'info>,
    /// CHECK: Safe
    pub pc_mint: AccountInfo<'info>,
    /// CHECK: Safe
    #[account(
        mut, 
        seeds = [
            amm_program.key.as_ref(),
            serum_market.key.as_ref(),
            b"coin_vault_associated_seed"
        ], 
        bump,
        seeds::program = amm_program.key
    )]
    pub pool_coin_token_account: AccountInfo<'info>,
    /// CHECK: Safe
    #[account(
        mut, 
        seeds = [
            amm_program.key.as_ref(),
            serum_market.key.as_ref(),
            b"pc_vault_associated_seed"
        ], 
        bump,
        seeds::program = amm_program.key
    )]
    pub pool_pc_token_account: AccountInfo<'info>,
    /// CHECK: Safe
    #[account(
        mut, 
        seeds = [
            amm_program.key.as_ref(),
            serum_market.key.as_ref(),
            b"temp_lp_token_associated_seed"
        ], 
        bump,
        seeds::program = amm_program.key
    )]
    pub pool_temp_lp_token_account: AccountInfo<'info>,
    /// CHECK: Safe
    pub serum_market: AccountInfo<'info>,
    pub user_wallet: Signer<'info>,
    /// CHECK: Safe
    #[account(address = spl_token::ID)]
    pub spl_token_program: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

impl<'a, 'b, 'c, 'info> From<&mut ProxyPreInitialize<'info>>
    for CpiContext<'a, 'b, 'c, 'info, PreInitialize<'info>>
{
    fn from(
        accounts: &mut ProxyPreInitialize<'info>,
    ) -> CpiContext<'a, 'b, 'c, 'info, PreInitialize<'info>> {
        let cpi_accounts = PreInitialize {
            amm_target_orders: accounts.amm_target_orders.clone(),
            pool_withdraw_queue: accounts.pool_withdraw_queue.clone(),
            amm_authority: accounts.amm_authority.clone(),
            lp_mint: accounts.lp_mint.clone(),
            coin_mint: accounts.coin_mint.clone(),
            pc_mint: accounts.pc_mint.clone(),
            pool_coin_token_account: accounts.pool_coin_token_account.clone(),
            pool_pc_token_account: accounts.pool_pc_token_account.clone(),
            pool_temp_lp_token_account: accounts.pool_temp_lp_token_account.clone(),
            serum_market: accounts.serum_market.clone(),
            user_wallet: accounts.user_wallet.to_account_info().clone(),
            spl_token_program: accounts.spl_token_program.clone(),
            system_program: accounts.system_program.clone(),
            rent: accounts.rent.clone(),
        };
        let cpi_program = accounts.amm_program.clone();
        CpiContext::new(cpi_program, cpi_accounts)
    }
}

pub fn pre_initialize(ctx: Context<ProxyPreInitialize>, nonce: u8) -> Result<()> {
    amm_anchor::pre_initialize(ctx.accounts.into(), nonce)
}
