//! Instruction types

#![allow(clippy::too_many_arguments)]

use solana_program::{
    instruction::{AccountMeta, Instruction},
    program_error::ProgramError,
    pubkey::Pubkey,
    sysvar,
};
use std::convert::TryInto;
use std::mem::size_of;

solana_program::declare_id!("5quBtoiQqxF9Jv6KYKctB59NT3gtJD2Y65kdnB1Uev3h");

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct InitializeInstruction {
    /// nonce used to create valid program address
    pub nonce: u8,
    /// utc timestamps for pool open
    pub open_time: u64,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct PreInitializeInstruction {
    /// nonce used to create valid program address
    pub nonce: u8,
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
pub struct SwapInstructionBaseIn {
    // SOURCE amount to transfer, output to DESTINATION is based on the exchange rate
    pub amount_in: u64,
    /// Minimum amount of DESTINATION token to output, prevents excessive slippage
    pub minimum_amount_out: u64,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SwapInstructionBaseOut {
    // SOURCE amount to transfer, output to DESTINATION is based on the exchange rate
    pub max_amount_in: u64,
    /// Minimum amount of DESTINATION token to output, prevents excessive slippage
    pub amount_out: u64,
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

    ///   reserved
    Reserved,

    ///   reserved 1
    Reserved1,

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

    ///   reserved 2
    Reserved2,

    ///   reserved 3
    Reserved3,

    ///   reserved 4
    Reserved4,

    ///   reserved 5
    Reserved5,

    /// Swap coin or pc from pool, base amount_in with a slippage of minimum_amount_out
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
    SwapBaseIn(SwapInstructionBaseIn),

    PreInitialize(PreInitializeInstruction),

    /// Swap coin or pc from pool, base amount_out with a slippage of max_amount_in
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
    SwapBaseOut(SwapInstructionBaseOut),

    ///   reserved 6
    Reserved6,
}

impl AmmInstruction {
    /// Unpacks a byte buffer into a [AmmInstruction](enum.AmmInstruction.html).
    pub fn unpack(input: &[u8]) -> Result<Self, ProgramError> {
        let (&tag, rest) = input
            .split_first()
            .ok_or(ProgramError::InvalidInstructionData)?;
        Ok(match tag {
            0 => {
                let (nonce, rest) = Self::unpack_u8(rest)?;
                let (open_time, _reset) = Self::unpack_u64(rest)?;
                Self::Initialize(InitializeInstruction { nonce, open_time })
            }
            3 => {
                let (max_coin_amount, rest) = Self::unpack_u64(rest)?;
                let (max_pc_amount, rest) = Self::unpack_u64(rest)?;
                let (base_side, _rest) = Self::unpack_u64(rest)?;
                Self::Deposit(DepositInstruction {
                    max_coin_amount,
                    max_pc_amount,
                    base_side,
                })
            }
            4 => {
                let (amount, _rest) = Self::unpack_u64(rest)?;
                Self::Withdraw(WithdrawInstruction { amount })
            }

            9 => {
                let (amount_in, rest) = Self::unpack_u64(rest)?;
                let (minimum_amount_out, _rest) = Self::unpack_u64(rest)?;
                Self::SwapBaseIn(SwapInstructionBaseIn {
                    amount_in,
                    minimum_amount_out,
                })
            }
            10 => {
                let (nonce, _rest) = Self::unpack_u8(rest)?;
                Self::PreInitialize(PreInitializeInstruction { nonce })
            }
            11 => {
                let (max_amount_in, rest) = Self::unpack_u64(rest)?;
                let (amount_out, _rest) = Self::unpack_u64(rest)?;
                Self::SwapBaseOut(SwapInstructionBaseOut {
                    max_amount_in,
                    amount_out,
                })
            }
            _ => return Err(ProgramError::InvalidInstructionData.into()),
        })
    }

    fn unpack_u8(input: &[u8]) -> Result<(u8, &[u8]), ProgramError> {
        if input.len() >= 1 {
            let (amount, rest) = input.split_at(1);
            let amount = amount
                .get(..1)
                .and_then(|slice| slice.try_into().ok())
                .map(u8::from_le_bytes)
                .ok_or(ProgramError::InvalidInstructionData)?;
            Ok((amount, rest))
        } else {
            Err(ProgramError::InvalidInstructionData.into())
        }
    }

    fn unpack_u64(input: &[u8]) -> Result<(u64, &[u8]), ProgramError> {
        if input.len() >= 8 {
            let (amount, rest) = input.split_at(8);
            let amount = amount
                .get(..8)
                .and_then(|slice| slice.try_into().ok())
                .map(u64::from_le_bytes)
                .ok_or(ProgramError::InvalidInstructionData)?;
            Ok((amount, rest))
        } else {
            Err(ProgramError::InvalidInstructionData.into())
        }
    }

    /// Packs a [AmmInstruction](enum.AmmInstruction.html) into a byte buffer.
    pub fn pack(&self) -> Result<Vec<u8>, ProgramError> {
        let mut buf = Vec::with_capacity(size_of::<Self>());
        match &*self {
            Self::Initialize(InitializeInstruction { nonce, open_time }) => {
                buf.push(0);
                buf.push(*nonce);
                buf.extend_from_slice(&open_time.to_le_bytes());
            }

            Self::Deposit(DepositInstruction {
                max_coin_amount,
                max_pc_amount,
                base_side,
            }) => {
                buf.push(3);
                buf.extend_from_slice(&max_coin_amount.to_le_bytes());
                buf.extend_from_slice(&max_pc_amount.to_le_bytes());
                buf.extend_from_slice(&base_side.to_le_bytes());
            }
            Self::Withdraw(WithdrawInstruction { amount }) => {
                buf.push(4);
                buf.extend_from_slice(&amount.to_le_bytes());
            }

            Self::SwapBaseIn(SwapInstructionBaseIn {
                amount_in,
                minimum_amount_out,
            }) => {
                buf.push(9);
                buf.extend_from_slice(&amount_in.to_le_bytes());
                buf.extend_from_slice(&minimum_amount_out.to_le_bytes());
            }
            Self::PreInitialize(PreInitializeInstruction { nonce }) => {
                buf.push(10);
                buf.push(*nonce);
            }
            Self::SwapBaseOut(SwapInstructionBaseOut {
                max_amount_in,
                amount_out,
            }) => {
                buf.push(11);
                buf.extend_from_slice(&max_amount_in.to_le_bytes());
                buf.extend_from_slice(&amount_out.to_le_bytes());
            }

            _ => return Err(ProgramError::InvalidInstructionData.into()),
        }
        Ok(buf)
    }
}

/// Unpacks a reference from a bytes buffer.
pub fn unpack_buffer<T>(input: &[u8]) -> Result<&T, ProgramError> {
    if input.len() < size_of::<T>() {
        return Err(ProgramError::InvalidAccountData);
    }
    #[allow(clippy::cast_ptr_alignment)]
    let val: &T = unsafe { &*(&input[0] as *const u8 as *const T) };
    Ok(val)
}

/// Creates an 'preinitialize' instruction.
pub fn pre_initialize(
    program_id: &Pubkey,
    amm_target_orders: &Pubkey,
    amm_authority: &Pubkey,
    amm_lp_mint: &Pubkey,
    amm_coin_mint: &Pubkey,
    amm_pc_mint: &Pubkey,
    amm_coin_vault: &Pubkey,
    amm_pc_vault: &Pubkey,
    serum_market: &Pubkey,
    user_wallet: &Pubkey,

    nonce: u8,
) -> Result<Instruction, ProgramError> {
    let init_data = AmmInstruction::PreInitialize(PreInitializeInstruction { nonce });
    let data = init_data.pack()?;

    let accounts = vec![
        // spl token
        AccountMeta::new_readonly(spl_token::id(), false),
        AccountMeta::new_readonly(solana_program::system_program::id(), false),
        AccountMeta::new_readonly(sysvar::rent::id(), false),
        // amm account
        AccountMeta::new(*amm_target_orders, false),
        AccountMeta::new_readonly(*amm_authority, false),
        AccountMeta::new(*amm_lp_mint, false),
        AccountMeta::new_readonly(*amm_coin_mint, false),
        AccountMeta::new_readonly(*amm_pc_mint, false),
        AccountMeta::new(*amm_coin_vault, false),
        AccountMeta::new(*amm_pc_vault, false),
        // serum
        AccountMeta::new_readonly(*serum_market, false),
        // user wallet
        AccountMeta::new(*user_wallet, true),
    ];

    Ok(Instruction {
        program_id: *program_id,
        accounts,
        data,
    })
}

/// Creates an 'initialize' instruction.
pub fn initialize(
    program_id: &Pubkey,
    amm_id: &Pubkey,
    amm_authority: &Pubkey,
    amm_open_orders: &Pubkey,
    amm_lp_mint: &Pubkey,
    amm_coin_mint: &Pubkey,
    amm_pc_mint: &Pubkey,
    amm_coin_vault: &Pubkey,
    amm_pc_vault: &Pubkey,
    amm_target_orders: &Pubkey,
    model_data_account: &Pubkey,
    serum_program_id: &Pubkey,
    serum_market: &Pubkey,
    user_dest_lp_token: &Pubkey,
    user_wallet: &Pubkey,
    srm_token: Option<Pubkey>,

    nonce: u8,
    open_time: u64,
) -> Result<Instruction, ProgramError> {
    let init_data = AmmInstruction::Initialize(InitializeInstruction { nonce, open_time });
    let data = init_data.pack()?;

    let mut accounts = vec![
        // spl token
        AccountMeta::new_readonly(spl_token::id(), false),
        AccountMeta::new_readonly(solana_program::system_program::id(), false),
        AccountMeta::new_readonly(sysvar::rent::id(), false),
        // amm
        AccountMeta::new(*amm_id, false),
        AccountMeta::new_readonly(*amm_authority, false),
        AccountMeta::new(*amm_open_orders, false),
        AccountMeta::new(*amm_lp_mint, false),
        AccountMeta::new_readonly(*amm_coin_mint, false),
        AccountMeta::new_readonly(*amm_pc_mint, false),
        AccountMeta::new_readonly(*amm_coin_vault, false),
        AccountMeta::new_readonly(*amm_pc_vault, false),
        AccountMeta::new(*amm_target_orders, false),
        AccountMeta::new_readonly(*model_data_account, false),
        // serum
        AccountMeta::new_readonly(*serum_program_id, false),
        AccountMeta::new_readonly(*serum_market, false),
        // user wallet
        AccountMeta::new(*user_dest_lp_token, false),
        AccountMeta::new(*user_wallet, true),
    ];
    if let Some(srm_token_key) = srm_token {
        accounts.push(AccountMeta::new(srm_token_key, false))
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
    amm_id: &Pubkey,
    amm_authority: &Pubkey,
    amm_open_orders: &Pubkey,
    amm_target_orders: &Pubkey,
    amm_lp_mint: &Pubkey,
    amm_coin_vault: &Pubkey,
    amm_pc_vault: &Pubkey,
    model_data_account: &Pubkey,
    serum_market: &Pubkey,
    user_source_coin_token: &Pubkey,
    user_source_pc_token: &Pubkey,
    user_dest_lp_token: &Pubkey,
    user_owner: &Pubkey,

    max_coin_amount: u64,
    max_pc_amount: u64,
    base_side: u64,
) -> Result<Instruction, ProgramError> {
    let data = AmmInstruction::Deposit(DepositInstruction {
        max_coin_amount,
        max_pc_amount,
        base_side,
    })
    .pack()?;

    let accounts = vec![
        // spl token
        AccountMeta::new_readonly(spl_token::id(), false),
        // amm
        AccountMeta::new(*amm_id, false),
        AccountMeta::new_readonly(*amm_authority, false),
        AccountMeta::new_readonly(*amm_open_orders, false),
        AccountMeta::new(*amm_target_orders, false),
        AccountMeta::new(*amm_lp_mint, false),
        AccountMeta::new(*amm_coin_vault, false),
        AccountMeta::new(*amm_pc_vault, false),
        AccountMeta::new_readonly(*model_data_account, false),
        // serum
        AccountMeta::new_readonly(*serum_market, false),
        // user
        AccountMeta::new(*user_source_coin_token, false),
        AccountMeta::new(*user_source_pc_token, false),
        AccountMeta::new(*user_dest_lp_token, false),
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
    amm_id: &Pubkey,
    amm_authority: &Pubkey,
    amm_open_orders: &Pubkey,
    amm_target_orders: &Pubkey,
    amm_lp_mint: &Pubkey,
    amm_coin_vault: &Pubkey,
    amm_pc_vault: &Pubkey,
    model_data_account: &Pubkey,
    serum_program_id: &Pubkey,
    serum_market: &Pubkey,
    serum_coin_vault: &Pubkey,
    serum_pc_vault: &Pubkey,
    serum_vault_signer: &Pubkey,
    user_source_lp_token: &Pubkey,
    user_dest_coin_token: &Pubkey,
    user_dest_pc_token: &Pubkey,
    user_owner: &Pubkey,

    referrer_pc_account: Option<&Pubkey>,
    serum_event_q: Option<&Pubkey>,
    serum_bids: Option<&Pubkey>,
    serum_asks: Option<&Pubkey>,

    amount: u64,
) -> Result<Instruction, ProgramError> {
    let data = AmmInstruction::Withdraw(WithdrawInstruction { amount }).pack()?;

    let mut accounts = vec![
        // spl token
        AccountMeta::new_readonly(spl_token::id(), false),
        // amm
        AccountMeta::new(*amm_id, false),
        AccountMeta::new_readonly(*amm_authority, false),
        AccountMeta::new(*amm_open_orders, false),
        AccountMeta::new(*amm_target_orders, false),
        AccountMeta::new(*amm_lp_mint, false),
        AccountMeta::new(*amm_coin_vault, false),
        AccountMeta::new(*amm_pc_vault, false),
        AccountMeta::new_readonly(*model_data_account, false),
        // serum
        AccountMeta::new_readonly(*serum_program_id, false),
        AccountMeta::new(*serum_market, false),
        AccountMeta::new(*serum_coin_vault, false),
        AccountMeta::new(*serum_pc_vault, false),
        AccountMeta::new_readonly(*serum_vault_signer, false),
        // user
        AccountMeta::new(*user_source_lp_token, false),
        AccountMeta::new(*user_dest_coin_token, false),
        AccountMeta::new(*user_dest_pc_token, false),
        AccountMeta::new_readonly(*user_owner, true),
    ];

    if let Some(referrer_pc_key) = referrer_pc_account {
        accounts.push(AccountMeta::new(*referrer_pc_key, false));
    }
    if let (Some(serum_event_q_key), Some(serum_bids_key), Some(serum_asks_key)) =
        (serum_event_q, serum_bids, serum_asks)
    {
        accounts.push(AccountMeta::new(*serum_event_q_key, false));
        accounts.push(AccountMeta::new(*serum_bids_key, false));
        accounts.push(AccountMeta::new(*serum_asks_key, false));
    }

    Ok(Instruction {
        program_id: *program_id,
        accounts,
        data,
    })
}

/// Creates a 'swap base in' instruction.
pub fn swap_base_in(
    program_id: &Pubkey,
    amm_id: &Pubkey,
    amm_authority: &Pubkey,
    amm_open_orders: &Pubkey,
    amm_coin_vault: &Pubkey,
    amm_pc_vault: &Pubkey,
    model_data_account: &Pubkey,
    serum_program_id: &Pubkey,
    serum_market: &Pubkey,
    serum_bids: &Pubkey,
    serum_asks: &Pubkey,
    serum_event_queue: &Pubkey,
    serum_coin_vault: &Pubkey,
    serum_pc_vault: &Pubkey,
    serum_vault_signer: &Pubkey,
    user_source_token: &Pubkey,
    user_destination_token: &Pubkey,
    user_source_owner: &Pubkey,

    amount_in: u64,
    minimum_amount_out: u64,
) -> Result<Instruction, ProgramError> {
    let data = AmmInstruction::SwapBaseIn(SwapInstructionBaseIn {
        amount_in,
        minimum_amount_out,
    })
    .pack()?;

    let accounts = vec![
        // spl token
        AccountMeta::new_readonly(spl_token::id(), false),
        // amm
        AccountMeta::new(*amm_id, false),
        AccountMeta::new_readonly(*amm_authority, false),
        AccountMeta::new(*amm_open_orders, false),
        AccountMeta::new(*amm_coin_vault, false),
        AccountMeta::new(*amm_pc_vault, false),
        AccountMeta::new_readonly(*model_data_account, false),
        // serum
        AccountMeta::new_readonly(*serum_program_id, false),
        AccountMeta::new(*serum_market, false),
        AccountMeta::new(*serum_bids, false),
        AccountMeta::new(*serum_asks, false),
        AccountMeta::new(*serum_event_queue, false),
        AccountMeta::new(*serum_coin_vault, false),
        AccountMeta::new(*serum_pc_vault, false),
        AccountMeta::new_readonly(*serum_vault_signer, false),
        // user
        AccountMeta::new(*user_source_token, false),
        AccountMeta::new(*user_destination_token, false),
        AccountMeta::new_readonly(*user_source_owner, true),
    ];

    Ok(Instruction {
        program_id: *program_id,
        accounts,
        data,
    })
}

/// Creates a 'swap base out' instruction.
pub fn swap_base_out(
    program_id: &Pubkey,
    amm_id: &Pubkey,
    amm_authority: &Pubkey,
    amm_open_orders: &Pubkey,
    amm_coin_vault: &Pubkey,
    amm_pc_vault: &Pubkey,
    model_data_account: &Pubkey,
    serum_program_id: &Pubkey,
    serum_market: &Pubkey,
    serum_bids: &Pubkey,
    serum_asks: &Pubkey,
    serum_event_queue: &Pubkey,
    serum_coin_vault: &Pubkey,
    serum_pc_vault: &Pubkey,
    serum_vault_signer: &Pubkey,
    user_source_token: &Pubkey,
    user_destination_token: &Pubkey,
    user_source_owner: &Pubkey,

    max_amount_in: u64,
    amount_out: u64,
) -> Result<Instruction, ProgramError> {
    let data = AmmInstruction::SwapBaseOut(SwapInstructionBaseOut {
        max_amount_in,
        amount_out,
    })
    .pack()?;

    let accounts = vec![
        // spl token
        AccountMeta::new_readonly(spl_token::id(), false),
        // amm
        AccountMeta::new(*amm_id, false),
        AccountMeta::new_readonly(*amm_authority, false),
        AccountMeta::new(*amm_open_orders, false),
        AccountMeta::new(*amm_coin_vault, false),
        AccountMeta::new(*amm_pc_vault, false),
        AccountMeta::new_readonly(*model_data_account, false),
        // serum
        AccountMeta::new_readonly(*serum_program_id, false),
        AccountMeta::new(*serum_market, false),
        AccountMeta::new(*serum_bids, false),
        AccountMeta::new(*serum_asks, false),
        AccountMeta::new(*serum_event_queue, false),
        AccountMeta::new(*serum_coin_vault, false),
        AccountMeta::new(*serum_pc_vault, false),
        AccountMeta::new_readonly(*serum_vault_signer, false),
        // user
        AccountMeta::new(*user_source_token, false),
        AccountMeta::new(*user_destination_token, false),
        AccountMeta::new_readonly(*user_source_owner, true),
    ];

    Ok(Instruction {
        program_id: *program_id,
        accounts,
        data,
    })
}
