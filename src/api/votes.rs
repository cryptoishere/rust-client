use std::borrow::Borrow;

use crate::api::Result;
use crate::http::client::Client;
use crate::types::models::TransactionU64;

pub struct Votes {
    client: Client,
}

impl Votes {
    pub fn new(client: Client) -> Votes {
        Votes { client }
    }

    pub async fn all(&mut self) -> Result<Vec<TransactionU64>> {
        self.all_params(Vec::<(String, String)>::new()).await
    }

    pub async fn all_params<I, K, V>(&mut self, parameters: I) -> Result<Vec<TransactionU64>>
    where
        I: IntoIterator,
        I::Item: Borrow<(K, V)>,
        K: AsRef<str>,
        V: AsRef<str>,
    {
        self.client.get_with_params("votes", parameters).await
    }

    pub async fn show(&mut self, id: &str) -> Result<TransactionU64> {
        let endpoint = format!("votes/{}", id);
        self.client.get(&endpoint).await
    }
}
