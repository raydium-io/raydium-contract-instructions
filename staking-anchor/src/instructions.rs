use crate::*;
use anchor_lang::{prelude::*, solana_program};
use raydium_contract_instructions::staking_instruction;

/// Creates and invokes a [raydium_contract_instructions::staking_instruction::initialize] instruction.
///
/// # Arguments
///
/// See [raydium_contract_instructions::staking_instruction::InitArgs].
///
pub fn initialize<'a, 'b, 'c, 'info>(
    ctx: CpiContext<'a, 'b, 'c, 'info, Initialize<'info>>,
    init_args: InitArgs,
) -> Result<()> {
    let ix = staking_instruction::initialize(
        ctx.program.key,
        ctx.accounts.stake_pool.key,
        ctx.accounts.authority.key,
        ctx.accounts.lp_vault.key,
        ctx.accounts.reward_vault.key,
        staking_instruction::InitArgs {
            nonce: init_args.nonce,
            reward_per_slot: init_args.reward_per_slot,
            ignore: init_args.ignore,
        },
    )?;

    solana_program::program::invoke_signed(
        &ix,
        &ToAccountInfos::to_account_infos(&ctx),
        ctx.signer_seeds,
    )?;
    Ok(())
}

/// Creates and invokes a [raydium_contract_instructions::staking_instruction::deposit] instruction.
///
/// # Arguments
///
/// * `amount` - The amount to deposit.
pub fn deposit<'a, 'b, 'c, 'info>(
    ctx: CpiContext<'a, 'b, 'c, 'info, Deposit<'info>>,
    amount: u64,
) -> Result<()> {
    let ix = staking_instruction::deposit(
        ctx.program.key,
        ctx.accounts.stake_pool.key,
        ctx.accounts.pool_authority.key,
        ctx.accounts.staker_info.key,
        ctx.accounts.staker_owner.key,
        ctx.accounts.src_lp_token.key,
        ctx.accounts.vault_lp_token.key,
        ctx.accounts.dest_reward_token.key,
        ctx.accounts.vault_reward_token.key,
        ctx.accounts.spl_token_program.key,
        amount,
    )?;

    solana_program::program::invoke_signed(
        &ix,
        &ToAccountInfos::to_account_infos(&ctx),
        ctx.signer_seeds,
    )?;
    Ok(())
}

/// Creates and invokes a [raydium_contract_instructions::staking_instruction::withdraw] instruction.
///
/// # Arguments
///
/// * `amount` - The amount to deposit.
pub fn withdraw<'a, 'b, 'c, 'info>(
    ctx: CpiContext<'a, 'b, 'c, 'info, Withdraw<'info>>,
    amount: u64,
) -> Result<()> {
    let ix = staking_instruction::withdraw(
        ctx.program.key,
        ctx.accounts.stake_pool.key,
        ctx.accounts.pool_authority.key,
        ctx.accounts.staker_info.key,
        ctx.accounts.staker_owner.key,
        ctx.accounts.dest_lp_token.key,
        ctx.accounts.vault_lp_token.key,
        ctx.accounts.dest_reward_token.key,
        ctx.accounts.vault_reward_token.key,
        ctx.accounts.spl_token_program.key,
        amount,
    )?;

    solana_program::program::invoke_signed(
        &ix,
        &ToAccountInfos::to_account_infos(&ctx),
        ctx.signer_seeds,
    )?;
    Ok(())
}

/// Creates and invokes a [raydium_contract_instructions::staking_instruction::emergency_withdraw] instruction.
///
/// # Arguments
///
pub fn emergency_withdraw<'a, 'b, 'c, 'info>(
    ctx: CpiContext<'a, 'b, 'c, 'info, EmergencyWithdraw<'info>>,
) -> Result<()> {
    let ix = staking_instruction::emergency_withdraw(
        ctx.program.key,
        ctx.accounts.stake_pool.key,
        ctx.accounts.pool_authority.key,
        ctx.accounts.staker_info.key,
        ctx.accounts.staker_owner.key,
        ctx.accounts.dest_lp_token.key,
        ctx.accounts.vault_lp_token.key,
        ctx.accounts.spl_token_program.key,
    )?;

    solana_program::program::invoke_signed(
        &ix,
        &ToAccountInfos::to_account_infos(&ctx),
        ctx.signer_seeds,
    )?;
    Ok(())
}

/// Creates and invokes a [raydium_contract_instructions::staking_instruction::update_pool] instruction.
///
/// # Arguments
///
pub fn update_pool<'a, 'b, 'c, 'info>(
    ctx: CpiContext<'a, 'b, 'c, 'info, UpdatePool<'info>>,
) -> Result<()> {
    let ix = staking_instruction::update_pool(
        ctx.program.key,
        ctx.accounts.stake_pool.key,
        ctx.accounts.vault_lp_token.key,
    )?;

    solana_program::program::invoke_signed(
        &ix,
        &ToAccountInfos::to_account_infos(&ctx),
        ctx.signer_seeds,
    )?;
    Ok(())
}

/// Creates and invokes a [raydium_contract_instructions::staking_instruction::create_associated_account] instruction.
///
/// # Arguments
///
pub fn create_associated_account<'a, 'b, 'c, 'info>(
    ctx: CpiContext<'a, 'b, 'c, 'info, CreateAssociatedAccount<'info>>,
) -> Result<()> {
    let ix = staking_instruction::create_associated_account(
        ctx.program.key,
        ctx.accounts.stake_pool.key,
        ctx.accounts.associated_user_stake_info.key,
        ctx.accounts.owner.key,
    )?;

    solana_program::program::invoke_signed(
        &ix,
        &ToAccountInfos::to_account_infos(&ctx),
        ctx.signer_seeds,
    )?;
    Ok(())
}
