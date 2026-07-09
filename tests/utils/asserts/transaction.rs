use crate::utils::asserts::shared::assert_timestamp_data;
use arkecosystem_client::api::models::lock::Lock;
use arkecosystem_client::api::models::transaction::TransactionPostResponse;
use arkecosystem_client::types::models::{TransactionU64, TransferTransactionU64};
use serde_json::Value;
use std::borrow::Borrow;
use std::str::FromStr;

pub fn assert_transfer_transaction_data(actual: TransferTransactionU64, expected: &Value) {
    assert_eq!(actual.id, expected["id"].as_str().unwrap());
    assert_eq!(actual.block_id, expected["blockId"].as_str().unwrap());

    assert_eq!(actual.version, expected["version"].as_u64().unwrap() as u8);

    assert_eq!(
        actual.r#type as u64,
        expected["type"].as_u64().unwrap()
    );

    if let Some(type_group) = actual.type_group {
        assert_eq!(type_group, expected["typeGroup"].as_u64().unwrap() as u8);
    }

    assert_eq!(
        actual.amount,
        u64::from_str(expected["amount"].as_str().unwrap()).unwrap()
    );

    assert_eq!(
        actual.fee,
        u64::from_str(expected["fee"].as_str().unwrap()).unwrap()
    );

    assert_eq!(actual.sender, expected["sender"].as_str().unwrap());

    assert_eq!(
        actual.sender_public_key,
        expected["senderPublicKey"].as_str().unwrap()
    );

    assert_eq!(actual.recipient, expected["recipient"].as_str().unwrap());

    assert_eq!(actual.signature, expected["signature"].as_str().unwrap());
    if let Some(vendor_field) = actual.vendor_field {
        assert_eq!(vendor_field, expected["vendorField"].as_str().unwrap());
    }

    assert_eq!(actual.confirmations, expected["confirmations"].as_u64().unwrap());

    if let Some(timestamp) = actual.timestamp {
        assert_timestamp_data(&timestamp, &expected["timestamp"].clone());
    }

    assert_eq!(actual.nonce, expected["nonce"].as_str().unwrap());
}

pub fn assert_transaction_data(actual: TransactionU64, expected: &Value) {
    assert_eq!(actual.id, expected["id"].as_str().unwrap());
    if let Some(block_id) = actual.block_id {
        assert_eq!(block_id, expected["blockId"].as_str().unwrap());
    }

    assert_eq!(actual.version, expected["version"].as_u64().unwrap() as u8);

    assert_eq!(
        actual.r#type as u64,
        expected["type"].as_u64().unwrap()
    );

    if let Some(type_group) = actual.type_group {
        assert_eq!(type_group, expected["typeGroup"].as_u64().unwrap() as u8);
    }

    assert_eq!(
        actual.amount,
        u64::from_str(expected["amount"].as_str().unwrap()).unwrap()
    );

    assert_eq!(
        actual.fee,
        u64::from_str(expected["fee"].as_str().unwrap()).unwrap()
    );

    if let Some(sender) = actual.sender {
        assert_eq!(sender, expected["sender"].as_str().unwrap());
    }

    assert_eq!(
        actual.sender_public_key,
        expected["senderPublicKey"].as_str().unwrap()
    );

    assert_eq!(actual.recipient, expected["recipient"].as_str().unwrap());

    assert_eq!(actual.signature, expected["signature"].as_str().unwrap());
    if let Some(vendor_field) = actual.vendor_field {
        assert_eq!(vendor_field, expected["vendorField"].as_str().unwrap());
    }

    if let Some(confirmations) = actual.confirmations {
        assert_eq!(confirmations, expected["confirmations"].as_u64().unwrap());
    }

    if let Some(timestamp) = actual.timestamp {
        assert_timestamp_data(&timestamp, &expected["timestamp"].clone());
    }
    if let Some(nonce) = actual.nonce {
        assert_eq!(nonce, expected["nonce"].as_str().unwrap());
    }
}

pub fn assert_transaction_post_data(actual: TransactionPostResponse, expected: &Value) {
    for (pos, value) in actual.accept.iter().enumerate() {
        assert_eq!(value, &expected["accept"][pos]);
    }
    for (pos, value) in actual.broadcast.iter().enumerate() {
        assert_eq!(value, &expected["broadcast"][pos]);
    }
    for (pos, value) in actual.excess.iter().enumerate() {
        assert_eq!(value, &expected["excess"][pos]);
    }
    for (pos, value) in actual.invalid.iter().enumerate() {
        assert_eq!(value, &expected["invalid"][pos]);
    }
}

pub fn assert_vote_data(actual: TransactionU64, expected: &Value) {
    assert_transaction_data(actual, &expected);
}

pub fn assert_lock_data(actual: Lock, expected: &Value) {
    assert_eq!(actual.lock_id, expected["lockId"].as_str().unwrap());
    assert_eq!(
        actual.amount,
        u64::from_str(expected["amount"].as_str().unwrap()).unwrap()
    );
    assert_eq!(actual.secret_hash, expected["secretHash"].as_str().unwrap());
    assert_eq!(
        actual.sender_public_key,
        expected["senderPublicKey"].as_str().unwrap()
    );
    assert_eq!(
        actual.recipient_id,
        expected["recipientId"].as_str().unwrap()
    );
    if actual.vendor_field.is_some() {
        assert_eq!(
            actual.vendor_field.unwrap(),
            expected["vendorField"].as_str().unwrap()
        );
    }
    assert_eq!(
        actual.expiration_type as u64,
        expected["expirationType"].as_u64().unwrap()
    );
    assert_eq!(
        actual.expiration_value as u64,
        expected["expirationValue"].as_u64().unwrap()
    );
    assert_timestamp_data(&actual.timestamp, expected["timestamp"].borrow());
    if actual.is_expired.is_some() {
        assert_eq!(
            actual.is_expired.unwrap(),
            expected["isExpired"].as_bool().unwrap()
        );
    }
}

pub fn test_transfer_transaction_array(actual: Vec<TransferTransactionU64>, expected: Value) {
    for (pos, trx) in actual.iter().enumerate() {
        assert_transfer_transaction_data(trx.clone(), &expected["data"][pos]);
    }
}

pub fn test_transaction_array(actual: Vec<TransactionU64>, expected: Value) {
    for (pos, trx) in actual.iter().enumerate() {
        assert_transaction_data(trx.clone(), &expected["data"][pos]);
    }
}

pub fn test_vote_array(actual: Vec<TransactionU64>, expected: Value) {
    for (pos, vote_trx) in actual.iter().enumerate() {
        assert_vote_data(vote_trx.clone(), &expected["data"][pos]);
    }
}

pub fn test_lock_array(actual: Vec<Lock>, expected: Value) {
    for (pos, lock) in actual.iter().enumerate() {
        assert_lock_data(lock.clone(), &expected["data"][pos]);
    }
}
