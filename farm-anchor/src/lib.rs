//! Anchor-compatible SDK for the Raydium farm program.

#![deny(rustdoc::all)]
#![allow(rustdoc::missing_doc_code_examples)]
#![allow(clippy::nonstandard_macro_braces)]

mod accounts;
mod instructions;

pub use accounts::*;
pub use instructions::*;

use anchor_lang::prelude::*;

declare_id!("CBuCnLe26faBpcBP2fktp4rp8abpcAnTWft6ZrP5Q4T");

/// Farm Program
#[derive(Clone)]
pub struct Farm;

impl anchor_lang::Id for Farm {
    fn id() -> Pubkey {
        ID
    }
}
