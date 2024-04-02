//! Accounts structs for Raydium farm program.

use anchor_lang::prelude::*;
use anchor_spl::token::Token;
/// Accounts for an `Initialize` instruction.
#[derive(Accounts)]
pub struct Initialize<'info> {
    /// CHECK: Safe. Initializes a new StakePool.
    pub owner: UncheckedAccount<'info>,
    /// CHECK: Safe.
    #[account(mut)]
    pub stake_pool: UncheckedAccount<'info>,
    /// CHECK: Safe.
    pub authority: UncheckedAccount<'info>,
    /// CHECK: Safe.
    pub lp_vault: UncheckedAccount<'info>,
    /// CHECK: Safe.
    pub reward_vault_a: UncheckedAccount<'info>,
    /// System clock
    pub clock: Sysvar<'info, Clock>,
}

/// Initialize arguments
#[derive(AnchorSerialize, AnchorDeserialize, Eq, PartialEq, Clone, Copy, Debug)]
pub struct InitArgs {
    /// nonce for calc authority
    pub nonce: u64,
    /// reward per slot b
    pub reward_per_slot_a: u64,
    /// reward per slot a
    pub reward_per_slot_b: u64,
}

/// Accounts for an `Deposit` instruction.
#[derive(Accounts)]
pub struct Deposit<'info> {
    /// CHECK: Safe.
    #[account(mut)]
    pub stake_pool: UncheckedAccount<'info>,
    /// CHECK: Safe.
    pub pool_authority: UncheckedAccount<'info>,
    /// CHECK: Safe.
    #[account(mut)]
    pub staker_info: UncheckedAccount<'info>,
    #[account(mut)]
    pub staker_owner: Signer<'info>,
    /// CHECK: Safe.
    #[account(mut)]
    pub src_lp_token: UncheckedAccount<'info>,
    /// CHECK: Safe.
    #[account(mut)]
    pub vault_lp_token: UncheckedAccount<'info>,
    /// CHECK: Safe.
    #[account(mut)]
    pub dest_reward_token_a: UncheckedAccount<'info>,
    /// CHECK: Safe.
    #[account(mut)]
    pub vault_reward_token_a: UncheckedAccount<'info>,
    pub clock: Sysvar<'info, Clock>,
    pub spl_token_program: Program<'info, Token>,
}

/// Accounts for an `Deposit` instruction.
#[derive(Accounts)]
pub struct DepositV2<'info> {
    /// CHECK: Safe.
    #[account(mut)]
    pub stake_pool: UncheckedAccount<'info>,
    /// CHECK: Safe.
    pub pool_authority: UncheckedAccount<'info>,
    /// CHECK: Safe.
    #[account(mut)]
    pub staker_info: UncheckedAccount<'info>,
    #[account(mut)]
    pub staker_owner: Signer<'info>,
    /// CHECK: Safe.
    #[account(mut)]
    pub src_lp_token: UncheckedAccount<'info>,
    /// CHECK: Safe.
    #[account(mut)]
    pub vault_lp_token: UncheckedAccount<'info>,
    /// CHECK: Safe.
    #[account(mut)]
    pub dest_reward_token_a: UncheckedAccount<'info>,
    /// CHECK: Safe.
    #[account(mut)]
    pub vault_reward_token_a: UncheckedAccount<'info>,
    /// CHECK: Safe.
    #[account(mut)]
    pub dest_reward_token_b: UncheckedAccount<'info>,
    /// CHECK: Safe.
    #[account(mut)]
    pub vault_reward_token_b: UncheckedAccount<'info>,
    pub clock: Sysvar<'info, Clock>,
    pub spl_token_program: Program<'info, Token>,
}

