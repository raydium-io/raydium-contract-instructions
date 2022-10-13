//! Instruction builders and invokers for AMM instructions.

use crate::*;
use anchor_lang::{prelude::*, solana_program};
use raydium_contract_instructions::amm_instruction;

/// Creates and invokes a [raydium_contract_instructions::amm_instruction::pre_initialize] instruction.
///
/// # Arguments
///
/// See [raydium_contract_instructions::amm_instruction::PreInitializeInstruction].
///
/// * `nonce` - The nonce used to generate the amm_authority.
pub fn pre_initialize<'a, 'b, 'c, 'info>(
    ctx: CpiContext<'a, 'b, 'c, 'info, PreInitialize<'info>>,
    nonce: u8,
) -> Result<()> {
    let ix = amm_instruction::pre_initialize(
        ctx.program.key,
        ctx.accounts.amm_target_orders.key,
        ctx.accounts.pool_withdraw_queue.key,
        ctx.accounts.amm_authority.key,
        ctx.accounts.lp_mint.key,
        ctx.accounts.coin_mint.key,
        ctx.accounts.pc_mint.key,
        ctx.accounts.pool_coin_token_account.key,
        ctx.accounts.pool_pc_token_account.key,
        ctx.accounts.pool_temp_lp_token_account.key,
        ctx.accounts.serum_market.key,
        ctx.accounts.user_wallet.key,
        nonce,
    )?;

    solana_program::program::invoke_signed(
        &ix,
        &ToAccountInfos::to_account_infos(&ctx),
        ctx.signer_seeds,
    )?;
    Ok(())
}

/// Creates and invokes a [raydium_contract_instructions::amm_instruction::initialize] instruction.
///
/// # Arguments
///
/// See [raydium_contract_instructions::amm_instruction::InitializeInstruction].
///
/// * `nonce` - The nonce used to generate the amm_authority.
/// * `open_time` - The effective time.
pub fn initialize<'a, 'b, 'c, 'info>(
    ctx: CpiContext<'a, 'b, 'c, 'info, Initialize<'info>>,
    nonce: u8,
    open_time: u64,
) -> Result<()> {
    let ix = amm_instruction::initialize(
        ctx.program.key,
        ctx.accounts.amm.key,
        ctx.accounts.amm_authority.key,
        ctx.accounts.amm_open_orders.key,
        ctx.accounts.lp_mint.key,
        ctx.accounts.coin_mint.key,
        ctx.accounts.pc_mint.key,
        ctx.accounts.pool_coin_token_account.key,
        ctx.accounts.pool_pc_token_account.key,
        ctx.accounts.pool_withdraw_queue.key,
        ctx.accounts.pool_target_orders_account.key,
        ctx.accounts.pool_lp_token_account.key,
        ctx.accounts.pool_temp_lp_token_account.key,
        ctx.accounts.serum_program.key,
        ctx.accounts.serum_market.key,
        ctx.accounts.user_wallet.key,
        nonce,
        open_time,
    )?;
    solana_program::program::invoke_signed(
        &ix,
        &ToAccountInfos::to_account_infos(&ctx),
        ctx.signer_seeds,
    )?;
    Ok(())
}

/// Creates and invokes a [raydium_contract_instructions::amm_instruction::deposit] instruction.
///
/// # Arguments
///
/// See [raydium_contract_instructions::amm_instruction::DepositInstruction].
///
/// * `max_coin_amount` - Pool token amount to transfer. token_a and token_b amount are set by the current exchange rate and size of the pool
/// * `max_pc_amount` - The effective time.
/// * `base_side` - .
pub fn deposit<'a, 'b, 'c, 'info>(
    ctx: CpiContext<'a, 'b, 'c, 'info, Deposit<'info>>,
    max_coin_amount: u64,
    max_pc_amount: u64,
    base_side: u64,
) -> Result<()> {
    let ix = amm_instruction::deposit(
        ctx.program.key,
        ctx.accounts.amm.key,
        ctx.accounts.amm_authority.key,
        ctx.accounts.amm_open_orders.key,
        ctx.accounts.amm_target_orders.key,
        ctx.accounts.lp_mint.key,
        ctx.accounts.pool_coin_token_account.key,
        ctx.accounts.pool_pc_token_account.key,
        ctx.accounts.serum_market.key,
        ctx.accounts.serum_event_queue.key,
        ctx.accounts.user_coin_token_account.key,
        ctx.accounts.user_pc_token_account.key,
        ctx.accounts.user_lp_token_account.key,
        ctx.accounts.user_owner.key,
        max_coin_amount,
        max_pc_amount,
        base_side,
    )?;
    solana_program::program::invoke_signed(
        &ix,
        &ToAccountInfos::to_account_infos(&ctx),
        ctx.signer_seeds,
    )?;
    Ok(())
}

/// Creates and invokes a [raydium_contract_instructions::amm_instruction::withdraw] instruction.
///
/// # Arguments
///
/// See [raydium_contract_instructions::amm_instruction::WithdrawInstruction].
///
/// * `amount` - Pool token amount to transfer. token_a and token_b amount are set by
///             the current exchange rate and size of the pool.

