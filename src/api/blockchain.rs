use crate::api::models::blockchain::BlockchainData;
use crate::api::Result;
use crate::http::client::Client;

pub struct Blockchain {
    client: Client,
}

impl Blockchain {
    pub fn new(client: Client) -> Blockchain {
        Blockchain { client }
    }

    pub async fn state(&mut self) -> Result<BlockchainData> {
        self.client.get("blockchain").await
    }
}
