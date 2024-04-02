//! Anchor-compatible SDK for the Raydium AMM program.
// #![deny(missing_docs)]
#![deny(rustdoc::all)]
#![allow(rustdoc::missing_doc_code_examples)]
#![allow(clippy::nonstandard_macro_braces)]

mod accounts;
mod instructions;

pub use accounts::*;
pub use instructions::*;

use anchor_lang::prelude::*;
/// aaa
pub mod create_pool_fee_address {
    anchor_lang::declare_id!("7YttLkHDoNj9wyDur5pM1ejNaAvT9X4eqaYcHQqtj2G5");
}
/// bbb
pub mod openbook_program_id {
    anchor_lang::declare_id!("srmqPvymJeFKQ4zGQed1GFppgkRHL9kaELCbyksJtPX");
}

declare_id!("675kPX9MHTjS2zt1qfr1NYHuzeLXfQM9H24wFSUt1Mp8");

/// The AMM program
#[derive(Clone)]
pub struct Amm;

impl anchor_lang::Id for Amm {
    fn id() -> Pubkey {
        ID
    }
}
