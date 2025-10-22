use std::collections::HashMap;
use serde::{Deserialize, Serialize};

use crate::api::models::transaction::TransactionFeesCore;
use crate::common::deserialize_as_u64_from_number_or_string;

#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NodeConfiguration {
    pub core: HashMap<String, String>,
    #[serde(rename = "nethash")]
    pub nethash: String,
    pub slip44: u32,
    pub wif: u32,
    pub token: String,
    pub symbol: String,
    pub explorer: String,
    pub version: u32,
    pub ports: HashMap<String, Option<u16>>,
    pub constants: NodeConstants,
    pub transaction_pool: TransactionPool,
}

#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NodeStatus {
    pub synced: bool,
    pub now: u64,
    pub blocks_count: i64,
    pub timestamp: u64,
}

#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct NodeSyncing {
    pub syncing: bool,
    pub blocks: i64,
    pub height: u64,
    pub id: String,
}

#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NodeConstants {
    pub height: u64,
    #[serde(deserialize_with = "deserialize_as_u64_from_number_or_string")]
    pub reward: u64,
    pub active_delegates: u32,
    pub blocktime: u32,
    pub block: NodeBlock,
    pub epoch: String,
    pub fees: Fees,
    pub vendor_field_length: u32,
    pub multi_payment_limit: u32,
    pub htlc_enabled: bool,
    pub block_burn_address: bool,
    pub aip11: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aip37: Option<bool>,
}

#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NodeBlock {
    pub version: u32,
    pub max_transactions: u64,
    pub max_payload: u64,
    pub id_full_sha256: bool,
}

#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Fees {
    pub static_fees: TransactionFeesCore,
}

#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DynamicFees {
    pub enabled: bool,
    pub min_fee_pool: u64,
    pub min_fee_broadcast: u64,
    pub addon_bytes: TransactionFeesCore,
}

#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TransactionPool {
    pub dynamic_fees: DynamicFees,
    max_transactions_in_pool: u32,
    max_transactions_per_sender: u32,
    max_transactions_per_request: u32,
    max_transaction_age: u32,
    max_transaction_bytes: u32,
}
