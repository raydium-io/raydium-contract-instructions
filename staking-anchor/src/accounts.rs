//! Accounts structs for Raydium farm program.

use anchor_lang::prelude::*;
use anchor_spl::token::Token;
/// Accounts for an `Initialize` instruction.
#[derive(Accounts, Clone)]
pub struct Initialize<'info> {
    /// CHECK: Safe.
    #[account(mut)]
    pub stake_pool: UncheckedAccount<'info>,
    #[account(mut)]
    pub authority: Signer<'info>,
    /// CHECK: Safe.
    pub lp_vault: UncheckedAccount<'info>,
    /// CHECK: Safe.
    pub reward_vault: UncheckedAccount<'info>,
    /// Clock program
    pub clock: Sysvar<'info, Clock>,
}

/// Initialize arguments
#[derive(AnchorSerialize, AnchorDeserialize, Eq, PartialEq, Clone, Copy, Debug)]
pub struct InitArgs {
    /// nonce for calc authority
    pub nonce: u64,
    /// reward per slot b
    pub reward_per_slot: u64,
    /// reward per slot a
    pub ignore: u128,
}

/// Accounts for an `Deposit` instruction.
#[derive(Accounts, Clone)]
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
    pub dest_reward_token: UncheckedAccount<'info>,
    /// CHECK: Safe.
    #[account(mut)]
    pub vault_reward_token: UncheckedAccount<'info>,
    pub clock: Sysvar<'info, Clock>,
    pub spl_token_program: Program<'info, Token>,
}

/// Accounts for an `Withdraw` instruction.
#[derive(Accounts, Clone)]
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
    pub dest_reward_token: UncheckedAccount<'info>,
    /// CHECK: Safe.
    #[account(mut)]
    pub vault_reward_token: UncheckedAccount<'info>,
    pub clock: Sysvar<'info, Clock>,
    pub spl_token_program: Program<'info, Token>,
}

/// Accounts for an `EmergencyWithdraw` instruction.
#[derive(Accounts, Clone)]
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
#[derive(Accounts, Clone)]
pub struct UpdatePool<'info> {
    /// CHECK: Safe.
    #[account(mut)]
    pub stake_pool: UncheckedAccount<'info>,
    /// CHECK: Safe.
    #[account(mut)]
    pub vault_lp_token: UncheckedAccount<'info>,
    /// CHECK: Safe. Clock program
    pub clock: Sysvar<'info, Clock>,
}

/// Accounts for an `CreateAssociatedAccount` instruction.
#[derive(Accounts, Clone)]
pub struct CreateAssociatedAccount<'info> {
    /// CHECK: Safe.
    #[account(mut)]
    pub stake_pool: UncheckedAccount<'info>,
    /// CHECK: Safe.
    #[account(mut)]
    pub associated_user_stake_info: UncheckedAccount<'info>,
    #[account(mut)]
    pub owner: Signer<'info>,
    /// CHECK: Safe System program
    pub system_program: Program<'info, System>,
    /// CHECK: Safe. Clock program
    pub clock: Sysvar<'info, Clock>,
}
