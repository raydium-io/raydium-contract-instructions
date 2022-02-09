use solana_program::pubkey::Pubkey;

/// well known  JSON RPC API DNS
pub const CACHE_RCP_AMM: &'static str = "https://api.raydium.io/cache/rpc/amm";

/// well known SDK DNS
pub const LIQUIDITY: &'static str = "https://sdk.raydium.io/liquidity/mainnet.json";

// well known Raydium pool
// https://solscan.io/tx/3JjYbidNvzVXAL8zHtTw7QvFSjk5PELhvzXsHyQ5jFD2jqzdjBcxZGmybjkpSVBsWJBQYPKgEVrGL4ixnkxTXSxQ
// https://solscan.io/tx/X9eP2qK3Z95UZXDpUeUTd21uorkWrpy4FKzZHK5EZSykDYNBtb4fu4nfowoyWcGbjL6s1swCLE9rsFfjQZS51Ec
pub const RAY_SOL: Pubkey = solana_program::pubkey!("AVs9TA4nWDzfPJE9gGVNJMVhcQy3V9PGazuz33BfG2RA");
