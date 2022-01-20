// TODO: if not smart contract - bpf or option with rpc
// https://github.com/raydium-io/raydium-sdk
use serde_derive::Deserialize;
use serde_derive::Serialize;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AmmCacheRpc {
    pub jsonrpc: String,
    pub result: Vec<Result>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Result {
    pub account: Account,
    pub pubkey: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Account {
    pub data: Vec<String>,
    pub executable: bool,
    pub lamports: i64,
    pub owner: String,
    pub rent_epoch: i64,
}


// SDK
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SdkPoolRoot {
    pub name: String,
    pub timestamp: String,
    pub version: Version,
    pub official: Vec<Official>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Version {
    pub major: u16,
    pub minor: u16,
    pub patch: u16,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Official {
    pub id: String,
    pub base_mint: String,
    pub quote_mint: String,
    pub lp_mint: String,
    pub version: u16,
    pub program_id: String,
    pub authority: String,
    pub open_orders: String,
    pub target_orders: String,
    pub base_vault: String,
    pub quote_vault: String,
    pub withdraw_queue: String,
    pub lp_vault: String,
    pub market_version: u16,
    pub market_program_id: String,
    pub market_id: String,
    pub market_authority: String,
    pub market_base_vault: String,
    pub market_quote_vault: String,
    pub market_bids: String,
    pub market_asks: String,
    pub market_event_queue: String,
}
