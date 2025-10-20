use crate::api::models::blockchain::Blockchain as BlockchainState;
use crate::api::Result;
use crate::http::client::Client;

pub struct Blockchain {
    client: Client,
}

impl Blockchain {
    pub fn new(client: Client) -> Blockchain {
        Blockchain { client }
    }

    pub async fn state(&mut self) -> Result<BlockchainState> {
        self.client.get("blockchain").await
    }
}
