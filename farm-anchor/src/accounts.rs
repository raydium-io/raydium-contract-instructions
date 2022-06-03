//! Accounts structs for Raydium farm program.

use anchor_lang::prelude::*;

/// Accounts for an `Initialize` instruction.
#[derive(Accounts, Clone)]
pub struct Initialize<'info> {
    /// CHECK: Safe. Initializes a new StakePool.
    pub owner: AccountInfo<'info>,
    /// CHECK: Safe.
    #[account(mut)]
    pub stake_pool: AccountInfo<'info>,
    /// CHECK: Safe.
    pub authority: AccountInfo<'info>,
    /// CHECK: Safe.
    pub lp_vault: AccountInfo<'info>,
    /// CHECK: Safe.
    pub reward_vault_a: AccountInfo<'info>,
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
#[derive(Accounts, Clone)]
pub struct Deposit<'info> {
    /// CHECK: Safe.
    #[account(mut)]
    pub stake_pool: AccountInfo<'info>,
    /// CHECK: Safe.
    pub pool_authority: AccountInfo<'info>,
    /// CHECK: Safe.
    #[account(mut)]
    pub staker_info: AccountInfo<'info>,
    /// CHECK: Safe.
    pub staker_owner: AccountInfo<'info>,
    /// CHECK: Safe.
    #[account(mut)]
    pub src_lp_token: AccountInfo<'info>,
    /// CHECK: Safe.
    #[account(mut)]
    pub vault_lp_token: AccountInfo<'info>,
    /// CHECK: Safe.
    #[account(mut)]
    pub dest_reward_token_a: AccountInfo<'info>,
    /// CHECK: Safe.
    #[account(mut)]
    pub vault_reward_token_a: AccountInfo<'info>,
    /// CHECK: Safe. Clock program
    pub clock: Sysvar<'info, Clock>,
    /// CHECK: Safe Spl token program
    #[account(address = spl_token::ID)]
    pub spl_token_program: AccountInfo<'info>,
}

/// Accounts for an `Deposit` instruction.
#[derive(Accounts, Clone)]
pub struct DepositV2<'info> {
    /// CHECK: Safe.
    #[account(mut)]
    pub stake_pool: AccountInfo<'info>,
    /// CHECK: Safe.
    pub pool_authority: AccountInfo<'info>,
    /// CHECK: Safe.
    #[account(mut)]
    pub staker_info: AccountInfo<'info>,
    /// CHECK: Safe.
    pub staker_owner: AccountInfo<'info>,
    /// CHECK: Safe.
    #[account(mut)]
    pub src_lp_token: AccountInfo<'info>,
    /// CHECK: Safe.
    #[account(mut)]
    pub vault_lp_token: AccountInfo<'info>,
    /// CHECK: Safe.
    #[account(mut)]
    pub dest_reward_token_a: AccountInfo<'info>,
    /// CHECK: Safe.
    #[account(mut)]
    pub vault_reward_token_a: AccountInfo<'info>,
    /// CHECK: Safe.
    #[account(mut)]
    pub dest_reward_token_b: AccountInfo<'info>,
    /// CHECK: Safe.
    #[account(mut)]
    pub vault_reward_token_b: AccountInfo<'info>,
    /// CHECK: Safe. Clock program
    pub clock: Sysvar<'info, Clock>,
    /// CHECK: Safe Spl token program
    #[account(address = spl_token::ID)]
    pub spl_token_program: AccountInfo<'info>,
}

