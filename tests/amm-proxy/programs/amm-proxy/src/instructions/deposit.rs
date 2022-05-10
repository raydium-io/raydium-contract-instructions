use amm_anchor::Deposit;
use anchor_lang::prelude::*;

#[derive(Accounts, Clone)]
pub struct ProxyDeposit<'info> {
    /// CHECK: Safe
    pub amm_program: AccountInfo<'info>,
    /// CHECK: Safe
    #[account(mut)]
    pub amm: AccountInfo<'info>,
    /// CHECK: Safe
    pub amm_authority: AccountInfo<'info>,
    /// CHECK: Safe
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
    pub serum_market: AccountInfo<'info>,
    /// CHECK: Safe
    #[account(mut)]
    pub user_coin_token_account: AccountInfo<'info>,
    /// CHECK: Safe
    #[account(mut)]
    pub user_pc_token_account: AccountInfo<'info>,
    /// CHECK: Safe
    #[account(mut)]
    pub user_lp_token_account: AccountInfo<'info>,
    pub user_owner: Signer<'info>,
    /// CHECK: Safe
    #[account(address = spl_token::ID)]
    pub spl_token_program: AccountInfo<'info>,
}

impl<'a, 'b, 'c, 'info> From<&mut ProxyDeposit<'info>>
    for CpiContext<'a, 'b, 'c, 'info, Deposit<'info>>
{
    fn from(accounts: &mut ProxyDeposit<'info>) -> CpiContext<'a, 'b, 'c, 'info, Deposit<'info>> {
        let cpi_accounts = Deposit {
            amm: accounts.amm.clone(),
            amm_authority: accounts.amm_authority.clone(),
            amm_open_orders: accounts.amm_open_orders.clone(),
            amm_target_orders: accounts.amm_target_orders.clone(),
            lp_mint: accounts.lp_mint.clone(),
            pool_coin_token_account: accounts.pool_coin_token_account.clone(),
            pool_pc_token_account: accounts.pool_pc_token_account.clone(),
            serum_market: accounts.serum_market.clone(),
            user_coin_token_account: accounts.user_coin_token_account.clone(),
            user_pc_token_account: accounts.user_pc_token_account.clone(),
            user_lp_token_account: accounts.user_lp_token_account.clone(),
            user_owner: accounts.user_owner.to_account_info().clone(),
            spl_token_program: accounts.spl_token_program.clone(),
        };
        let cpi_program = accounts.amm_program.clone();
        CpiContext::new(cpi_program, cpi_accounts)
    }
}

pub fn deposit(
    ctx: Context<ProxyDeposit>,
    max_coin_amount: u64,
    max_pc_amount: u64,
    base_side: u64,
) -> Result<()> {
    amm_anchor::deposit(
        ctx.accounts.into(),
        max_coin_amount,
        max_pc_amount,
        base_side,
    )
}
