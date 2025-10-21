use serde_json::Value;
use std::str::FromStr;

use arkecosystem_client::api::models::wallet::Wallet;

pub fn assert_wallet_data(actual: Wallet, expected: &Value) {
    assert_eq!(actual.address, expected["address"].as_str().unwrap());
    if let Some(public_key) = actual.public_key {
        assert_eq!(public_key, expected["publicKey"].as_str().unwrap());
    }
    assert_eq!(
        actual.nonce,
        u64::from_str(expected["nonce"].as_str().unwrap()).unwrap()
    );
    assert_eq!(
        actual.balance,
        u64::from_str(expected["balance"].as_str().unwrap()).unwrap()
    );
}

pub fn test_wallet_array(actual: Vec<Wallet>, expected: Value) {
    for (pos, wallet) in actual.iter().enumerate() {
        assert_wallet_data(wallet.clone(), &expected["data"][pos]);
    }
}
