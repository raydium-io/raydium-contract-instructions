//! State transition types

use solana_program::pubkey::Pubkey;

/// Initialized program details.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct StakePool {
    /// state
    pub state: u64,
    /// Nonce used in program address.
    pub nonce: u64,
    /// lp token vault fro present all lp
    pub lp_vault: Pubkey,
    /// reward vault
    pub reward_vault: Pubkey,
    /// owner
    pub owner: Pubkey,
    /// padding
    pub padding: [u64; 6],
    /// total reward multified  decimals
    pub reward_total: u64,
    /// acc ray per share
    pub acc_ray_per_share: u128,
    /// Last slot pool updated
    pub last_pool_update_slot: u64,
    /// reward per slot, multified  decimels
    pub reward_per_slot: u64,
}

/// Information about the singe  stake account
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct StakerInfo {
    /// state
    pub state: u64,

    /// stake pool account
    pub stake_pool: Pubkey,

    /// spl token owner account and havrest owner pubkey
    pub staker_owner: Pubkey,

    /// staking balance in lp
    pub deposit_balance: u64,

    /// reward debt
    pub reward_debt: u64,
}

/// Information about the singe  stake account
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct StakerInfoV2 {
    /// state
    pub state: u64,

    /// stake pool account
    pub stake_pool: Pubkey,

    /// spl token owner account and havrest owner pubkey
    pub staker_owner: Pubkey,

    /// staking balance in lp
    pub deposit_balance: u64,

    /// reward debt
    pub reward_debt: u128,

    /// account type
    pub account_type: u64,

    /// padding
    pub padding: [u64; 16],
}
