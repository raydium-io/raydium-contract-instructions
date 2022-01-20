// TODO: if not smart contract - bpf or option with rpc
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
