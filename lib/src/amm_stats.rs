//! State transition types

use solana_program::pubkey::Pubkey;

#[repr(u64)]
pub enum AmmStatus {
    Uninitialized = 0u64,
    Initialized = 1u64,
    Disabled = 2u64,
    WithdrawOnly = 3u64,
}

#[repr(u64)]
pub enum AmmState {
    InvlidState = 0u64,
    IdleState = 1u64,
    CancelAllOrdersState = 2u64,
    PlanOrdersState = 3u64,
    CancelOrderState = 4u64,
    PlaceOrdersState = 5u64,
    PurgeOrderState = 6u64,
    WithdrawTransferState = 7u64,
}

#[cfg_attr(feature = "client", derive(Debug))]
#[repr(C)]
#[derive(Clone, Copy, Default, PartialEq)]
pub struct AmmInfo {
    /// 1 Initialized status.
    pub status: u64,
    /// Nonce used in program address.
    /// The program address is created deterministically with the nonce,
    /// amm program id, and amm account pubkey.  This program address has
    /// authority over the amm's token coin account, token pc account, and pool
    /// token mint.
    pub nonce: u64,
    /// max order count
    pub order_num: u64,
    /// within this range, 5 => 5% range
    pub depth: u64,
    /// coin decimal
    pub coin_decimals: u64,
    /// pc decimal
    pub pc_decimals: u64,
    /// amm state
    pub state: u64,
    /// amm reset_flag
    pub reset_flag: u64,
    /// min size 1->0.000001
    pub min_size: u64,
    /// vol_max_cut_ratio numerator, sys_decimal_value as denominator
    pub vol_max_cut_ratio: u64,
    /// amount wave numerator, sys_decimal_value as denominator
    pub amount_wave: u64,
    /// coinLotSize 1 -> 0.000001
    pub coin_lot_size: u64,
    /// pcLotSize 1 -> 0.000001
    pub pc_lot_size: u64,
    /// min_cur_price: (2 * amm.order_num * amm.pc_lot_size) * max_price_multiplier
    pub min_price_multiplier: u64,
    /// max_cur_price: (2 * amm.order_num * amm.pc_lot_size) * max_price_multiplier
    pub max_price_multiplier: u64,
    /// system decimal value, used to normalize the value of coin and pc amount
    pub sys_decimal_value: u64,

    /// numerator of the min_separate
    pub min_separate_numerator: u64,
    /// denominator of the min_separate
    pub min_separate_denominator: u64,

    /// numerator of the fee
    pub trade_fee_numerator: u64,
    /// denominator of the fee
    /// and 'trade_fee_denominator' must be equal to 'min_separate_denominator'
    pub trade_fee_denominator: u64,

    /// numerator of the pnl
    pub pnl_numerator: u64,
    /// denominator of the pnl
    pub pnl_denominator: u64,

    /// numerator of the swap_fee
    pub swap_fee_numerator: u64,
    /// denominator of the swap_fee
    pub swap_fee_denominator: u64,

    /// delay to take pnl coin
    pub need_take_pnl_coin: u64,
    /// delay to take pnl pc
    pub need_take_pnl_pc: u64,
    /// total pnl pc
    pub total_pnl_pc: u64,
    /// total pnl coin
    pub total_pnl_coin: u64,
    /// pool total deposit pc
    pub pool_total_deposit_pc: u128,
    /// pool total deposit coin
    pub pool_total_deposit_coin: u128,

    /// swap coin in amount
    pub swap_coin_in_amount: u128,
    /// swap pc out amount
    pub swap_pc_out_amount: u128,
    /// swap coin to pc fee
    pub swap_coin2pc_fee: u64,

    /// swap pc in amount
    pub swap_pc_in_amount: u128,
    /// swap coin out amount
    pub swap_coin_out_amount: u128,
    /// swap pc to coin fee
    pub swap_pc2coin_fee: u64,

    /// Token coin
    pub token_coin: Pubkey,
    /// Token pc
    pub token_pc: Pubkey,
    /// Coin mint
    pub coin_mint: Pubkey,
    /// Pc mint
    pub pc_mint: Pubkey,
    /// lp mint
    pub lp_mint: Pubkey,
    /// open_orders key
    pub open_orders: Pubkey,
    /// market key
    pub market: Pubkey,
    /// serum dex key
    pub serum_dex: Pubkey,
    /// target_orders key
    pub target_orders: Pubkey,
    /// withdraw key
    pub withdraw_queue: Pubkey,
    /// temp lp key
    pub token_temp_lp: Pubkey,
    /// amm owner key
    pub amm_owner: Pubkey,
    /// pnl_owner key
    pub pnl_owner: Pubkey,
}
