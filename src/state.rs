use borsh::{BorshSerialize, BorshDeserialize};
use solana_program::pubkey::Pubkey;

/// See https://github.com/raydium-io/raydium-sdk/blob/master/src/liquidity/layout.ts
#[derive(Debug, Clone, BorshSerialize, BorshDeserialize)]
pub struct LiquidityStateLayoutV4 {
    pub status: u64,
    pub nonce: u64,
    pub max_order: u64,
    pub depth: u64,
    /// minimal decimal step amid orders in relation to decimals of relevant mint
    pub base_decimal: u64,
    pub quote_decimal: u64,
    pub state: u64,
    pub reset_flag: u64,
    /// min size of trade in quote
    pub min_size: u64,
    pub vol_max_cut_ratio: u64,
    pub amount_wave_ratio: u64,
    pub base_lot_size: u64,
    pub quote_lot_size: u64,
    pub min_price_multiplier: u64,
    pub max_price_multiplier: u64,
    pub system_decimal_value: u64,
    pub min_separate_numerator: u64,
    pub min_separate_denominator: u64,
    pub trade_fee_numerator: u64,
    pub trade_fee_denominator: u64,
    pub pnl_numerator: u64,
    pub pnl_denominator: u64,
    pub swap_fee_numerator: u64,
    pub swap_fee_denominator: u64,
    pub base_need_take_pnl: u64,
    pub quote_need_take_pnl: u64,
    pub quote_total_pnl: u64,
    pub base_total_pnl: u64,
    pub quote_total_deposited:u128,
    pub base_total_deposited:u128,
    pub swap_base_in_amount:u128,
    pub swap_quote_out_amount:u128,
    pub swap_base2_quote_fee: u64,
    pub swap_quote_in_amount: u128,
    pub swap_base_out_amount: u128,
    pub swap_quote2_base_fee: u64,
    // amm vault
    /// base spl token account
    pub base_vault: Pubkey,
    /// quite spl token account
    pub quote_vault: Pubkey,
    // mint
    pub base_mint: Pubkey,
    pub quote_mint: Pubkey,
    pub lp_mint: Pubkey,
    // market
    /// orders on market done by this pool
    pub open_orders: Pubkey,
    /// usually order book, usually serum
    pub market_id: Pubkey,
    pub market_program_id: Pubkey,
    pub target_orders: Pubkey,
    pub withdraw_queue: Pubkey,
    pub lp_vault: Pubkey,
    pub owner: Pubkey,
    pub pnl_owner: Pubkey,
}