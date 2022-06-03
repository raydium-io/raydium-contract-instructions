use anchor_lang::prelude::*;
use instructions::*;

pub mod instructions;

declare_id!("B584oELyjhQAJdR6UVyb7XUHpbTLwrH6oLbjnh4QpjhN");

#[program]
pub mod amm_proxy {
    use super::*;

    /// Pre initiazlize a swap pool
    pub fn proxy_pre_initialize(ctx: Context<ProxyPreInitialize>, nonce: u8) -> Result<()> {
        instructions::pre_initialize(ctx, nonce)
    }

    /// Initiazlize a swap pool
    pub fn proxy_initialize(
        ctx: Context<ProxyInitialize>,
        nonce: u8,
        open_time: u64,
    ) -> Result<()> {
        instructions::initialize(ctx, nonce, open_time)
    }

    /// deposit instruction
    pub fn proxy_deposit(
        ctx: Context<ProxyDeposit>,
        max_coin_amount: u64,
        max_pc_amount: u64,
        base_side: u64,
    ) -> Result<()> {
        instructions::deposit(ctx, max_coin_amount, max_pc_amount, base_side)
    }

    /// withdraw instruction
    pub fn proxy_withdraw(ctx: Context<ProxyWithdraw>, amount: u64) -> Result<()> {
        instructions::withdraw(ctx, amount)
    }

    /// swap_base_in instruction
    pub fn proxy_swap_base_in(
        ctx: Context<ProxySwapBaseIn>,
        amount_in: u64,
        minimum_amount_out: u64,
    ) -> Result<()> {
        instructions::swap_base_in(ctx, amount_in, minimum_amount_out)
    }

    /// swap_base_out instruction
    pub fn proxy_swap_base_out(
        ctx: Context<ProxySwapBaseOut>,
        max_amount_in: u64,
        amount_out: u64,
    ) -> Result<()> {
        instructions::swap_base_out(ctx, max_amount_in, amount_out)
    }
}
