use amm_anchor::Withdraw;
use anchor_lang::prelude::*;

#[derive(Accounts, Clone)]
pub struct ProxyWithdraw<'info> {
    /// CHECK: Safe
    pub amm_program: AccountInfo<'info>,
    /// CHECK: Safe
    #[account(mut)]
    pub amm: AccountInfo<'info>,
    /// CHECK: Safe
    pub amm_authority: AccountInfo<'info>,
    /// CHECK: Safe
    #[account(mut)]
    pub amm_open_orders: AccountInfo<'info>,
    /// CHECK: Safe
    #[account(mut)]
    pub amm_target_orders: AccountInfo<'info>,
    /// CHECK: Safe
    #[account(mut)]
    pub lp_mint: AccountInfo<'info>,
    /// CHECK: Safe
    #[account(mut)]
    pub pool_coin_token_account: AccountInfo<'info>,
    /// CHECK: Safe
    #[account(mut)]
    pub pool_pc_token_account: AccountInfo<'info>,
    /// CHECK: Safe
    #[account(mut)]
    pub pool_withdraw_queue: AccountInfo<'info>,
    /// CHECK: Safe
    #[account(mut)]
    pub pool_temp_lp_token_account: AccountInfo<'info>,
    /// CHECK: Safe
    pub serum_program: AccountInfo<'info>,
    /// CHECK: Safe
    #[account(mut)]
    pub serum_market: AccountInfo<'info>,
    /// CHECK: Safe
    #[account(mut)]
    pub serum_coin_vault_account: AccountInfo<'info>,
    /// CHECK: Safe
    #[account(mut)]
    pub serum_pc_vault_account: AccountInfo<'info>,
    /// CHECK: Safe
    pub serum_vault_signer: AccountInfo<'info>,
    /// CHECK: Safe
    #[account(mut)]
    pub user_lp_token_account: AccountInfo<'info>,
    /// CHECK: Safe
    #[account(mut)]
    pub user_coin_token_account: AccountInfo<'info>,
    /// CHECK: Safe
    #[account(mut)]
    pub user_pc_token_account: AccountInfo<'info>,
    pub user_owner: Signer<'info>,
    /// CHECK: Safe
    #[account(mut)]
    pub serum_event_q: AccountInfo<'info>,
    /// CHECK: Safe
    #[account(mut)]
    pub serum_bids: AccountInfo<'info>,
    /// CHECK: Safe
    #[account(mut)]
    pub serum_asks: AccountInfo<'info>,
    /// CHECK: Safe
    #[account(address = spl_token::ID)]
    pub spl_token_program: AccountInfo<'info>,
}

impl<'a, 'b, 'c, 'info> From<&mut ProxyWithdraw<'info>>
    for CpiContext<'a, 'b, 'c, 'info, Withdraw<'info>>
{
    fn from(accounts: &mut ProxyWithdraw<'info>) -> CpiContext<'a, 'b, 'c, 'info, Withdraw<'info>> {
        let cpi_accounts = Withdraw {
            amm: accounts.amm.clone(),
            amm_authority: accounts.amm_authority.clone(),
            amm_open_orders: accounts.amm_open_orders.clone(),
            amm_target_orders: accounts.amm_target_orders.clone(),
            lp_mint: accounts.lp_mint.clone(),
            pool_coin_token_account: accounts.pool_coin_token_account.clone(),
            pool_pc_token_account: accounts.pool_pc_token_account.clone(),
            pool_withdraw_queue: accounts.pool_withdraw_queue.clone(),
            pool_temp_lp_token_account: accounts.pool_temp_lp_token_account.clone(),
            serum_program: accounts.serum_program.clone(),
            serum_market: accounts.serum_market.clone(),
            serum_coin_vault_account: accounts.serum_coin_vault_account.clone(),
            serum_pc_vault_account: accounts.serum_pc_vault_account.clone(),
            serum_vault_signer: accounts.serum_vault_signer.clone(),
            user_lp_token_account: accounts.user_lp_token_account.clone(),
            user_coin_token_account: accounts.user_coin_token_account.clone(),
            user_pc_token_account: accounts.user_pc_token_account.clone(),
            user_owner: accounts.user_owner.to_account_info().clone(),
            spl_token_program: accounts.spl_token_program.clone(),
            serum_event_q: accounts.serum_event_q.clone(),
            serum_bids: accounts.serum_bids.clone(),
            serum_asks: accounts.serum_asks.clone(),
        };
        let cpi_program = accounts.amm_program.clone();
        CpiContext::new(cpi_program, cpi_accounts)
    }
}

/// withdraw instruction
pub fn withdraw(ctx: Context<ProxyWithdraw>, amount: u64) -> Result<()> {
    amm_anchor::withdraw(ctx.accounts.into(), amount)
}
