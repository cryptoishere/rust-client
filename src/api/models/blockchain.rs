use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct BlockchainData {
    pub block: BlockInfo,
    pub supply: String,
}

#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct BlockInfo {
    pub height: u64,
    pub id: String,
}
