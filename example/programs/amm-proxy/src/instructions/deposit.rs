use amm_anchor::Deposit;
use anchor_lang::prelude::*;
use anchor_spl::token::Token;
#[derive(Accounts, Clone)]
pub struct ProxyDeposit<'info> {
    /// CHECK: Safe
    pub amm_program: UncheckedAccount<'info>,
    /// CHECK: Safe. Amm Account
    #[account(mut)]
    pub amm: UncheckedAccount<'info>,
    /// CHECK: Safe. Amm authority, a PDA create with seed = [b"amm authority"]
    #[account()]
    pub amm_authority: UncheckedAccount<'info>,
    /// CHECK: Safe. AMM open_orders Account.
    #[account()]
    pub amm_open_orders: UncheckedAccount<'info>,
    /// CHECK: Safe. AMM target orders account. To store plan orders infomations.
    #[account(mut)]
    pub amm_target_orders: UncheckedAccount<'info>,
    /// CHECK: Safe. LP mint account. Must be empty, owned by $authority.
    #[account(mut)]
    pub amm_lp_mint: UncheckedAccount<'info>,
    /// CHECK: Safe. amm_coin_vault account, $authority can transfer amount.
    #[account(mut)]
    pub amm_coin_vault: UncheckedAccount<'info>,
    /// CHECK: Safe. amm_pc_vault account, $authority can transfer amount.
    #[account(mut)]
    pub amm_pc_vault: UncheckedAccount<'info>,
    /// CHECK: Safe. OpenBook market account, OpenBook program is the owner.
    pub market: UncheckedAccount<'info>,
    /// CHECK: Safe. OpenBook market event queue account, OpenBook program is the owner.
    pub market_event_queue: UncheckedAccount<'info>,
    /// CHECK: Safe. User token coin to deposit into.
    #[account(mut)]
    pub user_token_coin: UncheckedAccount<'info>,
    /// CHECK: Safe. User token pc to deposit into.
    #[account(mut)]
    pub user_token_pc: UncheckedAccount<'info>,
    /// CHECK: Safe. User lp token, to deposit the generated tokens, user is the owner
    #[account(mut)]
    pub user_token_lp: UncheckedAccount<'info>,
    /// CHECK: Safe. User wallet account
    #[account(mut)]
    pub user_owner: Signer<'info>,
    /// CHECK: Safe. The spl token program
    pub token_program: Program<'info, Token>,
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
            amm_lp_mint: accounts.amm_lp_mint.clone(),
            amm_coin_vault: accounts.amm_coin_vault.clone(),
            amm_pc_vault: accounts.amm_pc_vault.clone(),
            market: accounts.market.clone(),
            market_event_queue: accounts.market_event_queue.clone(),
            user_token_coin: accounts.user_token_coin.clone(),
            user_token_pc: accounts.user_token_pc.clone(),
            user_token_lp: accounts.user_token_lp.clone(),
            user_owner: accounts.user_owner.clone(),
            token_program: accounts.token_program.clone(),
        };
        let cpi_program = accounts.amm_program.to_account_info();
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
