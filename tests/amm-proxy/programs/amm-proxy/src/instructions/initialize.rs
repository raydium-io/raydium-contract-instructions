use amm_anchor::Initialize;
use anchor_lang::prelude::*;

#[derive(Accounts, Clone)]
pub struct ProxyInitialize<'info> {
    /// CHECK: Safe
    pub amm_program: AccountInfo<'info>,
    /// CHECK: Safe
    #[account(
        mut, 
        seeds = [
            amm_program.key.as_ref(),
            serum_market.key.as_ref(),
            b"amm_associated_seed"], 
        bump,
        seeds::program = amm_program.key
    )]
    pub amm: AccountInfo<'info>,
    /// CHECK: Safe
    #[account(
        mut, 
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
            b"open_order_associated_seed"], 
        bump,
        seeds::program = amm_program.key
    )]
    pub amm_open_orders: AccountInfo<'info>,
    /// CHECK: Safe
    #[account(mut)]
    pub lp_mint: AccountInfo<'info>,
    /// CHECK: Safe
    pub coin_mint: AccountInfo<'info>,
    /// CHECK: Safe
    pub pc_mint: AccountInfo<'info>,
    /// CHECK: Safe
    pub pool_coin_token_account: AccountInfo<'info>,
    /// CHECK: Safe
    pub pool_pc_token_account: AccountInfo<'info>,
    /// CHECK: Safe
    #[account(mut)]
    pub pool_withdraw_queue: AccountInfo<'info>,
    /// CHECK: Safe
    #[account(mut)]
    pub pool_target_orders_account: AccountInfo<'info>,
    /// CHECK: Safe
    #[account(mut)]
    pub pool_lp_token_account: AccountInfo<'info>,
    /// CHECK: Safe
    pub pool_temp_lp_token_account: AccountInfo<'info>,
    /// CHECK: Safe
    pub serum_program: AccountInfo<'info>,
    /// CHECK: Safe
    pub serum_market: AccountInfo<'info>,
    /// CHECK: Safe
    pub user_wallet: AccountInfo<'info>,
    /// CHECK: Safe
    #[account(address = spl_token::ID)]
    pub spl_token_program: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

impl<'a, 'b, 'c, 'info> From<&mut ProxyInitialize<'info>>
    for CpiContext<'a, 'b, 'c, 'info, Initialize<'info>>
{
    fn from(
        accounts: &mut ProxyInitialize<'info>,
    ) -> CpiContext<'a, 'b, 'c, 'info, Initialize<'info>> {
        let cpi_accounts = Initialize {
            amm: accounts.amm.clone(),
            amm_authority: accounts.amm_authority.clone(),
            amm_open_orders: accounts.amm_open_orders.clone(),
            lp_mint: accounts.lp_mint.clone(),
            coin_mint: accounts.coin_mint.clone(),
            pc_mint: accounts.pc_mint.clone(),
            pool_coin_token_account: accounts.pool_coin_token_account.clone(),
            pool_pc_token_account: accounts.pool_pc_token_account.clone(),
            pool_withdraw_queue: accounts.pool_withdraw_queue.clone(),
            pool_target_orders_account: accounts.pool_target_orders_account.clone(),
            pool_lp_token_account: accounts.pool_lp_token_account.clone(),
            pool_temp_lp_token_account: accounts.pool_temp_lp_token_account.clone(),
            serum_program: accounts.serum_program.clone(),
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

/// Initiazlize a swap pool
pub fn initialize(ctx: Context<ProxyInitialize>, nonce: u8, open_time: u64) -> Result<()> {
    amm_anchor::initialize(ctx.accounts.into(), nonce, open_time)
}
