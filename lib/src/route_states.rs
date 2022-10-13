//! State transition types
//!

use solana_program::pubkey::Pubkey;
/// Information about the single route pta account
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct RoutePdaInfo {
    /// route from amm id
    pub route_from_amm_id: Pubkey,
    /// route to amm id
    pub route_to_amm_id: Pubkey,
    /// route token mint
    pub route_token_mint: Pubkey,

    /// user swap route amount
    pub route_amount: u64,
    /// user got swap amount
    pub out_amount: u64,
    /// user route slot
    pub route_in_slot: u64,
}
