use crate::api::models::transaction::{TransferTransaction, Transaction};

pub type TransactionU64 = Transaction<u64, u64>;

pub type TransferTransactionU64 = TransferTransaction<u64, u64>;
