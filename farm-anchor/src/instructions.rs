use crate::*;
use anchor_lang::{prelude::*, solana_program};
use raydium_contract_instructions::farm_instruction;

/// Creates and invokes a [raydium_contract_instructions::farm_instruction::initialize] instruction.
///
/// # Arguments
///
/// See [raydium_contract_instructions::farm_instruction::InitArgs].
///
pub fn initialize<'a, 'b, 'c, 'info>(
    ctx: CpiContext<'a, 'b, 'c, 'info, Initialize<'info>>,
    init_args: InitArgs,
) -> Result<()> {
    let reward_vault_b = ctx.remaining_accounts.get(0);
    let admin_authority = ctx.remaining_accounts.get(1);

    let ix = farm_instruction::initialize(
        ctx.program.key,
        ctx.accounts.owner.key,
        ctx.accounts.stake_pool.key,
        ctx.accounts.authority.key,
        ctx.accounts.lp_vault.key,
        ctx.accounts.reward_vault_a.key,
        reward_vault_b.map(|info| info.key),
        admin_authority.map(|info| info.key),
        farm_instruction::InitArgs {
            nonce: init_args.nonce,
            reward_per_slot_a: init_args.reward_per_slot_a,
            reward_per_slot_b: init_args.reward_per_slot_b,
        },
    )?;

    solana_program::program::invoke_signed(
        &ix,
        &ToAccountInfos::to_account_infos(&ctx),
        ctx.signer_seeds,
    )?;
    Ok(())
}

/// Creates and invokes a [raydium_contract_instructions::farm_instruction::deposit] instruction.
///
/// # Arguments
///
/// * `amount` - The amount to deposit.
pub fn deposit<'a, 'b, 'c, 'info>(
    ctx: CpiContext<'a, 'b, 'c, 'info, Deposit<'info>>,
    amount: u64,
) -> Result<()> {
    let dest_reward_token_b = ctx.remaining_accounts.get(0);
    let vault_reward_token_b = ctx.remaining_accounts.get(1);
    let ix = farm_instruction::deposit(
        ctx.program.key,
        ctx.accounts.stake_pool.key,
        ctx.accounts.pool_authority.key,
        ctx.accounts.staker_info.key,
        ctx.accounts.staker_owner.key,
        ctx.accounts.src_lp_token.key,
        ctx.accounts.vault_lp_token.key,
        ctx.accounts.dest_reward_token_a.key,
        ctx.accounts.vault_reward_token_a.key,
        dest_reward_token_b.map(|info| info.key),
        vault_reward_token_b.map(|info| info.key),
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

/// Creates and invokes a [raydium_contract_instructions::farm_instruction::deposit_v2] instruction.
///
/// # Arguments
///
/// * `amount` - The amount to deposit.
pub fn deposit_v2<'a, 'b, 'c, 'info>(
    ctx: CpiContext<'a, 'b, 'c, 'info, DepositV2<'info>>,
    amount: u64,
) -> Result<()> {
    let staker_info_v1 = ctx.remaining_accounts.get(0);
    let ix = farm_instruction::deposit_v2(
        ctx.program.key,
        ctx.accounts.stake_pool.key,
        ctx.accounts.pool_authority.key,
        ctx.accounts.staker_info.key,
        ctx.accounts.staker_owner.key,
        ctx.accounts.src_lp_token.key,
        ctx.accounts.vault_lp_token.key,
        ctx.accounts.dest_reward_token_a.key,
        ctx.accounts.vault_reward_token_a.key,
        ctx.accounts.dest_reward_token_b.key,
        ctx.accounts.vault_reward_token_b.key,
        ctx.accounts.spl_token_program.key,
        staker_info_v1.map(|info| info.key),
        amount,
    )?;

    solana_program::program::invoke_signed(
        &ix,
        &ToAccountInfos::to_account_infos(&ctx),
        ctx.signer_seeds,
    )?;
    Ok(())
}

/// Creates and invokes a [raydium_contract_instructions::farm_instruction::withdraw] instruction.
///
/// # Arguments
///
/// * `amount` - The amount to withdraw.
pub fn withdraw<'a, 'b, 'c, 'info>(
    ctx: CpiContext<'a, 'b, 'c, 'info, Withdraw<'info>>,
    amount: u64,
) -> Result<()> {
    let dest_reward_token_b = ctx.remaining_accounts.get(0);
    let vault_reward_token_b = ctx.remaining_accounts.get(1);
    let ix = farm_instruction::withdraw(
        ctx.program.key,
        ctx.accounts.stake_pool.key,
        ctx.accounts.pool_authority.key,
        ctx.accounts.staker_info.key,
        ctx.accounts.staker_owner.key,
        ctx.accounts.dest_lp_token.key,
        ctx.accounts.vault_lp_token.key,
        ctx.accounts.dest_reward_token_a.key,
        ctx.accounts.vault_reward_token_a.key,
        dest_reward_token_b.map(|info| info.key),
        vault_reward_token_b.map(|info| info.key),
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

/// Creates and invokes a [raydium_contract_instructions::farm_instruction::withdraw_v2] instruction.
///
/// # Arguments
///
/// * `amount` - The amount to withdraw.
pub fn withdraw_v2<'a, 'b, 'c, 'info>(
    ctx: CpiContext<'a, 'b, 'c, 'info, WithdrawV2<'info>>,
    amount: u64,
) -> Result<()> {
    let dest_reward_token_b = ctx.remaining_accounts.get(0);
    let vault_reward_token_b = ctx.remaining_accounts.get(1);
    let ix = farm_instruction::withdraw_v2(
        ctx.program.key,
        ctx.accounts.stake_pool.key,
        ctx.accounts.pool_authority.key,
        ctx.accounts.staker_info.key,
        ctx.accounts.staker_owner.key,
        ctx.accounts.dest_lp_token.key,
        ctx.accounts.vault_lp_token.key,
        ctx.accounts.dest_reward_token_a.key,
        ctx.accounts.vault_reward_token_a.key,
        dest_reward_token_b.map(|info| info.key),
        vault_reward_token_b.map(|info| info.key),
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

/// Creates and invokes a [raydium_contract_instructions::farm_instruction::emergency_withdraw] instruction.
///
/// # Arguments
///
pub fn emergency_withdraw<'a, 'b, 'c, 'info>(
    ctx: CpiContext<'a, 'b, 'c, 'info, EmergencyWithdraw<'info>>,
) -> Result<()> {
    let ix = farm_instruction::emergency_withdraw(
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

/// Creates and invokes a [raydium_contract_instructions::farm_instruction::update_pool] instruction.
///
/// # Arguments
///
pub fn update_pool<'a, 'b, 'c, 'info>(
    ctx: CpiContext<'a, 'b, 'c, 'info, UpdatePool<'info>>,
) -> Result<()> {
    let ix = farm_instruction::update_pool(
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

/// Creates and invokes a [raydium_contract_instructions::farm_instruction::create_associated_account] instruction.
///
/// # Arguments
///
pub fn create_associated_account<'a, 'b, 'c, 'info>(
    ctx: CpiContext<'a, 'b, 'c, 'info, CreateAssociatedAccount<'info>>,
) -> Result<()> {
    let ix = farm_instruction::create_associated_account(
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