/// Accounts for an `Withdraw` instruction.
#[derive(Accounts, Clone)]
pub struct Withdraw<'info> {
    /// CHECK: Safe.
    #[account(mut)]
    pub stake_pool: AccountInfo<'info>,
    /// CHECK: Safe.
    pub pool_authority: AccountInfo<'info>,
    /// CHECK: Safe.
    #[account(mut)]
    pub staker_info: AccountInfo<'info>,
    /// CHECK: Safe.
    pub staker_owner: AccountInfo<'info>,
    /// CHECK: Safe.
    #[account(mut)]
    pub dest_lp_token: AccountInfo<'info>,
    /// CHECK: Safe.
    #[account(mut)]
    pub vault_lp_token: AccountInfo<'info>,
    /// CHECK: Safe.
    #[account(mut)]
    pub dest_reward_token_a: AccountInfo<'info>,
    /// CHECK: Safe.
    #[account(mut)]
    pub vault_reward_token_a: AccountInfo<'info>,
    /// CHECK: Safe. Clock program
    pub clock: Sysvar<'info, Clock>,
    /// CHECK: Safe Spl token program
    #[account(address = spl_token::ID)]
    pub spl_token_program: AccountInfo<'info>,
}

/// Accounts for an `WithdrawV2` instruction.
#[derive(Accounts, Clone)]
pub struct WithdrawV2<'info> {
    /// CHECK: Safe.
    #[account(mut)]
    pub stake_pool: AccountInfo<'info>,
    /// CHECK: Safe.
    pub pool_authority: AccountInfo<'info>,
    /// CHECK: Safe.
    #[account(mut)]
    pub staker_info: AccountInfo<'info>,
    /// CHECK: Safe.
    pub staker_owner: AccountInfo<'info>,
    /// CHECK: Safe.
    #[account(mut)]
    pub dest_lp_token: AccountInfo<'info>,
    /// CHECK: Safe.
    #[account(mut)]
    pub vault_lp_token: AccountInfo<'info>,
    /// CHECK: Safe.
    #[account(mut)]
    pub dest_reward_token_a: AccountInfo<'info>,
    /// CHECK: Safe.
    #[account(mut)]
    pub vault_reward_token_a: AccountInfo<'info>,
    /// CHECK: Safe. Clock program
    pub clock: Sysvar<'info, Clock>,
    /// CHECK: Safe Spl token program
    #[account(address = spl_token::ID)]
    pub spl_token_program: AccountInfo<'info>,
}

/// Accounts for an `EmergencyWithdraw` instruction.
#[derive(Accounts, Clone)]
pub struct EmergencyWithdraw<'info> {
    /// CHECK: Safe.
    #[account(mut)]
    pub stake_pool: AccountInfo<'info>,
    /// CHECK: Safe.
    pub pool_authority: AccountInfo<'info>,
    /// CHECK: Safe.
    #[account(mut)]
    pub staker_info: AccountInfo<'info>,
    /// CHECK: Safe.
    pub staker_owner: AccountInfo<'info>,
    /// CHECK: Safe.
    #[account(mut)]
    pub dest_lp_token: AccountInfo<'info>,
    /// CHECK: Safe.
    #[account(mut)]
    pub vault_lp_token: AccountInfo<'info>,
    /// CHECK: Safe Spl token program
    #[account(address = spl_token::ID)]
    pub spl_token_program: AccountInfo<'info>,
}

/// Accounts for an `UpdatePool` instruction.
#[derive(Accounts, Clone)]
pub struct UpdatePool<'info> {
    /// CHECK: Safe.
    #[account(mut)]
    pub stake_pool: AccountInfo<'info>,
    /// CHECK: Safe.
    #[account(mut)]
    pub vault_lp_token: AccountInfo<'info>,
    /// CHECK: Safe. Clock program
    pub clock: Sysvar<'info, Clock>,
}

/// Accounts for an `CreateAssociatedAccount` instruction.
#[derive(Accounts, Clone)]
pub struct CreateAssociatedAccount<'info> {
    /// CHECK: Safe.
    #[account(mut)]
    pub stake_pool: AccountInfo<'info>,
    /// CHECK: Safe.
    #[account(mut)]
    pub associated_user_stake_info: AccountInfo<'info>,
    /// CHECK: Safe.
    pub owner: AccountInfo<'info>,
    /// CHECK: Safe System program
    pub system_program: Program<'info, System>,
    /// CHECK: Safe. Clock program
    pub rent: Sysvar<'info, Rent>,
}
