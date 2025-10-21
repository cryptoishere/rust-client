use std::collections::HashMap;

use serde::Deserialize;
use serde::Serialize;

use crate::common::deserialize_as_u64_from_number_or_string;

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct Wallet {
    pub address: String,
    #[serde(rename(deserialize = "publicKey", serialize = "publicKey"))]
    pub public_key: Option<String>,
    #[serde(deserialize_with = "deserialize_as_u64_from_number_or_string")]
    pub balance: u64,
    #[serde(deserialize_with = "deserialize_as_u64_from_number_or_string")]
    pub nonce: u64,
    pub attributes: HashMap<String, serde_json::Value>,
}

pub type Balances = HashMap<String, u64>;
