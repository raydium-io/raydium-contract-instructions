//! Instruction builders and invokers for AMM instructions.

use crate::*;
use anchor_lang::{prelude::*, solana_program};
use raydium_contract_instructions::stable_instruction;

/// Creates and invokes a [raydium_contract_instructions::stable_instruction::pre_initialize] instruction.
///
/// # Arguments
///
/// See [raydium_contract_instructions::stable_instruction::PreInitializeInstruction].
///
/// * `nonce` - The nonce used to generate the amm_authority.
pub fn pre_initialize<'a, 'b, 'c, 'info>(
    ctx: CpiContext<'a, 'b, 'c, 'info, PreInitialize<'info>>,
    nonce: u8,
) -> Result<()> {
    let ix = stable_instruction::pre_initialize(
        ctx.program.key,
        ctx.accounts.amm_target_orders.key,
        ctx.accounts.amm_authority.key,
        ctx.accounts.amm_lp_mint.key,
        ctx.accounts.coin_mint.key,
        ctx.accounts.pc_mint.key,
        ctx.accounts.amm_coin_vault.key,
        ctx.accounts.amm_pc_vault.key,
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

/// Creates and invokes a [raydium_contract_instructions::stable_instruction::initialize] instruction.
///
/// # Arguments
///
/// See [raydium_contract_instructions::stable_instruction::InitializeInstruction].
///
/// * `nonce` - The nonce used to generate the amm_authority.
/// * `open_time` - The effective time.
pub fn initialize<'a, 'b, 'c, 'info>(
    ctx: CpiContext<'a, 'b, 'c, 'info, Initialize<'info>>,
    nonce: u8,
    open_time: u64,
) -> Result<()> {
    let ix = stable_instruction::initialize(
        ctx.program.key,
        ctx.accounts.amm.key,
        ctx.accounts.amm_authority.key,
        ctx.accounts.amm_open_orders.key,
        ctx.accounts.amm_lp_mint.key,
        ctx.accounts.coin_mint.key,
        ctx.accounts.pc_mint.key,
        ctx.accounts.amm_coin_vault.key,
        ctx.accounts.amm_pc_vault.key,
        ctx.accounts.amm_target_orders.key,
        ctx.accounts.model_data_account.key,
        ctx.accounts.serum_program.key,
        ctx.accounts.serum_market.key,
        ctx.accounts.user_dest_lp_token.key,
        ctx.accounts.user_wallet.key,
        None,
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

/// Creates and invokes a [raydium_contract_instructions::stable_instruction::deposit] instruction.
///
/// # Arguments
///
/// See [raydium_contract_instructions::stable_instruction::DepositInstruction].
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
    let ix = stable_instruction::deposit(
        ctx.program.key,
        ctx.accounts.amm.key,
        ctx.accounts.amm_authority.key,
        ctx.accounts.amm_open_orders.key,
        ctx.accounts.amm_target_orders.key,
        ctx.accounts.amm_lp_mint.key,
        ctx.accounts.amm_coin_vault.key,
        ctx.accounts.amm_pc_vault.key,
        ctx.accounts.model_data_account.key,
        ctx.accounts.serum_market.key,
        ctx.accounts.user_source_coin_token.key,
        ctx.accounts.user_source_pc_token.key,
        ctx.accounts.user_dest_lp_token.key,
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

/// Creates and invokes a [raydium_contract_instructions::stable_instruction::withdraw] instruction.
///
/// # Arguments
///
/// See [raydium_contract_instructions::stable_instruction::WithdrawInstruction].
///
/// * `amount` - Pool token amount to transfer. token_a and token_b amount are set by
///             the current exchange rate and size of the pool.

pub fn withdraw<'a, 'b, 'c, 'info>(
    ctx: CpiContext<'a, 'b, 'c, 'info, Withdraw<'info>>,
    amount: u64,
) -> Result<()> {
    let referrer_pc_account = ctx.remaining_accounts.get(0);
    let serum_event_q = ctx.remaining_accounts.get(1);
    let serum_bids = ctx.remaining_accounts.get(2);
    let serum_asks = ctx.remaining_accounts.get(3);
    let ix = stable_instruction::withdraw(
        ctx.program.key,
        ctx.accounts.amm.key,
        ctx.accounts.amm_authority.key,
        ctx.accounts.amm_open_orders.key,
        ctx.accounts.amm_target_orders.key,
        ctx.accounts.amm_lp_mint.key,
        ctx.accounts.amm_coin_vault.key,
        ctx.accounts.amm_pc_vault.key,
        ctx.accounts.model_data_account.key,
        ctx.accounts.serum_program.key,
        ctx.accounts.serum_market.key,
        ctx.accounts.serum_coin_vault.key,
        ctx.accounts.serum_pc_vault.key,
        ctx.accounts.serum_vault_signer.key,
        ctx.accounts.user_source_lp_token.key,
        ctx.accounts.user_dest_coin_token.key,
        ctx.accounts.user_dest_pc_token.key,
        ctx.accounts.user_owner.key,
        referrer_pc_account.map(|info| info.key),
        serum_event_q.map(|info| info.key),
        serum_bids.map(|info| info.key),
        serum_asks.map(|info| info.key),
        amount,
    )?;
    solana_program::program::invoke_signed(
        &ix,
        &ToAccountInfos::to_account_infos(&ctx),
        ctx.signer_seeds,
    )?;
    Ok(())
}

/// Creates and invokes a [raydium_contract_instructions::stable_instruction::swap_base_in] instruction.
///
/// # Arguments
///
/// See [raydium_contract_instructions::stable_instruction::SwapInstructionBaseIn].
///
/// * `amount_in` - SOURCE amount to transfer, output to DESTINATION is based on the exchange rate.
/// * `minimum_amount_out` - Minimum amount of DESTINATION token to output, prevents excessive slippage.
pub fn swap_base_in<'a, 'b, 'c, 'info>(
    ctx: CpiContext<'a, 'b, 'c, 'info, SwapBaseIn<'info>>,
    amount_in: u64,
    minimum_amount_out: u64,
) -> Result<()> {
    let ix = stable_instruction::swap_base_in(
        ctx.program.key,
        ctx.accounts.amm.key,
        ctx.accounts.amm_authority.key,
        ctx.accounts.amm_open_orders.key,
        ctx.accounts.amm_coin_vault.key,
        ctx.accounts.amm_pc_vault.key,
        ctx.accounts.model_data_account.key,
        ctx.accounts.serum_program.key,
        ctx.accounts.serum_market.key,
        ctx.accounts.serum_bids.key,
        ctx.accounts.serum_asks.key,
        ctx.accounts.serum_event_queue.key,
        ctx.accounts.serum_coin_vault.key,
        ctx.accounts.serum_pc_vault.key,
        ctx.accounts.serum_vault_signer.key,
        ctx.accounts.user_source_token.key,
        ctx.accounts.user_destination_token.key,
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

/// Creates and invokes a [raydium_contract_instructions::stable_instruction::swap_base_out] instruction.
///
/// # Arguments
///
/// See [raydium_contract_instructions::stable_instruction::SwapInstructionBaseOut].
///
/// * `max_amount_in` - SOURCE amount to transfer, output to DESTINATION is based on the exchange rate.
/// * `amount_out` - Minimum amount of DESTINATION token to output, prevents excessive slippage
pub fn swap_base_out<'a, 'b, 'c, 'info>(
    ctx: CpiContext<'a, 'b, 'c, 'info, SwapBaseOut<'info>>,
    max_amount_in: u64,
    amount_out: u64,
) -> Result<()> {
    let ix = stable_instruction::swap_base_out(
        ctx.program.key,
        ctx.accounts.amm.key,
        ctx.accounts.amm_authority.key,
        ctx.accounts.amm_open_orders.key,
        ctx.accounts.amm_coin_vault.key,
        ctx.accounts.amm_pc_vault.key,
        ctx.accounts.model_data_account.key,
        ctx.accounts.serum_program.key,
        ctx.accounts.serum_market.key,
        ctx.accounts.serum_bids.key,
        ctx.accounts.serum_asks.key,
        ctx.accounts.serum_event_queue.key,
        ctx.accounts.serum_coin_vault.key,
        ctx.accounts.serum_pc_vault.key,
        ctx.accounts.serum_vault_signer.key,
        ctx.accounts.user_source_token.key,
        ctx.accounts.user_destination_token.key,
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