/// Accounts for an `Withdraw` instruction.
#[derive(Accounts)]
pub struct Withdraw<'info> {
    /// CHECK: Safe.
    #[account(mut)]
    pub stake_pool: UncheckedAccount<'info>,
    /// CHECK: Safe.
    pub pool_authority: UncheckedAccount<'info>,
    /// CHECK: Safe.
    #[account(mut)]
    pub staker_info: UncheckedAccount<'info>,
    #[account(mut)]
    pub staker_owner: Signer<'info>,
    /// CHECK: Safe.
    #[account(mut)]
    pub dest_lp_token: UncheckedAccount<'info>,
    /// CHECK: Safe.
    #[account(mut)]
    pub vault_lp_token: UncheckedAccount<'info>,
    /// CHECK: Safe.
    #[account(mut)]
    pub dest_reward_token_a: UncheckedAccount<'info>,
    /// CHECK: Safe.
    #[account(mut)]
    pub vault_reward_token_a: UncheckedAccount<'info>,
    pub clock: Sysvar<'info, Clock>,
    pub spl_token_program: Program<'info, Token>,
}

/// Accounts for an `WithdrawV2` instruction.
#[derive(Accounts)]
pub struct WithdrawV2<'info> {
    /// CHECK: Safe.
    #[account(mut)]
    pub stake_pool: UncheckedAccount<'info>,
    /// CHECK: Safe.
    pub pool_authority: UncheckedAccount<'info>,
    /// CHECK: Safe.
    #[account(mut)]
    pub staker_info: UncheckedAccount<'info>,
    #[account(mut)]
    pub staker_owner: Signer<'info>,
    /// CHECK: Safe.
    #[account(mut)]
    pub dest_lp_token: UncheckedAccount<'info>,
    /// CHECK: Safe.
    #[account(mut)]
    pub vault_lp_token: UncheckedAccount<'info>,
    /// CHECK: Safe.
    #[account(mut)]
    pub dest_reward_token_a: UncheckedAccount<'info>,
    /// CHECK: Safe.
    #[account(mut)]
    pub vault_reward_token_a: UncheckedAccount<'info>,
    pub clock: Sysvar<'info, Clock>,
    pub spl_token_program: Program<'info, Token>,
}

/// Accounts for an `EmergencyWithdraw` instruction.
#[derive(Accounts)]
pub struct EmergencyWithdraw<'info> {
    /// CHECK: Safe.
    #[account(mut)]
    pub stake_pool: UncheckedAccount<'info>,
    /// CHECK: Safe.
    pub pool_authority: UncheckedAccount<'info>,
    /// CHECK: Safe.
    #[account(mut)]
    pub staker_info: UncheckedAccount<'info>,
    #[account(mut)]
    pub staker_owner: Signer<'info>,
    /// CHECK: Safe.
    #[account(mut)]
    pub dest_lp_token: UncheckedAccount<'info>,
    /// CHECK: Safe.
    #[account(mut)]
    pub vault_lp_token: UncheckedAccount<'info>,
    pub spl_token_program: Program<'info, Token>,
}

/// Accounts for an `UpdatePool` instruction.
#[derive(Accounts)]
pub struct UpdatePool<'info> {
    /// CHECK: Safe.
    #[account(mut)]
    pub stake_pool: UncheckedAccount<'info>,
    /// CHECK: Safe.
    #[account(mut)]
    pub vault_lp_token: UncheckedAccount<'info>,
    ///  Clock program
    pub clock: Sysvar<'info, Clock>,
}

/// Accounts for an `CreateAssociatedAccount` instruction.
#[derive(Accounts)]
pub struct CreateAssociatedAccount<'info> {
    /// CHECK: Safe.
    #[account(mut)]
    pub stake_pool: UncheckedAccount<'info>,
    /// CHECK: Safe.
    #[account(mut)]
    pub associated_user_stake_info: UncheckedAccount<'info>,
    #[account(mut)]
    pub owner: Signer<'info>,
    /// Safe System program
    pub system_program: Program<'info, System>,
    /// Clock program
    pub rent: Sysvar<'info, Rent>,
}
