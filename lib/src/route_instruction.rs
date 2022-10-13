//! Instruction types

#![allow(clippy::too_many_arguments)]

use crate::route_error::RouteError;
use solana_program::{
    instruction::{AccountMeta, Instruction},
    program_error::ProgramError,
    pubkey::Pubkey,
};
use std::convert::TryInto;
use std::mem::size_of;

pub mod amm_program {
    solana_program::declare_id!("675kPX9MHTjS2zt1qfr1NYHuzeLXfQM9H24wFSUt1Mp8");
}

pub mod stable_program {
    solana_program::declare_id!("5quBtoiQqxF9Jv6KYKctB59NT3gtJD2Y65kdnB1Uev3h");
}

pub mod amm_v3_program {
    solana_program::declare_id!("CAMMCzo5YL8w4VFF8KVHrK22GGUsp5VTaW7grrKgrWqK");
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct RouteSwapBaseInArgs {
    // SOURCE amount to transfer, output to DESTINATION is based on the exchange rate
    pub amount_in: u64,
    /// Minimum amount of DESTINATION token to output, prevents excessive slippage
    pub minimum_amount_out: u64,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct RouteSwapBaseOutArgs {
    pub max_amount_in: u64,
    // SOURCE amount to transfer, output to DESTINATION is based on the exchange rate
    pub amount_out: u64,
}

/// Instructions supported by the Route program.
#[repr(C)]
#[derive(Clone, Debug, PartialEq)]
pub enum RouteInstruction {
    // amm swap base in
    RouteSwapIn(RouteSwapBaseInArgs),
    RouteSwapMinOut,
    // stable swap base in
    RouteStableSwapIn(RouteSwapBaseInArgs),
    RouteStableSwapMinOut,

    // amm swap base in
    RouteSwapBaseInIn(RouteSwapBaseInArgs),
    RouteSwapBaseInMinOut,
}

impl RouteInstruction {
    /// Unpacks a byte buffer into a [RouteInstruction](enum.RouteInstruction.html).
    pub fn unpack(input: &[u8]) -> Result<Self, ProgramError> {
        let (&tag, rest) = input.split_first().ok_or(RouteError::InvalidInstruction)?;
        Ok(match tag {
            0 => {
                let (amount_in, rest) = Self::unpack_u64(rest)?;
                let (minimum_amount_out, _rest) = Self::unpack_u64(rest)?;
                Self::RouteSwapIn(RouteSwapBaseInArgs {
                    amount_in,
                    minimum_amount_out,
                })
            }
            1 => Self::RouteSwapMinOut,
            2 => {
                let (amount_in, rest) = Self::unpack_u64(rest)?;
                let (minimum_amount_out, _rest) = Self::unpack_u64(rest)?;
                Self::RouteStableSwapIn(RouteSwapBaseInArgs {
                    amount_in,
                    minimum_amount_out,
                })
            }
            3 => Self::RouteStableSwapMinOut,
            4 => {
                let (amount_in, rest) = Self::unpack_u64(rest)?;
                let (minimum_amount_out, _rest) = Self::unpack_u64(rest)?;
                Self::RouteSwapBaseInIn(RouteSwapBaseInArgs {
                    amount_in,
                    minimum_amount_out,
                })
            }
            5 => Self::RouteSwapBaseInMinOut,
            _ => return Err(RouteError::InvalidInstruction.into()),
        })
    }

    fn unpack_u64(input: &[u8]) -> Result<(u64, &[u8]), ProgramError> {
        if input.len() >= 8 {
            let (amount, rest) = input.split_at(8);
            let amount = amount
                .get(..8)
                .and_then(|slice| slice.try_into().ok())
                .map(u64::from_le_bytes)
                .ok_or(RouteError::InvalidInstruction)?;
            Ok((amount, rest))
        } else {
            Err(RouteError::InvalidInstruction.into())
        }
    }

    /// Packs a [RouteInstruction](enum.RouteInstruction.html) into a byte buffer.
    pub fn pack(&self) -> Result<Vec<u8>, ProgramError> {
        let mut buf = Vec::with_capacity(size_of::<Self>());
        match &*self {
            Self::RouteSwapIn(RouteSwapBaseInArgs {
                amount_in,
                minimum_amount_out,
            }) => {
                buf.push(0);
                buf.extend_from_slice(&amount_in.to_le_bytes());
                buf.extend_from_slice(&minimum_amount_out.to_le_bytes());
            }
            Self::RouteSwapMinOut => {
                buf.push(1);
            }
            Self::RouteStableSwapIn(RouteSwapBaseInArgs {
                amount_in,
                minimum_amount_out,
            }) => {
                buf.push(2);
                buf.extend_from_slice(&amount_in.to_le_bytes());
                buf.extend_from_slice(&minimum_amount_out.to_le_bytes());
            }
            Self::RouteStableSwapMinOut => {
                buf.push(3);
            }
            Self::RouteSwapBaseInIn(RouteSwapBaseInArgs {
                amount_in,
                minimum_amount_out,
            }) => {
                buf.push(4);
                buf.extend_from_slice(&amount_in.to_le_bytes());
                buf.extend_from_slice(&minimum_amount_out.to_le_bytes());
            }
            Self::RouteSwapBaseInMinOut => {
                buf.push(5);
            }
        }
        Ok(buf)
    }
}

/// Creates a 'route swap in' instruction.
pub fn route_swap_in(
    program_id: &Pubkey,

    route_from_amm_program_id: &Pubkey,
    route_from_amm_id: &Pubkey,
    route_to_amm_id: &Pubkey,
    amm_authority: &Pubkey,
    amm_open_orders: &Pubkey,
    amm_coin_vault: &Pubkey,
    amm_pc_vault: &Pubkey,

    serum_program_id: &Pubkey,
    serum_market: &Pubkey,
    serum_bids: &Pubkey,
    serum_asks: &Pubkey,
    serum_event_queue: &Pubkey,
    serum_coin_vault_account: &Pubkey,
    serum_pc_vault_account: &Pubkey,
    serum_vault_signer: &Pubkey,

    uer_source_token_account: &Pubkey,
    uer_route_token_account: &Pubkey,
    user_pda_account: &Pubkey,
    user_source_owner: &Pubkey,

    amount_in: u64,
    minimum_amount_out: u64,
) -> Result<Instruction, ProgramError> {
    let data = RouteInstruction::RouteSwapIn(RouteSwapBaseInArgs {
        amount_in,
        minimum_amount_out,
    })
    .pack()?;

    let accounts = vec![
        // spl token
        AccountMeta::new_readonly(solana_program::system_program::id(), false),
        AccountMeta::new_readonly(spl_token::id(), false),
        // amm
        AccountMeta::new_readonly(*route_from_amm_program_id, false),
        AccountMeta::new(*route_from_amm_id, false),
        AccountMeta::new_readonly(*route_to_amm_id, false),
        AccountMeta::new_readonly(*amm_authority, false),
        AccountMeta::new(*amm_open_orders, false),
        AccountMeta::new(*amm_coin_vault, false),
        AccountMeta::new(*amm_pc_vault, false),
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
        AccountMeta::new(*uer_route_token_account, false),
        AccountMeta::new(*user_pda_account, false),
        AccountMeta::new_readonly(*user_source_owner, true),
    ];

    Ok(Instruction {
        program_id: *program_id,
        accounts,
        data,
    })
}

/// Creates a 'route swap min out' instruction.
pub fn route_swap_min_out(
    program_id: &Pubkey,

    route_to_amm_program_id: &Pubkey,
    route_from_amm_id: &Pubkey,
    route_to_amm_id: &Pubkey,
    amm_authority: &Pubkey,
    amm_open_orders: &Pubkey,
    amm_coin_vault: &Pubkey,
    amm_pc_vault: &Pubkey,

    serum_program_id: &Pubkey,
    serum_market: &Pubkey,
    serum_bids: &Pubkey,
    serum_asks: &Pubkey,
    serum_event_queue: &Pubkey,
    serum_coin_vault_account: &Pubkey,
    serum_pc_vault_account: &Pubkey,
    serum_vault_signer: &Pubkey,

    uer_route_token_account: &Pubkey,
    uer_destination_token_account: &Pubkey,
    user_pda_account: &Pubkey,
    user_source_owner: &Pubkey,
) -> Result<Instruction, ProgramError> {
    let data = RouteInstruction::RouteSwapMinOut.pack()?;

    let accounts = vec![
        // spl token
        AccountMeta::new_readonly(spl_token::id(), false),
        // amm
        AccountMeta::new_readonly(*route_to_amm_program_id, false),
        AccountMeta::new_readonly(*route_from_amm_id, false),
        AccountMeta::new(*route_to_amm_id, false),
        AccountMeta::new_readonly(*amm_authority, false),
        AccountMeta::new(*amm_open_orders, false),
        AccountMeta::new(*amm_coin_vault, false),
        AccountMeta::new(*amm_pc_vault, false),
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
        AccountMeta::new(*uer_route_token_account, false),
        AccountMeta::new(*uer_destination_token_account, false),
        AccountMeta::new(*user_pda_account, false),
        AccountMeta::new_readonly(*user_source_owner, true),
    ];

    Ok(Instruction {
        program_id: *program_id,
        accounts,
        data,
    })
}

/// Creates a 'route stable swap in' instruction.
pub fn route_stable_swap_in(
    program_id: &Pubkey,

    route_from_amm_program_id: &Pubkey,
    route_from_amm_id: &Pubkey,
    route_to_amm_id: &Pubkey,
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
    serum_coin_vault_account: &Pubkey,
    serum_pc_vault_account: &Pubkey,
    serum_vault_signer: &Pubkey,

    uer_source_token_account: &Pubkey,
    uer_route_token_account: &Pubkey,
    user_pda_account: &Pubkey,
    user_source_owner: &Pubkey,

    amount_in: u64,
    minimum_amount_out: u64,
) -> Result<Instruction, ProgramError> {
    let data = RouteInstruction::RouteStableSwapIn(RouteSwapBaseInArgs {
        amount_in,
        minimum_amount_out,
    })
    .pack()?;

    let accounts = vec![
        // spl token
        AccountMeta::new_readonly(solana_program::system_program::id(), false),
        AccountMeta::new_readonly(spl_token::id(), false),
        // amm
        AccountMeta::new_readonly(*route_from_amm_program_id, false),
        AccountMeta::new(*route_from_amm_id, false),
        AccountMeta::new_readonly(*route_to_amm_id, false),
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
        AccountMeta::new(*serum_coin_vault_account, false),
        AccountMeta::new(*serum_pc_vault_account, false),
        AccountMeta::new_readonly(*serum_vault_signer, false),
        // user
        AccountMeta::new(*uer_source_token_account, false),
        AccountMeta::new(*uer_route_token_account, false),
        AccountMeta::new(*user_pda_account, false),
        AccountMeta::new_readonly(*user_source_owner, true),
    ];

    Ok(Instruction {
        program_id: *program_id,
        accounts,
        data,
    })
}

/// Creates a 'route stable swap min out' instruction.
pub fn route_stable_swap_min_out(
    program_id: &Pubkey,

    route_to_amm_program_id: &Pubkey,
    route_from_amm_id: &Pubkey,
    route_to_amm_id: &Pubkey,
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
    serum_coin_vault_account: &Pubkey,
    serum_pc_vault_account: &Pubkey,
    serum_vault_signer: &Pubkey,

    uer_route_token_account: &Pubkey,
    uer_destination_token_account: &Pubkey,
    user_pda_account: &Pubkey,
    user_source_owner: &Pubkey,
) -> Result<Instruction, ProgramError> {
    let data = RouteInstruction::RouteStableSwapMinOut.pack()?;

    let accounts = vec![
        // spl token
        AccountMeta::new_readonly(spl_token::id(), false),
        // amm
        AccountMeta::new_readonly(*route_to_amm_program_id, false),
        AccountMeta::new_readonly(*route_from_amm_id, false),
        AccountMeta::new(*route_to_amm_id, false),
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
        AccountMeta::new(*serum_coin_vault_account, false),
        AccountMeta::new(*serum_pc_vault_account, false),
        AccountMeta::new_readonly(*serum_vault_signer, false),
        // user
        AccountMeta::new(*uer_route_token_account, false),
        AccountMeta::new(*uer_destination_token_account, false),
        AccountMeta::new(*user_pda_account, false),
        AccountMeta::new_readonly(*user_source_owner, true),
    ];

    Ok(Instruction {
        program_id: *program_id,
        accounts,
        data,
    })
}

/// Creates a 'route swap base in in' instruction.
///
/// About remaining_accounts details
/// amm:
///     amm_authority `[]`,
///     serum_program_id `[]`,
///     serum_vault_signer `[]`
///     amm_open_orders `[writable]`,
///     amm_coin_vault `[writable]`,
///     amm_pc_vault `[writable]`,
///     serum_market `[writable]`,
///     serum_bids `[writable]`,
///     serum_asks `[writable]`,
///     serum_event_queue `[writable]`,
///     serum_coin_vault `[writable]`,
///     serum_pc_vault `[writable]`,
///
/// stable:
///     amm_authority `[]`,
///     serum_program_id `[]`,
///     serum_vault_signer `[]`,
///     model_data_account `[]`,
///     amm_open_orders `[writable]`,
///     amm_coin_vault `[writable]`,
///     amm_pc_vault `[writable]`,
///     serum_market `[writable]`,
///     serum_bids `[writable]`,
///     serum_asks `[writable]`,
///     serum_event_queue `[writable]`,
///     serum_coin_vault `[writable]`,
///     serum_pc_vault `[writable]`,
///
/// amm_v3:
///     amm_config `[]`,
///     pool_state `[writable]`,
///     input_vault `[writable]`,
///     output_vault `[writable]`,
///     observation_state `[writable]`,
///     cur_or_next_tick_array `[writable]`,
///     next_tick_array `[writable]`,
///     next_next_tick_array `[writable]`,
///     ...
pub fn route_swap_base_in_in(
    program_id: &Pubkey,
    from_amm_program_id: &Pubkey,
    from_amm_id: &Pubkey,
    to_amm_id: &Pubkey,
    user_source_token: &Pubkey,
    user_route_token: &Pubkey,
    user_pda_account: &Pubkey,
    user_source_owner: &Pubkey,
    remaining_accounts: Vec<Pubkey>,

    amount_in: u64,
    minimum_amount_out: u64,
) -> Result<Instruction, ProgramError> {
    let data = RouteInstruction::RouteSwapBaseInIn(RouteSwapBaseInArgs {
        amount_in,
        minimum_amount_out,
    })
    .pack()?;

    let mut accounts = vec![
        // spl token
        AccountMeta::new_readonly(solana_program::system_program::id(), false),
        AccountMeta::new_readonly(spl_token::id(), false),
        // from amm
        AccountMeta::new_readonly(*from_amm_program_id, false),
        AccountMeta::new(*from_amm_id, false),
        // to amm
        AccountMeta::new(*to_amm_id, false),
        // user
        AccountMeta::new(*user_source_token, false),
        AccountMeta::new(*user_route_token, false),
        AccountMeta::new(*user_pda_account, false),
        AccountMeta::new_readonly(*user_source_owner, true),
    ];
    let mut remaining = remaining_accounts;
    let read_only_accounts;
    let read_write_accounts;
    // split remaining_accounts for read_only and read_write accounts
    if *program_id == amm_program::id() {
        read_write_accounts = remaining.split_off(3);
        read_only_accounts = remaining;
    } else if *program_id == stable_program::id() {
        read_write_accounts = remaining.split_off(4);
        read_only_accounts = remaining;
    } else {
        read_write_accounts = remaining.split_off(1);
        read_only_accounts = remaining;
    }
    // extend read_only accounts
    accounts.extend(
        read_only_accounts
            .iter()
            .map(|pk| AccountMeta::new_readonly(*pk, false)),
    );
    // extend read_write accounts
    accounts.extend(
        read_write_accounts
            .iter()
            .map(|pk| AccountMeta::new(*pk, false)),
    );

    Ok(Instruction {
        program_id: *program_id,
        accounts,
        data,
    })
}

/// Creates a 'route swap base in min_out' instruction.
///
/// About remaining_accounts details
/// amm:
///     amm_authority `[]`,
///     serum_program_id `[]`,
///     serum_vault_signer `[]`
///     amm_open_orders `[writable]`,
///     amm_coin_vault `[writable]`,
///     amm_pc_vault `[writable]`,
///     serum_market `[writable]`,
///     serum_bids `[writable]`,
///     serum_asks `[writable]`,
///     serum_event_queue `[writable]`,
///     serum_coin_vault `[writable]`,
///     serum_pc_vault `[writable]`,
///
/// stable:
///     amm_authority `[]`,
///     serum_program_id `[]`,
///     serum_vault_signer `[]`,
///     model_data_account `[]`,
///     amm_open_orders `[writable]`,
///     amm_coin_vault `[writable]`,
///     amm_pc_vault `[writable]`,
///     serum_market `[writable]`,
///     serum_bids `[writable]`,
///     serum_asks `[writable]`,
///     serum_event_queue `[writable]`,
///     serum_coin_vault `[writable]`,
///     serum_pc_vault `[writable]`,
///
/// amm_v3:
///     amm_config `[]`,
///     pool_state `[writable]`,
///     input_vault `[writable]`,
///     output_vault `[writable]`,
///     observation_state `[writable]`,
///     cur_or_next_tick_array `[writable]`,
///     next_tick_array `[writable]`,
///     next_next_tick_array `[writable]`,
///     ...
pub fn route_swap_base_in_min_out(
    program_id: &Pubkey,
    to_amm_program_id: &Pubkey,
    to_amm_id: &Pubkey,
    from_amm_id: &Pubkey,
    user_route_token: &Pubkey,
    user_destination_token: &Pubkey,
    user_pda_account: &Pubkey,
    user_source_owner: &Pubkey,
    remaining_accounts: Vec<Pubkey>,
) -> Result<Instruction, ProgramError> {
    let data = RouteInstruction::RouteSwapBaseInMinOut.pack()?;

    let mut accounts = vec![
        // spl token
        AccountMeta::new_readonly(solana_program::system_program::id(), false),
        AccountMeta::new_readonly(spl_token::id(), false),
        // to amm
        AccountMeta::new_readonly(*to_amm_program_id, false),
        AccountMeta::new(*to_amm_id, false),
        // from amm
        AccountMeta::new(*from_amm_id, false),
        // user
        AccountMeta::new(*user_route_token, false),
        AccountMeta::new(*user_destination_token, false),
        AccountMeta::new(*user_pda_account, false),
        AccountMeta::new_readonly(*user_source_owner, true),
    ];
    let mut remaining = remaining_accounts;
    let read_only_accounts;
    let read_write_accounts;
    // split remaining_accounts for read_only and read_write accounts
    if *program_id == amm_program::id() {
        read_write_accounts = remaining.split_off(3);
        read_only_accounts = remaining;
    } else if *program_id == stable_program::id() {
        read_write_accounts = remaining.split_off(4);
        read_only_accounts = remaining;
    } else {
        // amm_v3
        read_write_accounts = remaining.split_off(1);
        read_only_accounts = remaining;
    }
    // extend read_only accounts
    accounts.extend(
        read_only_accounts
            .iter()
            .map(|pk| AccountMeta::new_readonly(*pk, false)),
    );
    // extend read_write accounts
    accounts.extend(
        read_write_accounts
            .iter()
            .map(|pk| AccountMeta::new(*pk, false)),
    );

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