pub fn withdraw<'a, 'b, 'c, 'info>(
    ctx: CpiContext<'a, 'b, 'c, 'info, Withdraw<'info>>,
    amount: u64,
) -> Result<()> {
    let ix = amm_instruction::withdraw(
        ctx.program.key,
        ctx.accounts.amm.key,
        ctx.accounts.amm_authority.key,
        ctx.accounts.amm_open_orders.key,
        ctx.accounts.amm_target_orders.key,
        ctx.accounts.lp_mint.key,
        ctx.accounts.pool_coin_token_account.key,
        ctx.accounts.pool_pc_token_account.key,
        ctx.accounts.pool_withdraw_queue.key,
        ctx.accounts.pool_temp_lp_token_account.key,
        ctx.accounts.serum_program.key,
        ctx.accounts.serum_market.key,
        ctx.accounts.serum_coin_vault_account.key,
        ctx.accounts.serum_pc_vault_account.key,
        ctx.accounts.serum_vault_signer.key,
        ctx.accounts.user_lp_token_account.key,
        ctx.accounts.user_coin_token_account.key,
        ctx.accounts.user_pc_token_account.key,
        ctx.accounts.user_owner.key,
        ctx.accounts.serum_event_q.key,
        ctx.accounts.serum_bids.key,
        ctx.accounts.serum_asks.key,
        amount,
    )?;
    solana_program::program::invoke_signed(
        &ix,
        &ToAccountInfos::to_account_infos(&ctx),
        ctx.signer_seeds,
    )?;
    Ok(())
}

/// Creates and invokes a [raydium_contract_instructions::amm_instruction::swap_base_in] instruction.
///
/// # Arguments
///
/// See [raydium_contract_instructions::amm_instruction::SwapInstructionBaseIn].
///
/// * `amount_in` - SOURCE amount to transfer, output to DESTINATION is based on the exchange rate.
/// * `minimum_amount_out` - Minimum amount of DESTINATION token to output, prevents excessive slippage.
pub fn swap_base_in<'a, 'b, 'c, 'info>(
    ctx: CpiContext<'a, 'b, 'c, 'info, SwapBaseIn<'info>>,
    amount_in: u64,
    minimum_amount_out: u64,
) -> Result<()> {
    let ix = amm_instruction::swap_base_in(
        ctx.program.key,
        ctx.accounts.amm.key,
        ctx.accounts.amm_authority.key,
        ctx.accounts.amm_open_orders.key,
        ctx.accounts.amm_target_orders.key,
        ctx.accounts.pool_coin_token_account.key,
        ctx.accounts.pool_pc_token_account.key,
        ctx.accounts.serum_program.key,
        ctx.accounts.serum_market.key,
        ctx.accounts.serum_bids.key,
        ctx.accounts.serum_asks.key,
        ctx.accounts.serum_event_queue.key,
        ctx.accounts.serum_coin_vault_account.key,
        ctx.accounts.serum_pc_vault_account.key,
        ctx.accounts.serum_vault_signer.key,
        ctx.accounts.user_source_token_account.key,
        ctx.accounts.user_destination_token_account.key,
        ctx.accounts.user_source_owner.key,
        amount_in,
        minimum_amount_out,
    )?;
    solana_program::program::invoke_signed(
        &ix,
        &ToAccountInfos::to_account_infos(&ctx),
        ctx.signer_seeds,
    )?;
    Ok(())
}

/// Creates and invokes a [raydium_contract_instructions::amm_instruction::swap_base_out] instruction.
///
/// # Arguments
///
/// See [raydium_contract_instructions::amm_instruction::SwapInstructionBaseOut].
///
/// * `max_amount_in` - SOURCE amount to transfer, output to DESTINATION is based on the exchange rate.
/// * `amount_out` - Minimum amount of DESTINATION token to output, prevents excessive slippage
pub fn swap_base_out<'a, 'b, 'c, 'info>(
    ctx: CpiContext<'a, 'b, 'c, 'info, SwapBaseOut<'info>>,
    max_amount_in: u64,
    amount_out: u64,
) -> Result<()> {
    let ix = amm_instruction::swap_base_out(
        ctx.program.key,
        ctx.accounts.amm.key,
        ctx.accounts.amm_authority.key,
        ctx.accounts.amm_open_orders.key,
        ctx.accounts.amm_target_orders.key,
        ctx.accounts.pool_coin_token_account.key,
        ctx.accounts.pool_pc_token_account.key,
        ctx.accounts.serum_program.key,
        ctx.accounts.serum_market.key,
        ctx.accounts.serum_bids.key,
        ctx.accounts.serum_asks.key,
        ctx.accounts.serum_event_queue.key,
        ctx.accounts.serum_coin_vault_account.key,
        ctx.accounts.serum_pc_vault_account.key,
        ctx.accounts.serum_vault_signer.key,
        ctx.accounts.user_source_token_account.key,
        ctx.accounts.user_destination_token_account.key,
        ctx.accounts.user_source_owner.key,
        max_amount_in,
        amount_out,
    )?;
    solana_program::program::invoke_signed(
        &ix,
        &ToAccountInfos::to_account_infos(&ctx),
        ctx.signer_seeds,
    )?;
    Ok(())
}
