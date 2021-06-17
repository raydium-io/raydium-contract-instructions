//! Instruction types

#![allow(clippy::too_many_arguments)]

use crate::error::AmmError;
use crate::state::{Fees, AmmParams};
use solana_program::{
    instruction::{AccountMeta, Instruction},
    program_error::ProgramError,
    pubkey::Pubkey,
    program_pack::Pack,
};
use std::convert::TryInto;
use std::mem::size_of;
use arrayref::{array_ref};

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct InitializeInstruction {
    /// nonce used to create valid program address
    pub nonce: u8,
    /// max order count
    pub order_num: u8,
    /// within this range, 5 => 5% range
    pub depth: u8,
    /// 1->1000000
    pub min_size: u32,
    /// 1->1000000
    pub vol_max_cut_ratio: u32,
    /// 1->1000000
    pub amount_wave: u64,
    /// min_cur_price: (2 * amm.order_num * amm.pc_lot_size) * min_price_multiplier
    pub min_price_multiplier: u64,
    /// max_cur_price: (2 * amm.order_num * amm.pc_lot_size) * max_price_multiplier
    pub max_price_multiplier: u64,
    /// system decimal value, used to normalize the value of coin and pc amount
    pub sys_decimal_value: u64,
    /// All fee information
    pub fees: Fees,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Initialize2Instruction {
    /// nonce used to create valid program address
    pub nonce: u8,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct MonitorStepInstruction {
    /// max value of plan/new/cancel orders
    pub plan_order_limit: u16,
    pub place_order_limit: u16,
    pub cancel_order_limit: u16,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DepositInstruction {
    /// Pool token amount to transfer. token_a and token_b amount are set by
    /// the current exchange rate and size of the pool
    pub max_coin_amount: u64,
    pub max_pc_amount: u64,
    pub base_side: u64,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct WithdrawInstruction {
    /// Pool token amount to transfer. token_a and token_b amount are set by
    /// the current exchange rate and size of the pool
    pub amount: u64,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct WithdrawTransferInstruction {
    pub limit: u16,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SetParamsInstruction {
    pub param: u8,
    pub value: Option<u64>,
    pub new_pubkey: Option<Pubkey>,
    pub fees: Option<Fees>,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct WithdrawSrmInstruction {
    pub amount: u64,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SwapInstruction {
    // SOURCE amount to transfer, output to DESTINATION is based on the exchange rate
    pub amount_in: u64,
    /// Minimum amount of DESTINATION token to output, prevents excessive slippage
    pub minimum_amount_out: u64,
}

/// Instructions supported by the AmmInfo program.
#[repr(C)]
#[derive(Clone, Debug, PartialEq)]
pub enum AmmInstruction {
    ///   Initializes a new AmmInfo.
    ///
    ///   0. `[]` Spl Token program id
    ///   1. `[writable, signer]` New amm Account to create.
    ///   2. `[]` $authority derived from `create_program_address(&[amm Account])`
    ///   3. `[]` amm open_orders Account
    ///   4. `[writable]` pool lp mint address. Must be empty, owned by $authority.
    ///   5. `[]` coin mint address
    ///   6. `[]` pc mint address
    ///   7. `[]` pool_token_coin Account. Must be non zero, owned by $authority.
    ///   8. `[]` pool_token_pc Account. Must be non zero, owned by $authority.
    ///   9. '[writable]` withdraw queue Account. To save withdraw dest_coin & dest_pc account with must cancle orders.
    ///   10. `[writable]` token_dest_lp Account. To deposit the initial pool token supply, user is the owner.
    ///   11. `[writable]` token_temp_lp Account. To save withdraw lp with must cancle orders as temp to transfer later.
    ///   12. `[]` serum dex program id
    ///   13. `[]` serum market Account. serum_dex program is the owner.
    Initialize(InitializeInstruction),

    ///   Continue Initializes the new AmmInfo.
    ///
    ///   0. `[]` Spl Token program id
    ///   1. `[]` Rent program id
    ///   2. `[writable, signer]` Continue to init amm Account.
    ///   3. `[]` $authority derived from `create_program_address(&[amm Account])`
    ///   4. `[writable]` amm open_orders Account
    ///   5. `[writable]` pool_token_coin Account. Must be non zero, owned by $authority.
    ///   6. `[writable]` pool_token_pc Account. Must be non zero, owned by $authority.
    ///   7. `[writable]` amm target_orders Account. To store plan orders infomations.
    ///   8. `[]` serum dex program id
    ///   9. `[writable]` serum market Account. serum_dex program is the owner.
    ///   10. `[writable]` coin_vault Account
    ///   11. `[writable]` pc_vault Account
    ///   12. '[writable]` req_q Account
    ///   13. `[writable]` event_q Account
    ///   14. `[writable]` bids Account
    ///   15. `[writable]` asks Account
    Initialize2(Initialize2Instruction),

    ///   MonitorStep. To monitor state turn around step by step.
    ///
    ///   0. `[]` Spl Token program id
    ///   1. `[]` Spl Rent id
    ///   2. `[]` Spl Clock id
    ///   3. `[writable]`amm Account
    ///   4. `[]` $authority derived from `create_program_address(&[amm Account])`
    ///   5. `[writable]` amm open_orders Account
    ///   6. `[writable]` amm target_orders Account. To store plan orders infomations.
    ///   7. `[writable]` pool_token_coin Account. Must be non zero, owned by $authority.
    ///   8. `[writable]` pool_token_pc Account. Must be non zero, owned by $authority.
    ///   9. '[writable]` withdraw queue Account. To save withdraw dest_coin & dest_pc account with must cancle orders.
    ///   10. `[]` serum dex program id
    ///   11. `[writable]` serum market Account. serum_dex program is the owner.
    ///   12. `[writable]` coin_vault Account
    ///   13. `[writable]` pc_vault Account
    ///   14. '[]` vault_signer Account
    ///   15. '[writable]` req_q Account
    ///   16. `[writable]` event_q Account
    ///   17. `[writable]` bids Account
    ///   18. `[writable]` asks Account
    ///   19. `[writable]` (optional) the (M)SRM account used for fee discounts
    MonitorStep(MonitorStepInstruction),

    ///   Deposit some tokens into the pool.  The output is a "pool" token representing ownership
    ///   into the pool. Inputs are converted to the current ratio.
    ///
    ///   0. `[]` Spl Token program id
    ///   1. `[writable]` amm Account
    ///   2. `[]` $authority
    ///   3. `[]` amm open_orders Account
    ///   4. `[writable]` amm target_orders Account. To store plan orders infomations.
    ///   5. `[writable]` pool lp mint address. Must be empty, owned by $authority.
    ///   6. `[writable]` pool_token_coin $authority can transfer amount,
    ///   7. `[writable]` pool_token_pc $authority can transfer amount,
    ///   8. `[]` serum market Account. serum_dex program is the owner.
    ///   9. `[writable]` user coin token Base Account to deposit into.
    ///   10. `[writable]` user pc token Base Account to deposit into.
    ///   11. `[writable]` user lp token. To deposit the generated tokens, user is the owner.
    ///   12. '[signer]` user owner Account
    Deposit(DepositInstruction),

    ///   Withdraw the token from the pool at the current ratio.
    ///
    ///   0. `[]` Spl Token program id
    ///   1. `[writable]` amm Account
    ///   2. `[]` $authority
    ///   3. `[writable]` amm open_orders Account
    ///   4. `[writable]` amm target_orders Account
    ///   5. `[writable]` pool lp mint address. Must be empty, owned by $authority.
    ///   6. `[writable]` pool_token_coin Amm Account to withdraw FROM,
    ///   7. `[writable]` pool_token_pc Amm Account to withdraw FROM,
    ///   8. `[writable]` withdraw queue Account
    ///   9. `[writable]` token_temp_lp Account
    ///   10. `[]` serum dex program id
    ///   11. `[writable]` serum market Account. serum_dex program is the owner.
    ///   12. `[writable]` coin_vault Account
    ///   13. `[writable]` pc_vault Account
    ///   14. '[]` vault_signer Account
    ///   15. `[writable]` user lp token Account. Source lp, amount is transferable by $authority.
    ///   16. `[writable]` user token coin Account. user Account to credit.
    ///   17. `[writable]` user token pc Account. user Account to credit.
    ///   18. `[singer]` user owner Account
    Withdraw(WithdrawInstruction),

    ///   Withdraw the token from the temp_pool at the current ratio.
    ///
    ///   0. `[]` Spl Token program id
    ///   1. `[writable]` amm Account
    ///   2. `[]` $authority
    ///   3. `[writable]` amm open_orders Account
    ///   4. `[writable]` pool lp mint address. Must be empty, owned by $authority.
    ///   5. `[writable]` pool_token_coin Amm Account to withdraw FROM,
    ///   6. `[writable]` pool_token_pc Amm Account to withdraw FROM,
    ///   7. `[writable]` withdraw queue Account
    ///   8. `[writable]` token_temp_lp Account
    ///   9. `[]` serum dex program id
    ///   10. `[writable]` serum market Account. serum_dex program is the owner.
    ///   11. `[writable]` coin_vault Account
    ///   12. `[writable]` pc_vault Account
    ///   13. '[]` vault_signer Account
    WithdrawTransfer(WithdrawTransferInstruction),

    ///   Set amm params
    ///
    ///   0. `[writable]` amm Account.
    ///   1. `[]` $authority derived from `create_program_address(&[amm Account])`
    ///   2. `[singer]` amm Account owner
    ///   3. `[]` (optional) the account to replace owner
    SetParams(SetParamsInstruction),

    ///   Withdraw Pnl from pool
    ///
    ///   0. `[]` Spl Token program id
    ///   1. `[writable]` amm Account
    ///   2. `[]` $authority
    ///   3. `[writable]` amm open_orders Account
    ///   4. `[writable]` pool_token_coin Amm Account to withdraw FROM,
    ///   5. `[writable]` pool_token_pc Amm Account to withdraw FROM,
    ///   6. `[writable]` coin pnl token Account to withdraw to
    ///   7. `[writable]` pc pnl token Account to withdraw to
    ///   8. `[singer]` pnl account owner
    ///   9. `[writable]` amm target_orders Account
    ///   10. `[]` serum dex program id
    ///   11. `[writable]` serum market Account. serum_dex program is the owner.
    ///   12. `[writable]` coin_vault Account
    ///   13. `[writable]` pc_vault Account
    ///   14. '[]` vault_signer Account
    WithdrawPnl,

    ///   Withdraw (M)SRM from the (M)SRM Account used for fee discounts
    ///
    ///   0. `[]` Spl Token program id
    ///   1. `[]` amm Account.
    ///   2. `[singer]` amm Account owner
    ///   3. `[]` $authority derived from `create_program_address(&[amm Account])`
    ///   4. `[writable]` the (M)SRM Account withdraw from
    ///   5. `[writable]` the (M)SRM Account withdraw to
    WithdrawSrm(WithdrawSrmInstruction),

    /// Swap coin or pc from pool
    ///
    ///   0. `[]` Spl Token program id
    ///   1. `[writable]` amm Account
    ///   2. `[]` $authority
    ///   3. `[writable]` amm open_orders Account
    ///   4. `[writable]` amm target_orders Account
    ///   5. `[writable]` pool_token_coin Amm Account to swap FROM or To,
    ///   6. `[writable]` pool_token_pc Amm Account to swap FROM or To,
    ///   7. `[]` serum dex program id
    ///   8. `[writable]` serum market Account. serum_dex program is the owner.
    ///   9. `[writable]` bids Account
    ///   10. `[writable]` asks Account
    ///   11. `[writable]` event_q Account
    ///   12. `[writable]` coin_vault Account
    ///   13. `[writable]` pc_vault Account
    ///   14. '[]` vault_signer Account
    ///   15. `[writable]` user source token Account. user Account to swap from.
    ///   16. `[writable]` user destination token Account. user Account to swap to.
    ///   17. `[singer]` user owner Account
    Swap(SwapInstruction),
}

impl AmmInstruction {
    /// Unpacks a byte buffer into a [AmmInstruction](enum.AmmInstruction.html).
    pub fn unpack(input: &[u8]) -> Result<Self, ProgramError> {
        let (&tag, rest) = input.split_first().ok_or(AmmError::InvalidInstruction)?;
        Ok(match tag {
            0 => {
                let (nonce, rest) = Self::unpack_u8(rest)?;
                let (order_num, rest) = Self::unpack_u8(rest)?;
                let (depth, rest) = Self::unpack_u8(rest)?;
                let (min_size, rest) = Self::unpack_u32(rest)?;
                let (vol_max_cut_ratio, rest) = Self::unpack_u32(rest)?;
                let (amount_wave, rest) = Self::unpack_u64(rest)?;
                let (min_price_multiplier, rest) = Self::unpack_u64(rest)?;
                let (max_price_multiplier, rest) = Self::unpack_u64(rest)?;
                let (sys_decimal_value, rest) = Self::unpack_u64(rest)?;
                if rest.len() >= Fees::LEN {
                    let (fees, _rest) = rest.split_at(Fees::LEN);
                    let fees = Fees::unpack_unchecked(fees)?;
                    Self::Initialize(InitializeInstruction{
                            nonce,
                            order_num,
                            depth,
                            min_size,
                            vol_max_cut_ratio,
                            amount_wave,
                            min_price_multiplier,
                            max_price_multiplier,
                            sys_decimal_value,
                            fees,
                        })
                }
                else {
                    return Err(AmmError::InvalidInstruction.into());
                }
            }
            1 => {
                let (nonce, _rest) = Self::unpack_u8(rest)?;
                Self::Initialize2(Initialize2Instruction{ nonce })
            }
            2 => {
                let (plan_order_limit, rest) = Self::unpack_u16(rest)?;
                let (place_order_limit, rest) = Self::unpack_u16(rest)?;
                let (cancel_order_limit, _rest) = Self::unpack_u16(rest)?;
                Self::MonitorStep(MonitorStepInstruction{plan_order_limit, place_order_limit, cancel_order_limit})
            }
            3  => {
                let (max_coin_amount, rest) = Self::unpack_u64(rest)?;
                let (max_pc_amount, rest) = Self::unpack_u64(rest)?;
                let (base_side, _rest) = Self::unpack_u64(rest)?;
                Self::Deposit(DepositInstruction{ max_coin_amount, max_pc_amount, base_side })
            }
            4 => {
                let (amount, _rest) = Self::unpack_u64(rest)?;
                Self::Withdraw(WithdrawInstruction{ amount })
            }
            5 => {
                let (limit, _rest) = Self::unpack_u16(rest)?;
                Self::WithdrawTransfer(WithdrawTransferInstruction{ limit })
            }
            6 => {
                let (param, rest) = Self::unpack_u8(rest)?;
                match AmmParams::from_u64(param as u64) {
                    AmmParams::AmmOwner | AmmParams::PnlOwner => {
                        if rest.len() >= 32 {
                            let new_pubkey = array_ref![rest, 0, 32];
                            Self::SetParams(SetParamsInstruction{param, value: None, new_pubkey:Some(Pubkey::new_from_array(*new_pubkey)), fees: None})
                        }
                        else {
                            return Err(AmmError::InvalidInstruction.into());
                        }
                    },
                    AmmParams::Fees => {
                        if rest.len() >= Fees::LEN {
                            let (fees, _rest) = rest.split_at(Fees::LEN);
                            let fees = Fees::unpack_from_slice(fees)?;
                            Self::SetParams(SetParamsInstruction{param, value: None, new_pubkey:None, fees: Some(fees)})
                        }
                        else {
                            return Err(AmmError::InvalidInstruction.into());
                        }
                    },
                    _ => {
                        if rest.len() >= 8 {
                            let (value, _rest) = Self::unpack_u64(rest)?;
                            Self::SetParams(SetParamsInstruction{param, value: Some(value), new_pubkey:None, fees:None})
                        }
                        else {
                            return Err(AmmError::InvalidInstruction.into());
                        }
                    }
                }
            }
            7 => {
                Self::WithdrawPnl
            }
            8 => {
                let (amount, _rest) = Self::unpack_u64(rest)?;
                Self::WithdrawSrm(WithdrawSrmInstruction{ amount })
            }
            9 => {
                let (amount_in, rest) = Self::unpack_u64(rest)?;
                let (minimum_amount_out, _rest) = Self::unpack_u64(rest)?;
                Self::Swap(SwapInstruction{amount_in, minimum_amount_out})
            }
            _ => return Err(AmmError::InvalidInstruction.into()),
        })
    }

    fn unpack_u8(input: &[u8]) -> Result<(u8, &[u8]), ProgramError> {
        if input.len() >= 1 {
            let (amount, rest) = input.split_at(1);
            let amount = amount
                .get(..1)
                .and_then(|slice| slice.try_into().ok())
                .map(u8::from_le_bytes)
                .ok_or(AmmError::InvalidInstruction)?;
            Ok((amount, rest))
        } else {
            Err(AmmError::InvalidInstruction.into())
        }
    }

    fn unpack_u16(input: &[u8]) -> Result<(u16, &[u8]), ProgramError> {
        if input.len() >= 2 {
            let (amount, rest) = input.split_at(2);
            let amount = amount
                .get(..2)
                .and_then(|slice| slice.try_into().ok())
                .map(u16::from_le_bytes)
                .ok_or(AmmError::InvalidInstruction)?;
            Ok((amount, rest))
        } else {
            Err(AmmError::InvalidInstruction.into())
        }
    }

    fn unpack_u32(input: &[u8]) -> Result<(u32, &[u8]), ProgramError> {
        if input.len() >= 4 {
            let (amount, rest) = input.split_at(4);
            let amount = amount
                .get(..4)
                .and_then(|slice| slice.try_into().ok())
                .map(u32::from_le_bytes)
                .ok_or(AmmError::InvalidInstruction)?;
            Ok((amount, rest))
        } else {
            Err(AmmError::InvalidInstruction.into())
        }
    }

    fn unpack_u64(input: &[u8]) -> Result<(u64, &[u8]), ProgramError> {
        if input.len() >= 8 {
            let (amount, rest) = input.split_at(8);
            let amount = amount
                .get(..8)
                .and_then(|slice| slice.try_into().ok())
                .map(u64::from_le_bytes)
                .ok_or(AmmError::InvalidInstruction)?;
            Ok((amount, rest))
        } else {
            Err(AmmError::InvalidInstruction.into())
        }
    }

    /// Packs a [AmmInstruction](enum.AmmInstruction.html) into a byte buffer.
    pub fn pack(&self) -> Result<Vec<u8>, ProgramError> {
        let mut buf = Vec::with_capacity(size_of::<Self>());
        match &*self {
            Self::Initialize(
                InitializeInstruction {
                    nonce,
                    order_num,
                    depth,
                    min_size,
                    vol_max_cut_ratio,
                    amount_wave,
                    min_price_multiplier,
                    max_price_multiplier,
                    sys_decimal_value,
                    fees,
                }
            ) => {
                buf.push(0);
                buf.push(*nonce);
                buf.push(*order_num);
                buf.push(*depth);
                buf.extend_from_slice(&min_size.to_le_bytes());
                buf.extend_from_slice(&vol_max_cut_ratio.to_le_bytes());
                buf.extend_from_slice(&amount_wave.to_le_bytes());
                buf.extend_from_slice(&min_price_multiplier.to_le_bytes());
                buf.extend_from_slice(&max_price_multiplier.to_le_bytes());
                buf.extend_from_slice(&sys_decimal_value.to_le_bytes());
                let mut fees_slice = [0u8; Fees::LEN];
                Pack::pack_into_slice(fees, &mut fees_slice[..]);
                buf.extend_from_slice(&fees_slice);
            }
            Self::Initialize2(Initialize2Instruction{ nonce }) => {
                buf.push(1);
                buf.push(*nonce);
            }
            Self::MonitorStep(MonitorStepInstruction{ plan_order_limit, place_order_limit, cancel_order_limit }) => {
                buf.push(2);
                buf.extend_from_slice(&plan_order_limit.to_le_bytes());
                buf.extend_from_slice(&place_order_limit.to_le_bytes());
                buf.extend_from_slice(&cancel_order_limit.to_le_bytes());
            }
            Self::Deposit(DepositInstruction{ max_coin_amount, max_pc_amount, base_side }) => {
                buf.push(3);
                buf.extend_from_slice(&max_coin_amount.to_le_bytes());
                buf.extend_from_slice(&max_pc_amount.to_le_bytes());
                buf.extend_from_slice(&base_side.to_le_bytes());
            }
            Self::Withdraw(WithdrawInstruction{ amount }) => {
                buf.push(4);
                buf.extend_from_slice(&amount.to_le_bytes());
            }
            Self::WithdrawTransfer(WithdrawTransferInstruction{ limit }) => {
                buf.push(5);
                buf.extend_from_slice(&limit.to_le_bytes());
            }
            Self::SetParams(SetParamsInstruction{param, value, new_pubkey, fees}) => {
                buf.push(6);
                buf.push(*param);
                match AmmParams::from_u64(*param as u64) {
                    AmmParams::AmmOwner | AmmParams::PnlOwner => {
                        let new_pubkey = match new_pubkey {
                            Some(a) => a,
                            None => return Err(AmmError::InvalidInput.into())
                        };
                        buf.extend_from_slice(&new_pubkey.to_bytes());
                    },
                    AmmParams::Fees => {
                        let fees = match fees {
                            Some(a) => a,
                            None => return Err(AmmError::InvalidInput.into())
                        };
                        let mut fees_slice = [0u8; Fees::LEN];
                        Pack::pack_into_slice(fees, &mut fees_slice[..]);
                        buf.extend_from_slice(&fees_slice);
                    },
                    _ => {
                        let value = match value {
                            Some(a) => a,
                            None => return Err(AmmError::InvalidInput.into())
                        };
                        buf.extend_from_slice(&value.to_le_bytes());
                    }
                }
            }
            Self::WithdrawPnl => {
                buf.push(7);
            }
            Self::WithdrawSrm(WithdrawSrmInstruction{ amount }) => {
                buf.push(8);
                buf.extend_from_slice(&amount.to_le_bytes());
            }
            Self::Swap(SwapInstruction{amount_in, minimum_amount_out}) => {
                buf.push(9);
                buf.extend_from_slice(&amount_in.to_le_bytes());
                buf.extend_from_slice(&minimum_amount_out.to_le_bytes());
            }
        }
        Ok(buf)
    }
}

/// Creates an 'initialize' instruction.
pub fn initialize(
    program_id: &Pubkey,
    spl_token_program_id: &Pubkey,
    amm_id: &Pubkey,
    amm_authority: &Pubkey,
    amm_open_orders: &Pubkey,
    lp_mint_address: &Pubkey,
    coin_mint_address: &Pubkey,
    pc_mint_address: &Pubkey,
    pool_coin_token_account: &Pubkey,
    pool_pc_token_account: &Pubkey,
    pool_withdraw_queue: &Pubkey,
    pool_lp_token_account: &Pubkey,
    pool_temp_lp_token_account: &Pubkey,
    serum_program_id: &Pubkey,
    serum_market: &Pubkey,

    nonce: u8,
    order_num: u8,
    depth: u8,
    min_size:u32,
    vol_max_cut_ratio: u32,
    amount_wave: u64,
    min_price_multiplier: u64,
    max_price_multiplier: u64,
    sys_decimal_value: u64,
    fees: Fees,
) -> Result<Instruction, ProgramError> {
    let init_data = AmmInstruction::Initialize(
        InitializeInstruction{
            nonce,
            order_num,
            depth,
            min_size,
            vol_max_cut_ratio,
            amount_wave,
            min_price_multiplier,
            max_price_multiplier,
            sys_decimal_value,
            fees
        }
    );
    let data = init_data.pack()?;

    let accounts = vec![
        // spl token
        AccountMeta::new_readonly(*spl_token_program_id, false),
        // amm
        AccountMeta::new(*amm_id, true),
        AccountMeta::new_readonly(*amm_authority, false),
        AccountMeta::new_readonly(*amm_open_orders, false),
        AccountMeta::new(*lp_mint_address, false),
        AccountMeta::new_readonly(*coin_mint_address, false),
        AccountMeta::new_readonly(*pc_mint_address, false),
        AccountMeta::new_readonly(*pool_coin_token_account, false),
        AccountMeta::new_readonly(*pool_pc_token_account, false),
        AccountMeta::new(*pool_withdraw_queue, false),
        AccountMeta::new(*pool_lp_token_account, false),
        AccountMeta::new_readonly(*pool_temp_lp_token_account, false),
        // serum
        AccountMeta::new_readonly(*serum_program_id, false),
        AccountMeta::new_readonly(*serum_market, false),
    ];

    Ok(Instruction {
        program_id: *program_id,
        accounts,
        data,
    })
}

/// Creates an 'initialize2' instruction.
pub fn initialize2(
    program_id: &Pubkey,
    spl_token_program_id: &Pubkey,
    spl_rent_id: &Pubkey,
    amm_id: &Pubkey,
    amm_authority: &Pubkey,
    amm_open_orders: &Pubkey,
    pool_coin_token_account: &Pubkey,
    pool_pc_token_account: &Pubkey,
    amm_target_orders: &Pubkey,
    srm_token_account: Option<Pubkey>,
    serum_program_id: &Pubkey,
    serum_market: &Pubkey,
    serum_coin_vault_account: &Pubkey,
    serum_pc_vault_account: & Pubkey,
    serum_req_q: &Pubkey,
    serum_event_q: &Pubkey,
    serum_bids: &Pubkey,
    serum_asks: &Pubkey,

    nonce: u8,
) -> Result<Instruction, ProgramError> {
    let init_data = AmmInstruction::Initialize2(Initialize2Instruction{ nonce });
    let data = init_data.pack()?;

    let mut accounts = vec![
        // spl
        AccountMeta::new_readonly(*spl_token_program_id, false),
        AccountMeta::new_readonly(*spl_rent_id, false),
        // amm
        AccountMeta::new(*amm_id, true),
        AccountMeta::new_readonly(*amm_authority, false),
        AccountMeta::new(*amm_open_orders, false),
        AccountMeta::new(*pool_coin_token_account, false),
        AccountMeta::new(*pool_pc_token_account, false),
        AccountMeta::new(*amm_target_orders, false),
        // serum
        AccountMeta::new_readonly(*serum_program_id, false),
        AccountMeta::new(*serum_market, false),
        AccountMeta::new(*serum_coin_vault_account, false),
        AccountMeta::new(*serum_pc_vault_account, false),
        AccountMeta::new(*serum_req_q, false),
        AccountMeta::new(*serum_event_q, false),
        AccountMeta::new(*serum_bids, false),
        AccountMeta::new(*serum_asks, false),
    ];

    if let Some(srm_token_key) = srm_token_account {
        accounts.push(AccountMeta::new(srm_token_key, false),)
    }

    Ok(Instruction {
        program_id: *program_id,
        accounts,
        data,
    })
}

/// Creates a 'deposit' instruction.
pub fn deposit(
    program_id: &Pubkey,
    spl_token_program_id: &Pubkey,
    amm_id: &Pubkey,
    amm_authority: &Pubkey,
    amm_open_orders: &Pubkey,
    amm_target_orders: &Pubkey,
    lp_mint_address: &Pubkey,
    pool_coin_token_account: &Pubkey,
    pool_pc_token_account: &Pubkey,
    serum_market: &Pubkey,
    user_coin_token_account: &Pubkey,
    user_pc_token_account: &Pubkey,
    user_lp_token_account: &Pubkey,
    user_owner: &Pubkey,

    max_coin_amount: u64,
    max_pc_amount: u64,
    base_side: u64,
) -> Result<Instruction, ProgramError> {
    let data = AmmInstruction::Deposit(DepositInstruction{ max_coin_amount, max_pc_amount, base_side }).pack()?;

    let accounts = vec![
        // spl token
        AccountMeta::new_readonly(*spl_token_program_id, false),
        // amm
        AccountMeta::new(*amm_id, false),
        AccountMeta::new_readonly(*amm_authority, false),
        AccountMeta::new_readonly(*amm_open_orders, false),
        AccountMeta::new(*amm_target_orders, false),
        AccountMeta::new(*lp_mint_address, false),
        AccountMeta::new(*pool_coin_token_account, false),
        AccountMeta::new(*pool_pc_token_account, false),
        // serum
        AccountMeta::new_readonly(*serum_market, false),
        // user
        AccountMeta::new(*user_coin_token_account, false),
        AccountMeta::new(*user_pc_token_account, false),
        AccountMeta::new(*user_lp_token_account, false),
        AccountMeta::new_readonly(*user_owner, true),
    ];

    Ok(Instruction {
        program_id: *program_id,
        accounts,
        data,
    })
}

/// Creates a 'withdraw' instruction.
pub fn withdraw(
    program_id: &Pubkey,
    spl_token_program_id: &Pubkey,
    amm_id: &Pubkey,
    amm_authority: &Pubkey,
    amm_open_orders: &Pubkey,
    amm_target_orders: &Pubkey,
    lp_mint_address: &Pubkey,
    pool_coin_token_account: &Pubkey,
    pool_pc_token_account: &Pubkey,
    pool_withdraw_queue: &Pubkey,
    pool_temp_lp_token_account: &Pubkey,
    serum_program_id: &Pubkey,
    serum_market: &Pubkey,
    serum_coin_vault_account: &Pubkey,
    serum_pc_vault_account: &Pubkey,
    serum_vault_signer: &Pubkey,
    user_lp_token_account: &Pubkey,
    uer_coin_token_account: &Pubkey,
    uer_pc_token_account: &Pubkey,
    user_owner: &Pubkey,

    amount: u64,
) -> Result<Instruction, ProgramError> {
    let data = AmmInstruction::Withdraw(WithdrawInstruction{ amount }).pack()?;

    let accounts = vec![
        // spl token
        AccountMeta::new_readonly(*spl_token_program_id, false),
        // amm
        AccountMeta::new(*amm_id, false),
        AccountMeta::new_readonly(*amm_authority, false),
        AccountMeta::new(*amm_open_orders, false),
        AccountMeta::new(*amm_target_orders, false),
        AccountMeta::new(*lp_mint_address, false),
        AccountMeta::new(*pool_coin_token_account, false),
        AccountMeta::new(*pool_pc_token_account, false),
        AccountMeta::new(*pool_withdraw_queue, false),
        AccountMeta::new(*pool_temp_lp_token_account, false),
        // serum
        AccountMeta::new_readonly(*serum_program_id, false),
        AccountMeta::new(*serum_market, false),
        AccountMeta::new(*serum_coin_vault_account, false),
        AccountMeta::new(*serum_pc_vault_account, false),
        AccountMeta::new_readonly(*serum_vault_signer, false),
        // user
        AccountMeta::new(*user_lp_token_account, false),
        AccountMeta::new(*uer_coin_token_account, false),
        AccountMeta::new(*uer_pc_token_account, false),
        AccountMeta::new_readonly(*user_owner, true),
    ];

    Ok(Instruction {
        program_id: *program_id,
        accounts,
        data,
    })
}

/// Creates a 'swap' instruction.
pub fn swap(
    program_id: &Pubkey,
    spl_token_program_id: &Pubkey,
    amm_id: &Pubkey,
    amm_authority: &Pubkey,
    amm_open_orders: &Pubkey,
    amm_target_orders: &Pubkey,
    pool_coin_token_account: &Pubkey,
    pool_pc_token_account: &Pubkey,
    serum_program_id: &Pubkey,
    serum_market: &Pubkey,
    serum_bids: &Pubkey,
    serum_asks: &Pubkey,
    serum_event_queue: &Pubkey,
    serum_coin_vault_account: &Pubkey,
    serum_pc_vault_account: &Pubkey,
    serum_vault_signer: &Pubkey,
    uer_source_token_account: &Pubkey,
    uer_destination_token_account: &Pubkey,
    user_source_owner: &Pubkey,

    amount_in: u64,
    minimum_amount_out: u64,
) -> Result<Instruction, ProgramError> {
    let data = AmmInstruction::Swap(SwapInstruction{ amount_in, minimum_amount_out }).pack()?;

    let accounts = vec![
        // spl token
        AccountMeta::new_readonly(*spl_token_program_id, false),
        // amm
        AccountMeta::new(*amm_id, false),
        AccountMeta::new_readonly(*amm_authority, false),
        AccountMeta::new(*amm_open_orders, false),
        AccountMeta::new(*amm_target_orders, false),
        AccountMeta::new(*pool_coin_token_account, false),
        AccountMeta::new(*pool_pc_token_account, false),
        // serum
        AccountMeta::new_readonly(*serum_program_id, false),
        AccountMeta::new(*serum_market, false),
        AccountMeta::new(*serum_bids, false),
        AccountMeta::new(*serum_asks, false),
        AccountMeta::new(*serum_event_queue, false),
        AccountMeta::new(*serum_coin_vault_account, false),
        AccountMeta::new(*serum_pc_vault_account, false),
        AccountMeta::new_readonly(*serum_vault_signer, false),
        // user
        AccountMeta::new(*uer_source_token_account, false),
        AccountMeta::new(*uer_destination_token_account, false),
        AccountMeta::new_readonly(*user_source_owner, true),
    ];

    Ok(Instruction {
        program_id: *program_id,
        accounts,
        data,
    })
}

/// Creates a 'withdraw_transfer' instruction.
pub fn withdraw_transfer(
    program_id: &Pubkey,
    spl_token_program_id: &Pubkey,
    amm_id: &Pubkey,
    amm_authority: &Pubkey,
    amm_open_orders: &Pubkey,
    lp_mint_address: &Pubkey,
    pool_coin_token_account: &Pubkey,
    pool_pc_token_account: &Pubkey,
    pool_withdraw_queue: &Pubkey,
    pool_temp_lp_token_account: &Pubkey,
    serum_program_id: &Pubkey,
    serum_market: &Pubkey,
    serum_coin_vault_account: &Pubkey,
    serum_pc_vault_account: &Pubkey,
    serum_vault_signer: &Pubkey,
    withdraw_dest_pks: &mut Vec<Pubkey>,
    limit: u16,
) -> Result<Instruction, ProgramError> {
    let data = AmmInstruction::WithdrawTransfer(WithdrawTransferInstruction{ limit }).pack()?;

    let mut accounts = vec![
        // spl token
        AccountMeta::new_readonly(*spl_token_program_id, false),
        // amm
        AccountMeta::new(*amm_id, false),
        AccountMeta::new_readonly(*amm_authority, false),
        AccountMeta::new(*amm_open_orders, false),
        AccountMeta::new(*lp_mint_address, false),
        AccountMeta::new(*pool_coin_token_account, false),
        AccountMeta::new(*pool_pc_token_account, false),
        AccountMeta::new(*pool_withdraw_queue, false),
        AccountMeta::new(*pool_temp_lp_token_account, false),
        // serum
        AccountMeta::new_readonly(*serum_program_id, false),
        AccountMeta::new(*serum_market, false),
        AccountMeta::new(*serum_coin_vault_account, false),
        AccountMeta::new(*serum_pc_vault_account, false),
        AccountMeta::new_readonly(*serum_vault_signer, false),
    ];

    for i in 0..withdraw_dest_pks.len() {
        let dest_coin_or_pc_meta = AccountMeta::new(withdraw_dest_pks[i], false);
        accounts.push(dest_coin_or_pc_meta);
    }

    Ok(Instruction {
        program_id: *program_id,
        accounts,
        data,
    })
}

/// Creates a 'withdrawpnl' instruction
pub fn withdrawpnl(
    program_id: &Pubkey,
    spl_token_program_id: &Pubkey,
    amm_id: &Pubkey,
    amm_authority: &Pubkey,
    amm_open_orders: &Pubkey,
    pool_coin_token_account: &Pubkey,
    pool_pc_token_account: &Pubkey,
    coin_pnl_token_account: &Pubkey,
    pc_pnl_token_account: &Pubkey,
    pnl_owner_account: &Pubkey,
    amm_target_orders: &Pubkey,
    serum_program_id: &Pubkey,
    serum_market: &Pubkey,
    serum_coin_vault_account: &Pubkey,
    serum_pc_vault_account: &Pubkey,
    serum_vault_signer: &Pubkey,
) -> Result<Instruction, ProgramError> {
    let data = AmmInstruction::WithdrawPnl.pack()?;

    let accounts = vec![
        // spl token
        AccountMeta::new_readonly(*spl_token_program_id, false),
        // amm
        AccountMeta::new(*amm_id, false),
        AccountMeta::new_readonly(*amm_authority, false),
        AccountMeta::new(*amm_open_orders, false),
        AccountMeta::new(*pool_coin_token_account, false),
        AccountMeta::new(*pool_pc_token_account, false),
        AccountMeta::new(*coin_pnl_token_account, false),
        AccountMeta::new(*pc_pnl_token_account, false),
        AccountMeta::new_readonly(*pnl_owner_account, true),
        AccountMeta::new(*amm_target_orders, false),
        // serum
        AccountMeta::new_readonly(*serum_program_id, false),
        AccountMeta::new(*serum_market, false),
        AccountMeta::new(*serum_coin_vault_account, false),
        AccountMeta::new(*serum_pc_vault_account, false),
        AccountMeta::new_readonly(*serum_vault_signer, false),
    ];

    Ok(Instruction {
        program_id: *program_id,
        accounts,
        data,
    })
}

/// Creates a 'SetParams' instruction.
pub fn set_params(
    program_id: &Pubkey,
    amm_id: &Pubkey,
    amm_authority: &Pubkey,
    amm_owner_account: &Pubkey,
    param: u8,
    value: Option<u64>,
    new_pubkey: Option<Pubkey>,
    fees: Option<Fees>
) -> Result<Instruction, ProgramError> {
    let data = AmmInstruction::SetParams(SetParamsInstruction{param, value, new_pubkey, fees}).pack()?;

    let mut accounts = vec![
        AccountMeta::new(*amm_id, false),
        AccountMeta::new_readonly(*amm_authority, false),
        AccountMeta::new_readonly(*amm_owner_account, true),
    ];
    if let Some(key) = new_pubkey {
        accounts.push(AccountMeta::new_readonly(key, false))
    }
    Ok(Instruction {
        program_id: *program_id,
        accounts,
        data,
    })
}

/// Creates a 'monitor_step' instruction.
pub fn monitor_step(
    program_id: &Pubkey,
    spl_token_program_id: &Pubkey,
    spl_rent_id: &Pubkey,
    clock_id: &Pubkey,
    amm_id: &Pubkey,
    amm_authority: &Pubkey,
    amm_open_orders: &Pubkey,
    amm_target_orders: &Pubkey,
    pool_coin_token_account: &Pubkey,
    pool_pc_token_account: &Pubkey,
    pool_withdraw_queue: &Pubkey,
    srm_token_account: Option<Pubkey>,
    serum_program_id: &Pubkey,
    serum_market: &Pubkey,
    serum_coin_vault_account: &Pubkey,
    serum_pc_vault_account: &Pubkey,
    serum_vault_signer: &Pubkey,
    serum_req_q: &Pubkey,
    serum_event_q: &Pubkey,
    serum_bids: &Pubkey,
    serum_asks: &Pubkey,

    plan_order_limit: u16,
    place_order_limit: u16,
    cancel_order_limit: u16,
) -> Result<Instruction, ProgramError> {
    let data = AmmInstruction::MonitorStep(MonitorStepInstruction{ plan_order_limit, place_order_limit, cancel_order_limit }).pack()?;

    let mut accounts = vec![
        // spl
        AccountMeta::new_readonly(*spl_token_program_id, false),
        AccountMeta::new_readonly(*spl_rent_id, false),
        AccountMeta::new_readonly(*clock_id, false),
        // amm
        AccountMeta::new(*amm_id, false),
        AccountMeta::new_readonly(*amm_authority, false),
        AccountMeta::new(*amm_open_orders, false),
        AccountMeta::new(*amm_target_orders, false),
        AccountMeta::new(*pool_coin_token_account, false),
        AccountMeta::new(*pool_pc_token_account, false),
        AccountMeta::new(*pool_withdraw_queue, false),
        // serum
        AccountMeta::new_readonly(*serum_program_id, false),
        AccountMeta::new(*serum_market, false),
        AccountMeta::new(*serum_coin_vault_account, false),
        AccountMeta::new(*serum_pc_vault_account, false),
        AccountMeta::new_readonly(*serum_vault_signer, false),
        AccountMeta::new(*serum_req_q, false),
        AccountMeta::new(*serum_event_q, false),
        AccountMeta::new(*serum_bids, false),
        AccountMeta::new(*serum_asks, false),
    ];

    if let Some(srm_token_key) = srm_token_account {
        accounts.push(AccountMeta::new(srm_token_key, false),)
    }

    Ok(Instruction {
        program_id: *program_id,
        accounts,
        data,
    })
}

/// Creates a 'withdrawsrm' instruction
pub fn withdrawsrm(
    program_id: &Pubkey,
    spl_token_program_id: &Pubkey,
    amm_id: &Pubkey,
    amm_authority: &Pubkey,
    amm_owner_account: &Pubkey,
    srm_token: &Pubkey,
    dest_srm_token: &Pubkey,
    amount: u64,
) -> Result<Instruction, ProgramError> {
    let data = AmmInstruction::WithdrawSrm(WithdrawSrmInstruction{amount}).pack()?;

    let accounts = vec![
        // spl token
        AccountMeta::new_readonly(*spl_token_program_id, false),
        // amm
        AccountMeta::new_readonly(*amm_id, false),
        AccountMeta::new_readonly(*amm_owner_account, true),
        AccountMeta::new_readonly(*amm_authority, false),
        // serum
        AccountMeta::new(*srm_token, false),
        AccountMeta::new(*dest_srm_token, false),
    ];

    Ok(Instruction {
        program_id: *program_id,
        accounts,
        data,
    })
}

/// Unpacks a reference from a bytes buffer.
/// TODO actually pack / unpack instead of relying on normal memory layout.
pub fn unpack<T>(input: &[u8]) -> Result<&T, ProgramError> {
    if input.len() < size_of::<u8>() + size_of::<T>() {
        return Err(ProgramError::InvalidAccountData);
    }
    #[allow(clippy::cast_ptr_alignment)]
    let val: &T = unsafe { &*(&input[1] as *const u8 as *const T) };
    Ok(val)
}
