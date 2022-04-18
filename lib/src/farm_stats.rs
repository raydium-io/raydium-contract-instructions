//! State transition types

use solana_program::pubkey::Pubkey;

/// Initialized program details.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct FarmPool {
    /// state
    pub state: u64,
    /// Nonce used in program address.
    pub nonce: u64,
    /// lp token vault
    pub lp_vault: Pubkey,
    /// reward vault a
    pub reward_vault_a: Pubkey,
    /// reward total a
    pub reward_total_a: u64,
    /// acc ray per share
    pub acc_ray_per_share_a: u128,
    /// reward per slot
    pub reward_per_slot_a: u64,

    /// reward vault b
    pub reward_vault_b: Option<Pubkey>,
    /// reward total b
    pub reward_total_b: u64,
    /// acc ray per share
    pub acc_ray_per_share_b: u128,
    /// reward per slot
    pub reward_per_slot_b: u64,

    /// Last slot pool updated
    pub last_pool_update_slot: u64,
    /// owner
    pub owner: Pubkey,
}

/// Information about the singe  stake account
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct FarmerInfo {
    /// state
    pub state: u64,

    /// stake pool account
    pub farm_pool: Pubkey,

    /// spl token owner account and havrest owner pubkey
    pub farmer: Pubkey,

    /// staking balance in lp
    pub deposit_balance: u64,

    /// reward debt a
    pub reward_debt_a: u64,

    /// reward debt b
    pub reward_debt_b: u64,
}

/// Information about the singe  stake account
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct FarmerInfoV2 {
    /// state
    pub state: u64,

    /// stake pool account
    pub farm_pool: Pubkey,

    /// spl token owner account and havrest owner pubkey
    pub farmer: Pubkey,

    /// staking balance in lp
    pub deposit_balance: u64,

    /// reward debt a
    pub reward_debt_a: u128,

    /// reward debt b
    pub reward_debt_b: u128,

    /// account type
    pub account_type: u64,

    /// pending
    pub pending: [u64; 16],
}
