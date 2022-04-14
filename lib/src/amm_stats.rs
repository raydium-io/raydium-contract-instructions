//! State transition types

use crate::error::AmmError;
use solana_program::{
    account_info::AccountInfo,
    program_error::ProgramError,
    program_pack::{IsInitialized, Pack, Sealed},
    pubkey::Pubkey,
};

use arrayref::{array_mut_ref, array_ref, array_refs, mut_array_refs};
use bytemuck::{cast_slice_mut, from_bytes_mut, Pod, Zeroable};
use safe_transmute::{self, trivial::TriviallyTransmutable};
use std::cell::RefMut;

#[repr(u64)]
pub enum AmmStatus {
    Uninitialized = 0u64,
    Initialized = 1u64,
    Disabled = 2u64,
    WithdrawOnly = 3u64,
}
impl AmmStatus {
    pub fn from_u64(status: u64) -> Self {
        match status {
            0u64 => AmmStatus::Uninitialized,
            1u64 => AmmStatus::Initialized,
            2u64 => AmmStatus::Disabled,
            3u64 => AmmStatus::WithdrawOnly,
            _ => unreachable!(),
        }
    }

    pub fn into_u64(&self) -> u64 {
        match self {
            AmmStatus::Uninitialized => 0u64,
            AmmStatus::Initialized => 1u64,
            AmmStatus::Disabled => 2u64,
            AmmStatus::WithdrawOnly => 3u64,
        }
    }
    pub fn valid_status(status: u64) -> bool {
        match status {
            1u64 | 2u64 | 3u64 => return true,
            _ => return false,
        }
    }
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
impl AmmState {
    pub fn from_u64(state: u64) -> Self {
        match state {
            0u64 => AmmState::InvlidState,
            1u64 => AmmState::IdleState,
            2u64 => AmmState::CancelAllOrdersState,
            3u64 => AmmState::PlanOrdersState,
            4u64 => AmmState::CancelOrderState,
            5u64 => AmmState::PlaceOrdersState,
            6u64 => AmmState::PurgeOrderState,
            7u64 => AmmState::WithdrawTransferState,
            _ => unreachable!(),
        }
    }

