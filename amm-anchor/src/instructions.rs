//! Instruction builders and invokers for AMM instructions.

use crate::*;
use anchor_lang::{prelude::*, solana_program};
use raydium_contract_instructions::amm_instruction;

/// Creates and invokes a [raydium_contract_instructions::amm_instruction::initialize2] instruction.
///
/// # Arguments
///
/// See [raydium_contract_instructions::amm_instruction::InitializeInstruction2].
///
/// * `nonce` - The nonce used to generate the amm_authority.
/// * `open_time` - The effective time.
/// * `init_pc_amount` - The deposit pc amount transfer to pool.
/// * `init_coin_amount` - The deposit coin amount transfer to pool.
pub fn initialize<'a, 'b, 'c, 'info>(
    ctx: CpiContext<'a, 'b, 'c, 'info, Initialize2<'info>>,
    nonce: u8,
    open_time: u64,
    init_pc_amount: u64,
    init_coin_amount: u64,
) -> Result<()> {
    let ix = amm_instruction::initialize2(
        ctx.program.key,
        ctx.accounts.amm.key,
        ctx.accounts.amm_authority.key,
        ctx.accounts.amm_open_orders.key,
        ctx.accounts.amm_lp_mint.key,
        ctx.accounts.amm_coin_mint.key,
        ctx.accounts.amm_pc_mint.key,
        ctx.accounts.amm_coin_vault.key,
        ctx.accounts.amm_pc_vault.key,
        ctx.accounts.amm_target_orders.key,
        ctx.accounts.amm_config.key,
        ctx.accounts.create_fee_destination.key,
        ctx.accounts.market_program.key,
        ctx.accounts.market.key,
        ctx.accounts.user_wallet.key,
        ctx.accounts.user_token_coin.key,
        ctx.accounts.user_token_pc.key,
        ctx.accounts.user_token_lp.key,
        nonce,
        open_time,
        init_pc_amount,
        init_coin_amount,
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
        ctx.accounts.amm_lp_mint.key,
        ctx.accounts.amm_coin_vault.key,
        ctx.accounts.amm_pc_vault.key,
        ctx.accounts.market.key,
        ctx.accounts.market_event_queue.key,
        ctx.accounts.user_token_coin.key,
        ctx.accounts.user_token_pc.key,
        ctx.accounts.user_token_lp.key,
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
        ctx.accounts.amm_lp_mint.key,
        ctx.accounts.amm_coin_vault.key,
        ctx.accounts.amm_pc_vault.key,
        ctx.accounts.market_program.key,
        ctx.accounts.market.key,
        ctx.accounts.market_coin_vault.key,
        ctx.accounts.market_pc_vault.key,
        ctx.accounts.market_vault_signer.key,
        ctx.accounts.user_token_lp.key,
        ctx.accounts.user_token_coin.key,
        ctx.accounts.user_token_pc.key,
        ctx.accounts.user_owner.key,
        ctx.accounts.market_event_q.key,
        ctx.accounts.market_bids.key,
        ctx.accounts.market_asks.key,
        None,
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
        ctx.accounts.amm_coin_vault.key,
        ctx.accounts.amm_pc_vault.key,
        ctx.accounts.market_program.key,
        ctx.accounts.market.key,
        ctx.accounts.market_bids.key,
        ctx.accounts.market_asks.key,
        ctx.accounts.market_event_queue.key,
        ctx.accounts.market_coin_vault.key,
        ctx.accounts.market_pc_vault.key,
        ctx.accounts.market_vault_signer.key,
        ctx.accounts.user_token_source.key,
        ctx.accounts.user_token_destination.key,
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
        ctx.accounts.amm_coin_vault.key,
        ctx.accounts.amm_pc_vault.key,
        ctx.accounts.market_program.key,
        ctx.accounts.market.key,
        ctx.accounts.market_bids.key,
        ctx.accounts.market_asks.key,
        ctx.accounts.market_event_queue.key,
        ctx.accounts.market_coin_vault.key,
        ctx.accounts.market_pc_vault.key,
        ctx.accounts.market_vault_signer.key,
        ctx.accounts.user_token_source.key,
        ctx.accounts.user_token_destination.key,
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
