//! Instruction types

#![allow(clippy::too_many_arguments)]

use solana_program::instruction::AccountMeta;
use solana_program::instruction::Instruction;
use solana_program::program_error::ProgramError;
use solana_program::pubkey::Pubkey;
use solana_program::sysvar;
use std::mem::size_of;

/// Inital values for the Stake Pool
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct InitArgs {
    /// nonce for calc authirity
    pub nonce: u64,
    /// reward per slot
    pub reward_per_slot: u64,
    /// ignore
    pub ignore: u128,
}

/// Instructions supported by the StakePool program.
#[repr(C)]
#[derive(Clone, Debug, PartialEq)]
pub enum StakePoolInstruction {
    ///   Initializes a new StakePool.
    Initialize(InitArgs),

    /// Deposit lp into the pool.
    Deposit(u64),

    /// Withdraw lp token from the pool .
    Withdraw(u64),

    /// Update Pool acc
    UpdatePool,

    /// Reserved0
    Reserved0,

    /// Reserved1
    Reserved1,

    /// Reserved2
    Reserved2,

    /// EmergencyWithdraw
    EmergencyWithdraw,

    /// Reserved3
    Reserved3,

    /// CreateAssociatedAccount
    CreateAssociatedAccount,

    /// Deposit lp token into the pool v2
    DepositV2(u64),

    /// Withdraw lp token from the pool v2
    WithdrawV2(u64),
}

impl StakePoolInstruction {
    /// Deserializes a byte buffer into an [StakePoolInstruction](enum.StakePoolInstruction.html).
    pub fn deserialize(input: &[u8]) -> Result<Self, ProgramError> {
        if input.len() < size_of::<u8>() {
            return Err(ProgramError::InvalidAccountData);
        }
        Ok(match input[0] {
            0 => {
                let val: &InitArgs = unpack(input)?;
                Self::Initialize(*val)
            }
            1 => {
                let val: &u64 = unpack(input)?;
                Self::Deposit(*val)
            }
            2 => {
                let val: &u64 = unpack(input)?;
                Self::Withdraw(*val)
            }
            3 => Self::UpdatePool,

            7 => Self::EmergencyWithdraw,

            9 => Self::CreateAssociatedAccount,
            10 => {
                let val: &u64 = unpack(input)?;
                Self::DepositV2(*val)
            }
            11 => {
                let val: &u64 = unpack(input)?;
                Self::WithdrawV2(*val)
            }
            _ => return Err(ProgramError::InvalidAccountData),
        })
    }

    /// Serializes an [StakePoolInstruction](enum.StakePoolInstruction.html) into a byte buffer.
    pub fn serialize(&self) -> Result<Vec<u8>, ProgramError> {
        let mut output = vec![0u8; size_of::<StakePoolInstruction>()];
        match self {
            Self::Initialize(init) => {
                output[0] = 0;
                #[allow(clippy::cast_ptr_alignment)]
                let value = unsafe { &mut *(&mut output[1] as *mut u8 as *mut InitArgs) };
                *value = *init;
            }
            Self::Deposit(val) => {
                output[0] = 1;
                #[allow(clippy::cast_ptr_alignment)]
                let value = unsafe { &mut *(&mut output[1] as *mut u8 as *mut u64) };
                *value = *val;
            }
            Self::Withdraw(val) => {
                output[0] = 2;
                #[allow(clippy::cast_ptr_alignment)]
                let value = unsafe { &mut *(&mut output[1] as *mut u8 as *mut u64) };
                *value = *val;
            }
            Self::UpdatePool => {
                output[0] = 3;
            }

            Self::EmergencyWithdraw => {
                output[0] = 7;
            }

            Self::CreateAssociatedAccount => {
                output[0] = 9;
            }
            Self::DepositV2(val) => {
                output[0] = 10;
                let value = unsafe { &mut *(&mut output[1] as *mut u8 as *mut u64) };
                *value = *val;
            }
            Self::WithdrawV2(val) => {
                output[0] = 11;
                let value = unsafe { &mut *(&mut output[1] as *mut u8 as *mut u64) };
                *value = *val;
            }

            _ => return Err(ProgramError::InvalidAccountData),
        }
        Ok(output)
    }
}

/// Unpacks a reference from a bytes buffer.
pub fn unpack<T>(input: &[u8]) -> Result<&T, ProgramError> {
    if input.len() < size_of::<u8>() + size_of::<T>() {
        return Err(ProgramError::InvalidAccountData);
    }
    #[allow(clippy::cast_ptr_alignment)]
    let val: &T = unsafe { &*(&input[1] as *const u8 as *const T) };
    Ok(val)
}

/// Creates an 'initialize' instruction.
pub fn initialize(
    program_id: &Pubkey,
    stake_pool: &Pubkey,
    authority: &Pubkey,
    lp_vault: &Pubkey,
    reward_vault: &Pubkey,
    init_args: InitArgs,
) -> Result<Instruction, ProgramError> {
    let init_data = StakePoolInstruction::Initialize(init_args);
    let data = init_data.serialize()?;
    let accounts = vec![
        AccountMeta::new(*stake_pool, true),
        AccountMeta::new_readonly(*authority, false),
        AccountMeta::new_readonly(*lp_vault, false),
        AccountMeta::new_readonly(*reward_vault, false),
        AccountMeta::new_readonly(sysvar::clock::id(), false),
    ];
    Ok(Instruction {
        program_id: *program_id,
        accounts,
        data,
    })
}

