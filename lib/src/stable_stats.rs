//! State transition types

use solana_program::pubkey::Pubkey;

#[cfg_attr(feature = "client", derive(Debug))]
#[derive(Clone, Copy)]
pub struct TargetOrder {
    pub plan_price: u128,
    pub plan_vol: u128,
    pub place_price: u64,
    pub place_vol: u64,
}

#[cfg_attr(feature = "client", derive(Debug))]
#[derive(Clone, Copy)]
pub struct TargetOrders {
    /// indicate the account type.
    pub account_type: u64,
    pub status: u64,
    pub buy_orders: [TargetOrder; 32],
    pub sell_orders: [TargetOrder; 32],
    pub last_order_numerator: u64,
    pub last_order_denominator: u64,

    pub plan_orders_cur: u64,
    pub place_orders_cur: u64,

    pub valid_buy_order_num: u64,
    pub valid_sell_order_num: u64,

    pub stable_mid_price: u128,

    pub padding0: [u128; 10],
    // padding, Unused bytes for future upgrades.
    pub padding: [u64; 32],
}

#[derive(Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Debug, Default)]
pub struct Order {
    pub price: u64,
    pub qty: u64,
    pub id: u128,
    pub slot: u64,
}

#[cfg_attr(feature = "client", derive(Debug))]
#[derive(Clone, Copy)]
pub struct MyOrders {
    pub buy_len: u64,
    pub sell_len: u64,
    pub buy_orders: [Order; 64],
    pub sell_orders: [Order; 64],
}

#[repr(u64)]
pub enum AmmStatus {
    Uninitialized = 0u64,
    Initialized = 1u64,
    Disabled = 2u64,
    WithdrawOnly = 3u64,
    // pool only can add or remove liquidity, can't swap and plan orders
    LiquidityOnly = 4u64,
    // pool only can add or remove liquidity and plan orders, can't swap
    OrderBookOnly = 5u64,
    // pool only can add or remove liquidity and swap, can't plan orders
    SwapOnly = 6u64,
    // transfer user swap_in token to the pool token vault as punishment before the ido pool open period through swap instruction
    SwapPunish = 7u64,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Fees {
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
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct OutPutData {
    /// delay to take pnl coin
    pub need_take_pnl_coin: u64,
    /// delay to take pnl pc
    pub need_take_pnl_pc: u64,
    /// total pnl pc
    pub total_pnl_pc: u64,
    /// total pnl coin
    pub total_pnl_coin: u64,
    /// ido pool open time
    pub pool_open_time: u64,
    /// punish pc amount
    pub punish_pc_amount: u64,
    /// punish coin amount
    pub punish_coin_amount: u64,
    /// switch from orderbookonly to init
    pub orderbook_to_init_time: u64,

    /// swap coin in amount
    pub swap_coin_in_amount: u128,
    /// swap pc out amount
    pub swap_pc_out_amount: u128,

    /// swap pc in amount
    pub swap_pc_in_amount: u128,
    /// swap coin out amount
    pub swap_coin_out_amount: u128,

    /// swap pc fee
    pub swap_pc_fee: u64,
    /// swap coin fee
    pub swap_coin_fee: u64,
}

#[cfg_attr(feature = "client", derive(Debug))]
#[repr(C)]
#[derive(Clone, Copy, PartialEq)]
pub struct AmmInfo {
    /// indicate the account type.
    pub account_type: u64,
    /// Initialized status.
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
    /// coinLotSize serum_market coin_lot_size
    pub coin_lot_size: u64,
    /// pcLotSize serum_market pc_lot_size
    pub pc_lot_size: u64,
    /// min_cur_price: (2 * amm.order_num * amm.pc_lot_size) * max_price_multiplier
    pub min_price_multiplier: u64,
    /// max_cur_price: (2 * amm.order_num * amm.pc_lot_size) * max_price_multiplier
    pub max_price_multiplier: u64,
    /// system decimal value, used to normalize the value of coin and pc amount
    pub sys_decimal_value: u64,
    /// stable price is too high or too low to abort trade for safe
    pub abort_trade_factor: u64,
    /// multiplier of price_tick for plan grid
    pub price_tick_multiplier: u64,
    /// price tick
    pub price_tick: u64,
    /// All fee information
    pub fees: Fees,
    /// data calc to output
    pub out_put: OutPutData,
    /// Token coin vault
    pub coin_vault: Pubkey,
    /// Token pc vault
    pub pc_vault: Pubkey,
    /// Coin mint
    pub coin_mint: Pubkey,
    /// Pc mint
    pub pc_mint: Pubkey,
    /// lp mint
    pub lp_mint: Pubkey,
    /// model data account key
    pub model_data_key: Pubkey,
    /// open_orders key
    pub open_orders: Pubkey,
    /// serum market key
    pub serum_market: Pubkey,
    /// serum program key
    pub serum_program: Pubkey,
    /// target_orders key
    pub target_orders: Pubkey,
    /// amm admin key
    pub amm_admin: Pubkey,
    /// padding, Unused bytes for future upgrades.
    pub padding: [u64; 64],
}
