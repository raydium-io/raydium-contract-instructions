//! Accounts structs for Raydium Stable AMM / Liquidity.

use anchor_lang::prelude::*;

/// Accounts for an `pre_initialize` instruction.
#[derive(Accounts, Clone)]
pub struct PreInitialize<'info> {
    /// AMM target orders account, a PDA create with seed = [program_id,serum_market_id, b"target_associated_seed"]
    /// CHECK: Safe
    #[account(mut)]
    pub amm_target_orders: AccountInfo<'info>,
    /// Amm authority, a PDA create with seed = [b"amm authority"]
    /// CHECK: Safe
    pub amm_authority: AccountInfo<'info>,
    /// Pool lp mint account, a PDA create with seed = [program_id,serum_market_id, b"lp_mint_associated_seed"].
    /// Must be empty, owned by $authority.
    /// CHECK: Safe
    #[account(mut)]
    pub amm_lp_mint: AccountInfo<'info>,
    /// Coin mint account
    /// CHECK: Safe
    pub coin_mint: AccountInfo<'info>,
    /// Pc mint account
    /// CHECK: Safe
    pub pc_mint: AccountInfo<'info>,
    /// Coin vault accoun Account.  a PDA create with seed = [program_id,serum_market_id, b"coin_vault_associated_seed"].
    /// Must be non zero, owned by $authority
    /// CHECK: Safe
    #[account(mut)]
    pub amm_coin_vault: AccountInfo<'info>,
    /// Pc vault accoun Account, a PDA create with seed = [program_id,serum_market_id, b"pc_vault_associated_seed"].
    /// Must be non zero, owned by $authority.
    /// CHECK: Safe
    #[account(mut)]
    pub amm_pc_vault: AccountInfo<'info>,
    /// Serum market Account. serum_dex program is the owner.
    /// CHECK: Safe
    pub serum_market: AccountInfo<'info>,
    /// The user wallet create the pool
    /// CHECK: Safe
    #[account(signer)]
    pub user_wallet: AccountInfo<'info>,
    /// CHECK: Safe Spl token program
    #[account(address = spl_token::ID)]
    pub spl_token_program: AccountInfo<'info>,
    /// CHECK: Safe System program
    pub system_program: Program<'info, System>,
    /// CHECK: Safe Sys program
    pub rent: Sysvar<'info, Rent>,
}

/// Accounts for an `initialize` instruction.
#[derive(Accounts, Clone)]
pub struct Initialize<'info> {
    /// The new amm Account to be create, a PDA create with seed = [program_id,serum_market_id, b"amm_associated_seed"]
    /// CHECK: Safe
    #[account(mut)]
    pub amm: AccountInfo<'info>,
    /// Amm authority, a PDA create with seed = [b"amm authority"]
    /// CHECK: Safe
    pub amm_authority: AccountInfo<'info>,
    /// Amm open_orders Account, a PDA create with seed = [program_id,serum_market_id, b"open_order_associated_seed"]
    /// CHECK: Safe
    #[account(mut)]
    pub amm_open_orders: AccountInfo<'info>,
    /// Pool lp mint account. Must be empty, owned by $authority.
    /// CHECK: Safe
    #[account(mut)]
    pub amm_lp_mint: AccountInfo<'info>,
    /// Coin mint account
    /// CHECK: Safe
    pub coin_mint: AccountInfo<'info>,
    /// Pc mint account
    /// CHECK: Safe
    pub pc_mint: AccountInfo<'info>,
    /// Coin vault account. Must be non zero, owned by $authority
    /// CHECK: Safe
    pub amm_coin_vault: AccountInfo<'info>,
    /// Pc vault account. Must be non zero, owned by $authority.
    /// CHECK: Safe
    pub amm_pc_vault: AccountInfo<'info>,
    /// Pool target orders account
    /// CHECK: Safe
    #[account(mut)]
    pub amm_target_orders: AccountInfo<'info>,
    /// Model data account
    /// CHECK: Safe
    #[account(mut)]
    pub model_data_account: AccountInfo<'info>,
    /// Serum dex program.
    /// CHECK: Safe
    pub serum_program: AccountInfo<'info>,
    /// Serum market account. serum_dex program is the owner.
    /// CHECK: Safe
    pub serum_market: AccountInfo<'info>,
    /// User lp token account
    /// CHECK: Safe
    #[account(mut)]
    pub user_dest_lp_token: AccountInfo<'info>,
    /// The user wallet create the pool
    /// CHECK: Safe
    #[account(signer)]
    pub user_wallet: AccountInfo<'info>,
    /// CHECK: Safe
    #[account(address = spl_token::ID)]
    pub spl_token_program: AccountInfo<'info>,
    /// CHECK: Safe System program
    pub system_program: Program<'info, System>,
    /// CHECK: Safe Spl token program
    pub rent: Sysvar<'info, Rent>,
}