/// Creates a 'Deposit' instruction.
pub fn deposit(
    program_id: &Pubkey,
    stake_pool: &Pubkey,
    pool_authority: &Pubkey,
    staker_info: &Pubkey,
    staker_owner: &Pubkey,
    src_lp_token: &Pubkey,
    vault_lp_token: &Pubkey,
    dest_reward_token: &Pubkey,
    vault_reward_token: &Pubkey,
    token_program_id: &Pubkey,
    amount: u64,
) -> Result<Instruction, ProgramError> {
    let args = StakePoolInstruction::Deposit(amount);
    let data = args.serialize()?;
    let accounts = vec![
        AccountMeta::new(*stake_pool, false),
        AccountMeta::new_readonly(*pool_authority, false),
        AccountMeta::new(*staker_info, false),
        AccountMeta::new_readonly(*staker_owner, true),
        AccountMeta::new(*src_lp_token, false),
        AccountMeta::new(*vault_lp_token, false),
        AccountMeta::new(*dest_reward_token, false),
        AccountMeta::new(*vault_reward_token, false),
        AccountMeta::new_readonly(sysvar::clock::id(), false),
        AccountMeta::new_readonly(*token_program_id, false),
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
    stake_pool: &Pubkey,
    pool_authority: &Pubkey,
    staker_info: &Pubkey,
    staker_owner: &Pubkey,
    dest_lp_token: &Pubkey,
    vault_lp_token: &Pubkey,
    dest_reward_token: &Pubkey,
    vault_reward_token: &Pubkey,
    token_program_id: &Pubkey,
    amount: u64,
) -> Result<Instruction, ProgramError> {
    let args = StakePoolInstruction::Withdraw(amount);
    let data = args.serialize()?;
    let accounts = vec![
        AccountMeta::new(*stake_pool, false),
        AccountMeta::new_readonly(*pool_authority, false),
        AccountMeta::new(*staker_info, false),
        AccountMeta::new_readonly(*staker_owner, true),
        AccountMeta::new(*dest_lp_token, false),
        AccountMeta::new(*vault_lp_token, false),
        AccountMeta::new(*dest_reward_token, false),
        AccountMeta::new(*vault_reward_token, false),
        AccountMeta::new_readonly(sysvar::clock::id(), false),
        AccountMeta::new_readonly(*token_program_id, false),
    ];
    Ok(Instruction {
        program_id: *program_id,
        accounts,
        data,
    })
}

/// Creates a 'emergencyWithdraw' instruction.
pub fn emergency_withdraw(
    program_id: &Pubkey,
    stake_pool: &Pubkey,
    pool_authority: &Pubkey,
    staker_info: &Pubkey,
    staker_owner: &Pubkey,
    dest_lp_token: &Pubkey,
    vault_lp_token: &Pubkey,
    token_program_id: &Pubkey,
) -> Result<Instruction, ProgramError> {
    let args = StakePoolInstruction::EmergencyWithdraw;
    let data = args.serialize()?;
    let accounts = vec![
        AccountMeta::new(*stake_pool, false),
        AccountMeta::new_readonly(*pool_authority, false),
        AccountMeta::new(*staker_info, false),
        AccountMeta::new_readonly(*staker_owner, true),
        AccountMeta::new(*dest_lp_token, false),
        AccountMeta::new(*vault_lp_token, false),
        AccountMeta::new_readonly(*token_program_id, false),
    ];
    Ok(Instruction {
        program_id: *program_id,
        accounts,
        data,
    })
}

/// Creates `update_pool` instruction
pub fn update_pool(
    program_id: &Pubkey,
    stake_pool: &Pubkey,
    vault_lp_token: &Pubkey,
) -> Result<Instruction, ProgramError> {
    let accounts = vec![
        AccountMeta::new(*stake_pool, false),
        AccountMeta::new(*vault_lp_token, false),
        AccountMeta::new_readonly(sysvar::clock::id(), false),
    ];
    Ok(Instruction {
        program_id: *program_id,
        accounts,
        data: StakePoolInstruction::UpdatePool.serialize()?,
    })
}

/// create_associated_account
pub fn create_associated_account(
    program_id: &Pubkey,
    stake_pool: &Pubkey,
    associated_user_stake_info: &Pubkey,
    owner: &Pubkey,
) -> Result<Instruction, ProgramError> {
    let args = StakePoolInstruction::CreateAssociatedAccount;
    let data = args.serialize()?;
    let accounts = vec![
        AccountMeta::new(*stake_pool, false),
        AccountMeta::new(*associated_user_stake_info, false),
        AccountMeta::new_readonly(*owner, true),
        AccountMeta::new_readonly(solana_program::system_program::id(), false),
        AccountMeta::new_readonly(sysvar::rent::id(), false),
    ];
    Ok(Instruction {
        program_id: *program_id,
        accounts,
        data,
    })
}
