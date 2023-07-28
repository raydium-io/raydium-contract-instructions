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

solana_program::declare_id!("675kPX9MHTjS2zt1qfr1NYHuzeLXfQM9H24wFSUt1Mp8");

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct InitializeInstruction {
    /// nonce used to create valid program address
    pub nonce: u8,
    pub open_time: u64,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct InitializeInstruction2 {
    /// nonce used to create valid program address
    pub nonce: u8,
    /// utc timestamps for pool open
    pub open_time: u64,
    /// init token pc amount
    pub init_pc_amount: u64,
    /// init token coin amount
    pub init_coin_amount: u64,
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
    Initialize2(InitializeInstruction2),

    Reserved0,

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

    Reserved1,

    Reserved2,

    Reserved3,

    Reserved4,

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

    Reserved5,
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
                let (open_time, _rest) = Self::unpack_u64(rest)?;
                Self::Initialize(InitializeInstruction { nonce, open_time })
            }
            1 => {
                let (nonce, rest) = Self::unpack_u8(rest)?;
                let (open_time, rest) = Self::unpack_u64(rest)?;
                let (init_pc_amount, rest) = Self::unpack_u64(rest)?;
                let (init_coin_amount, _reset) = Self::unpack_u64(rest)?;
                Self::Initialize2(InitializeInstruction2 {
                    nonce,
                    open_time,
                    init_pc_amount,
                    init_coin_amount,
                })
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
            Self::Initialize2(InitializeInstruction2 {
                nonce,
                open_time,
                init_pc_amount,
                init_coin_amount,
            }) => {
                buf.push(1);
                buf.push(*nonce);
                buf.extend_from_slice(&open_time.to_le_bytes());
                buf.extend_from_slice(&init_pc_amount.to_le_bytes());
                buf.extend_from_slice(&init_coin_amount.to_le_bytes());
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
            _ => {}
        }
        Ok(buf)
    }
}