/// Accounts for an `deposit` instruction.
#[derive(Accounts, Clone)]
pub struct Deposit<'info> {
    /// CHECK: Safe  Amm Account
    #[account(mut)]
    pub amm: AccountInfo<'info>,
    /// CHECK: Safe  Amm authority, a PDA account derived with seed `amm authority` and amm program address
    pub amm_authority: AccountInfo<'info>,
    /// CHECK: Safe  AMM open_orders Account.
    #[account(mut)]
    pub amm_open_orders: AccountInfo<'info>,
    /// CHECK: Safe  AMM target orders account. To store plan orders infomations.
    #[account(mut)]
    pub amm_target_orders: AccountInfo<'info>,
    /// CHECK: Safe  LP mint account. Must be empty, owned by $authority.
    #[account(mut)]
    pub amm_lp_mint: AccountInfo<'info>,
    /// CHECK: Safe  Coin vault account, $authority can transfer amount.
    #[account(mut)]
    pub amm_coin_vault: AccountInfo<'info>,
    /// CHECK: Safe  Pc vault account, $authority can transfer amount.
    #[account(mut)]
    pub amm_pc_vault: AccountInfo<'info>,
    /// CHECK: Safe  Model data account
    pub model_data_account: AccountInfo<'info>,
    /// CHECK: Safe  Serum market account, serum_dex program is the owner.
    pub serum_market: AccountInfo<'info>,
    /// CHECK: Safe  User coin token account to deposit into.
    #[account(mut)]
    pub user_source_coin_token: AccountInfo<'info>,
    /// CHECK: Safe  User pc token account to deposit into.
    #[account(mut)]
    pub user_source_pc_token: AccountInfo<'info>,
    /// CHECK: Safe  User lp token account, to deposit the generated tokens, user is the owner
    #[account(mut)]
    pub user_dest_lp_token: AccountInfo<'info>,
    /// CHECK: Safe  User wallet account
    #[account(signer)]
    pub user_owner: AccountInfo<'info>,
    /// CHECK: Safe Spl token program
    #[account(address = spl_token::ID)]
    pub spl_token_program: AccountInfo<'info>,
}

/// Accounts for an `withdraw` instruction.
#[derive(Accounts, Clone)]
pub struct Withdraw<'info> {
    /// CHECK: Safe Amm account
    #[account(mut)]
    pub amm: AccountInfo<'info>,
    /// CHECK: Safe Amm authority Account
    pub amm_authority: AccountInfo<'info>,
    /// CHECK: Safe amm open orders Account
    #[account(mut)]
    pub amm_open_orders: AccountInfo<'info>,
    /// CHECK: Safe amm target_orders Account. To store plan orders infomations.
    #[account(mut)]
    pub amm_target_orders: AccountInfo<'info>,
    /// CHECK: Safe pool lp mint account. Must be empty, owned by $authority.
    #[account(mut)]
    pub amm_lp_mint: AccountInfo<'info>,
    /// CHECK: Safe Coin vault account, $authority can transfer amount.
    #[account(mut)]
    pub amm_coin_vault: AccountInfo<'info>,
    /// CHECK: Safe Pc vault account, $authority can transfer amount.
    #[account(mut)]
    pub amm_pc_vault: AccountInfo<'info>,
    /// CHECK: Safe Model data account
    pub model_data_account: AccountInfo<'info>,
    /// CHECK: Safe serum dex program id
    pub serum_program: AccountInfo<'info>,
    /// CHECK: Safe serum market Account. serum_dex program is the owner.
    #[account(mut)]
    pub serum_market: AccountInfo<'info>,
    /// CHECK: Safe coin_vault Account
    #[account(mut)]
    pub serum_coin_vault: AccountInfo<'info>,
    /// CHECK: Safe pc_vault Account
    #[account(mut)]
    pub serum_pc_vault: AccountInfo<'info>,
    /// CHECK: Safe vault_signer Account
    pub serum_vault_signer: AccountInfo<'info>,
    /// CHECK: Safe user lp token Account. Source lp, amount is transferable by $authority.
    #[account(mut)]
    pub user_source_lp_token: AccountInfo<'info>,
    /// CHECK: Safe user token coin Account. user Account to credit.
    #[account(mut)]
    pub user_dest_coin_token: AccountInfo<'info>,
    /// CHECK: Safe user token pc Account. user Account to credit.
    #[account(mut)]
    pub user_dest_pc_token: AccountInfo<'info>,
    /// CHECK: Safe User wallet account
    #[account(signer)]
    pub user_owner: AccountInfo<'info>,
    /// CHECK: Safe Spl token program
    #[account(address = spl_token::ID)]
    pub spl_token_program: AccountInfo<'info>,
}

