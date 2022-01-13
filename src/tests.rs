use std::time::Duration;

use solana_client::rpc_client::RpcClient;
use solana_sdk::client::SyncClient;

use crate::state::LiquidityStateLayoutV4;
use borsh::{BorshSerialize, BorshDeserialize};

/// https://solscan.io/address/8UQuZxc7qTzPRCivTyBLKiBiCGVUAbRNWw5PkDZT5mVK
#[test]
fn state() {
    let ref pool = "8UQuZxc7qTzPRCivTyBLKiBiCGVUAbRNWw5PkDZT5mVK".parse().unwrap();
    let solana = RpcClient::new_with_timeout("https://api.mainnet-beta.solana.com".to_string(), Duration::from_secs(120));
    let pool = solana.get_account_data(pool).unwrap();
    let data = LiquidityStateLayoutV4::deserialize(&mut &pool[..]).unwrap();    
    assert_eq!(data.base_mint, "HKfs24UEDQpHS5hUyKYkHd9q7GY5UQ679q2bokeL2whu".parse().unwrap());
    assert_eq!(data.market_id, "998TAsB7D2FnFrri5YSVHfw7Ajgckz5skdu1qPPMFVSt".parse().unwrap());
    
    dbg!("{:?}", data);
} 