/// Creates an 'preinitialize' instruction.
pub fn pre_initialize(
    program_id: &Pubkey,
    amm_target_orders: &Pubkey,
    pool_withdraw_queue: &Pubkey,
    amm_authority: &Pubkey,
    lp_mint_address: &Pubkey,
    coin_mint_address: &Pubkey,
    pc_mint_address: &Pubkey,
    pool_coin_token_account: &Pubkey,
    pool_pc_token_account: &Pubkey,
    pool_temp_lp_token_account: &Pubkey,
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
        AccountMeta::new(*pool_withdraw_queue, false),
        AccountMeta::new_readonly(*amm_authority, false),
        AccountMeta::new(*lp_mint_address, false),
        AccountMeta::new_readonly(*coin_mint_address, false),
        AccountMeta::new_readonly(*pc_mint_address, false),
        AccountMeta::new(*pool_coin_token_account, false),
        AccountMeta::new(*pool_pc_token_account, false),
        AccountMeta::new(*pool_temp_lp_token_account, false),
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
    lp_mint_address: &Pubkey,
    coin_mint_address: &Pubkey,
    pc_mint_address: &Pubkey,
    pool_coin_token_account: &Pubkey,
    pool_pc_token_account: &Pubkey,
    pool_withdraw_queue: &Pubkey,
    pool_target_orders_account: &Pubkey,
    pool_lp_token_account: &Pubkey,
    pool_temp_lp_token_account: &Pubkey,
    serum_program_id: &Pubkey,
    serum_market: &Pubkey,
    user_wallet: &Pubkey,

    nonce: u8,
    open_time: u64,
) -> Result<Instruction, ProgramError> {
    let init_data = AmmInstruction::Initialize(InitializeInstruction { nonce, open_time });
    let data = init_data.pack()?;

    let accounts = vec![
        // spl token
        AccountMeta::new_readonly(spl_token::id(), false),
        AccountMeta::new_readonly(solana_program::system_program::id(), false),
        AccountMeta::new_readonly(sysvar::rent::id(), false),
        // amm
        AccountMeta::new(*amm_id, false),
        AccountMeta::new_readonly(*amm_authority, false),
        AccountMeta::new(*amm_open_orders, false),
        AccountMeta::new(*lp_mint_address, false),
        AccountMeta::new_readonly(*coin_mint_address, false),
        AccountMeta::new_readonly(*pc_mint_address, false),
        AccountMeta::new_readonly(*pool_coin_token_account, false),
        AccountMeta::new_readonly(*pool_pc_token_account, false),
        AccountMeta::new(*pool_withdraw_queue, false),
        AccountMeta::new(*pool_target_orders_account, false),
        AccountMeta::new(*pool_lp_token_account, false),
        AccountMeta::new_readonly(*pool_temp_lp_token_account, false),
        // serum
        AccountMeta::new_readonly(*serum_program_id, false),
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

/// Creates an 'initialize2' instruction.
pub fn initialize2(
    program_id: &Pubkey,
    amm_id: &Pubkey,
    amm_authority: &Pubkey,
    amm_open_orders: &Pubkey,
    lp_mint_key: &Pubkey,
    coin_mint_key: &Pubkey,
    pc_mint_key: &Pubkey,
    pool_coin_token_account: &Pubkey,
    pool_pc_token_account: &Pubkey,
    pool_withdraw_queue: &Pubkey,
    amm_target_orders: &Pubkey,
    pool_temp_lp_key: &Pubkey,
    serum_program_id: &Pubkey,
    serum_market: &Pubkey,
    user_wallet: &Pubkey,
    user_token_coin: &Pubkey,
    user_token_pc: &Pubkey,
    user_lp_token_account: &Pubkey,
    nonce: u8,
    open_time: u64,
    init_pc_amount: u64,
    init_coin_amount: u64,
) -> Result<Instruction, ProgramError> {
    let init_data = AmmInstruction::Initialize2(InitializeInstruction2 {
        nonce,
        open_time,
        init_pc_amount,
        init_coin_amount,
    });
    let data = init_data.pack()?;

    let accounts = vec![
        // spl
        AccountMeta::new_readonly(spl_token::id(), false),
        AccountMeta::new_readonly(spl_associated_token_account::id(), false),
        AccountMeta::new_readonly(solana_program::system_program::id(), false),
        AccountMeta::new_readonly(sysvar::rent::id(), false),
        // amm
        AccountMeta::new(*amm_id, false),
        AccountMeta::new_readonly(*amm_authority, false),
        AccountMeta::new(*amm_open_orders, false),
        AccountMeta::new(*lp_mint_key, false),
        AccountMeta::new_readonly(*coin_mint_key, false),
        AccountMeta::new_readonly(*pc_mint_key, false),
        AccountMeta::new(*pool_coin_token_account, false),
        AccountMeta::new(*pool_pc_token_account, false),
        AccountMeta::new(*pool_withdraw_queue, false),
        AccountMeta::new(*amm_target_orders, false),
        AccountMeta::new(*pool_temp_lp_key, false),
        // serum
        AccountMeta::new_readonly(*serum_program_id, false),
        AccountMeta::new_readonly(*serum_market, false),
        // user wallet
        AccountMeta::new(*user_wallet, true),
        AccountMeta::new(*user_token_coin, false),
        AccountMeta::new(*user_token_pc, false),
        AccountMeta::new(*user_lp_token_account, false),
    ];

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
    lp_mint_address: &Pubkey,
    pool_coin_token_account: &Pubkey,
    pool_pc_token_account: &Pubkey,
    serum_market: &Pubkey,
    serum_event_queue: &Pubkey,
    user_coin_token_account: &Pubkey,
    user_pc_token_account: &Pubkey,
    user_lp_token_account: &Pubkey,
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
        AccountMeta::new_readonly(*serum_event_queue, false),
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

    serum_event_q: &Pubkey,
    serum_bids: &Pubkey,
    serum_asks: &Pubkey,
    // lp amount
    amount: u64,
) -> Result<Instruction, ProgramError> {
    let data = AmmInstruction::Withdraw(WithdrawInstruction { amount }).pack()?;

    let accounts = vec![
        // spl token
        AccountMeta::new_readonly(spl_token::id(), false),
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
        // serum
        AccountMeta::new(*serum_event_q, false),
        AccountMeta::new(*serum_bids, false),
        AccountMeta::new(*serum_asks, false),
    ];

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

/// Creates a 'swap base out' instruction.
pub fn swap_base_out(
    program_id: &Pubkey,
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