    pub fn into_u64(&self) -> u64 {
        match self {
            AmmState::InvlidState => 0u64,
            AmmState::IdleState => 1u64,
            AmmState::CancelAllOrdersState => 2u64,
            AmmState::PlanOrdersState => 3u64,
            AmmState::CancelOrderState => 4u64,
            AmmState::PlaceOrdersState => 5u64,
            AmmState::PurgeOrderState => 6u64,
            AmmState::WithdrawTransferState => 7u64,
        }
    }
    pub fn valid_state(state: u64) -> bool {
        match state {
            0u64 | 1u64 | 2u64 | 3u64 | 4u64 | 5u64 | 6u64 | 7u64 => return true,
            _ => return false,
        }
    }
}

fn validate_fraction(numerator: u64, denominator: u64) -> Result<(), AmmError> {
    if numerator >= denominator || denominator == 0 {
        Err(AmmError::InvalidFee)
    } else {
        Ok(())
    }
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

impl Fees {
    /// Validate that the fees are reasonable
    pub fn validate(&self) -> Result<(), AmmError> {
        validate_fraction(self.min_separate_numerator, self.min_separate_denominator)?;
        validate_fraction(self.trade_fee_numerator, self.trade_fee_denominator)?;
        validate_fraction(self.pnl_numerator, self.pnl_denominator)?;
        validate_fraction(self.swap_fee_numerator, self.swap_fee_denominator)?;
        Ok(())
    }
}

/// IsInitialized is required to use `Pack::pack` and `Pack::unpack`
impl IsInitialized for Fees {
    fn is_initialized(&self) -> bool {
        true
    }
}

impl Sealed for Fees {}
impl Pack for Fees {
    const LEN: usize = 64;
    fn pack_into_slice(&self, output: &mut [u8]) {
        let output = array_mut_ref![output, 0, 64];
        let (
            min_separate_numerator,
            min_separate_denominator,
            trade_fee_numerator,
            trade_fee_denominator,
            pnl_numerator,
            pnl_denominator,
            swap_fee_numerator,
            swap_fee_denominator,
        ) = mut_array_refs![output, 8, 8, 8, 8, 8, 8, 8, 8];
        *min_separate_numerator = self.min_separate_numerator.to_le_bytes();
        *min_separate_denominator = self.min_separate_denominator.to_le_bytes();
        *trade_fee_numerator = self.trade_fee_numerator.to_le_bytes();
        *trade_fee_denominator = self.trade_fee_denominator.to_le_bytes();
        *pnl_numerator = self.pnl_numerator.to_le_bytes();
        *pnl_denominator = self.pnl_denominator.to_le_bytes();
        *swap_fee_numerator = self.swap_fee_numerator.to_le_bytes();
        *swap_fee_denominator = self.swap_fee_denominator.to_le_bytes();
    }

    fn unpack_from_slice(input: &[u8]) -> Result<Fees, ProgramError> {
        let input = array_ref![input, 0, 64];
        #[allow(clippy::ptr_offset_with_cast)]
        let (
            min_separate_numerator,
            min_separate_denominator,
            trade_fee_numerator,
            trade_fee_denominator,
            pnl_numerator,
            pnl_denominator,
            swap_fee_numerator,
            swap_fee_denominator,
        ) = array_refs![input, 8, 8, 8, 8, 8, 8, 8, 8];
        Ok(Self {
            min_separate_numerator: u64::from_le_bytes(*min_separate_numerator),
            min_separate_denominator: u64::from_le_bytes(*min_separate_denominator),
            trade_fee_numerator: u64::from_le_bytes(*trade_fee_numerator),
            trade_fee_denominator: u64::from_le_bytes(*trade_fee_denominator),
            pnl_numerator: u64::from_le_bytes(*pnl_numerator),
            pnl_denominator: u64::from_le_bytes(*pnl_denominator),
            swap_fee_numerator: u64::from_le_bytes(*swap_fee_numerator),
            swap_fee_denominator: u64::from_le_bytes(*swap_fee_denominator),
        })
    }
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
}

impl OutPutData {
    pub fn initialize(&mut self) -> Result<(), AmmError> {
        self.need_take_pnl_coin = 0u64;
        self.need_take_pnl_pc = 0u64;
        self.total_pnl_pc = 0u64;
        self.total_pnl_coin = 0u64;
        self.pool_total_deposit_pc = 1000000000u128;
        self.pool_total_deposit_coin = 1000000000u128;
        self.swap_coin_in_amount = 0u128;
        self.swap_pc_out_amount = 0u128;
        self.swap_coin2pc_fee = 0u64;
        self.swap_pc_in_amount = 0u128;
        self.swap_coin_out_amount = 0u128;
        self.swap_pc2coin_fee = 0u64;
        Ok(())
    }
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
    /// All fee information
    pub fees: Fees,
    /// data calc to output
    pub out_put: OutPutData,
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

#[cfg(target_endian = "little")]
unsafe impl Zeroable for AmmInfo {}
#[cfg(target_endian = "little")]
unsafe impl Pod for AmmInfo {}
#[cfg(target_endian = "little")]
unsafe impl TriviallyTransmutable for AmmInfo {}

impl AmmInfo {
    /// Helper function to get the more efficient packed size of the struct
    #[inline]
    pub fn load_amm_mut<'a>(
        amm_account: &'a AccountInfo,
        check_status: bool,
    ) -> Result<RefMut<'a, AmmInfo>, ProgramError> {
        let account_data: RefMut<'a, [u8]>;
        let amm_data: RefMut<'a, AmmInfo>;

        account_data = RefMut::map(amm_account.try_borrow_mut_data()?, |data| *data);
        amm_data = RefMut::map(account_data, |data| from_bytes_mut(cast_slice_mut(data)));
        if check_status {
            amm_data.check_status()?;
        }
        Ok(amm_data)
    }

    #[inline]
    pub fn check_status(&self) -> Result<bool, ProgramError> {
        if self.status == AmmStatus::Uninitialized as u64 {
            Err(AmmError::InvalidStatus.into())
        } else {
            Ok(true)
        }
    }
}