/// Accounts for an `swap_base_in` instruction.
#[derive(Accounts, Clone)]
pub struct SwapBaseIn<'info> {
    /// CHECK: Safe amm Account
    #[account(mut)]
    pub amm: AccountInfo<'info>,
    /// CHECK: Safe Amm authority Account
    pub amm_authority: AccountInfo<'info>,
    /// CHECK: Safe amm open_orders Account
    #[account(mut)]
    pub amm_open_orders: AccountInfo<'info>,
    /// CHECK: Safe pool_token_coin Amm Account to swap FROM or To,
    #[account(mut)]
    pub amm_coin_vault: AccountInfo<'info>,
    /// CHECK: Safe pool_token_pc Amm Account to swap FROM or To,
    #[account(mut)]
    pub amm_pc_vault: AccountInfo<'info>,
    /// CHECK: Safe Model data account Account
    pub model_data_account: AccountInfo<'info>,
    /// CHECK: Safe serum dex program id
    pub serum_program: AccountInfo<'info>,
    /// CHECK: Safe serum market Account. serum_dex program is the owner.
    #[account(mut)]
    pub serum_market: AccountInfo<'info>,
    /// CHECK: Safe bids Account
    #[account(mut)]
    pub serum_bids: AccountInfo<'info>,
    /// CHECK: Safe asks Account
    #[account(mut)]
    pub serum_asks: AccountInfo<'info>,
    /// CHECK: Safe event_q Account
    #[account(mut)]
    pub serum_event_queue: AccountInfo<'info>,
    /// CHECK: Safe Serum coin vault account
    #[account(mut)]
    pub serum_coin_vault: AccountInfo<'info>,
    /// CHECK: Safe Serum pc vault account
    #[account(mut)]
    pub serum_pc_vault: AccountInfo<'info>,
    /// CHECK: Safe Serum vault signer account
    pub serum_vault_signer: AccountInfo<'info>,
    /// CHECK: Safe User source token Account. user Account to swap from.
    #[account(mut)]
    pub user_source_token: AccountInfo<'info>,
    /// CHECK: Safe User destination token Account. user Account to swap to.
    #[account(mut)]
    pub user_destination_token: AccountInfo<'info>,
    /// CHECK: Safe User owner Account
    #[account(signer)]
    pub user_source_owner: AccountInfo<'info>,
    /// CHECK: Safe Spl token program
    #[account(address = spl_token::ID)]
    pub spl_token_program: AccountInfo<'info>,
}

/// Accounts for an `swap_base_out` instruction.
#[derive(Accounts, Clone)]
pub struct SwapBaseOut<'info> {
    /// CHECK: Safe amm Account
    #[account(mut)]
    pub amm: AccountInfo<'info>,
    /// CHECK: Safe Amm authority Account
    pub amm_authority: AccountInfo<'info>,
    /// CHECK: Safe amm open_orders Account
    #[account(mut)]
    pub amm_open_orders: AccountInfo<'info>,
    /// CHECK: Safe pool_token_coin Amm Account to swap FROM or To,
    #[account(mut)]
    pub amm_coin_vault: AccountInfo<'info>,
    /// CHECK: Safe pool_token_pc Amm Account to swap FROM or To,
    #[account(mut)]
    pub amm_pc_vault: AccountInfo<'info>,
    /// CHECK: Safe Model data account Account
    pub model_data_account: AccountInfo<'info>,
    /// CHECK: Safe serum dex program id
    pub serum_program: AccountInfo<'info>,
    /// CHECK: Safe serum market Account. serum_dex program is the owner.
    #[account(mut)]
    pub serum_market: AccountInfo<'info>,
    /// CHECK: Safe bids Account
    #[account(mut)]
    pub serum_bids: AccountInfo<'info>,
    /// CHECK: Safe asks Account
    #[account(mut)]
    pub serum_asks: AccountInfo<'info>,
    /// CHECK: Safe event_q Account
    #[account(mut)]
    pub serum_event_queue: AccountInfo<'info>,
    /// CHECK: Safe Serum coin vault account
    #[account(mut)]
    pub serum_coin_vault: AccountInfo<'info>,
    /// CHECK: Safe Serum pc vault account
    #[account(mut)]
    pub serum_pc_vault: AccountInfo<'info>,
    /// CHECK: Safe Serum vault signer account
    pub serum_vault_signer: AccountInfo<'info>,
    /// CHECK: Safe User source token Account. user Account to swap from.
    #[account(mut)]
    pub user_source_token: AccountInfo<'info>,
    /// CHECK: Safe User destination token Account. user Account to swap to.
    #[account(mut)]
    pub user_destination_token: AccountInfo<'info>,
    /// CHECK: Safe User owner Account
    #[account(signer)]
    pub user_source_owner: AccountInfo<'info>,
    /// CHECK: Safe Spl token program
    #[account(address = spl_token::ID)]
    pub spl_token_program: AccountInfo<'info>,
}
