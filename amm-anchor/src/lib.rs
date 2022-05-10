//! Anchor-compatible SDK for the Raydium AMM program.
#![deny(missing_docs)]
#![deny(rustdoc::all)]
#![allow(rustdoc::missing_doc_code_examples)]
#![allow(clippy::nonstandard_macro_braces)]

mod accounts;
mod instructions;

pub use accounts::*;
pub use instructions::*;

use anchor_lang::prelude::*;

declare_id!("675kPX9MHTjS2zt1qfr1NYHuzeLXfQM9H24wFSUt1Mp8");

/// The AMM program
#[derive(Clone)]
pub struct Amm;

impl anchor_lang::Id for Amm {
    fn id() -> Pubkey {
        ID
    }
}
