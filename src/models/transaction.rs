use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct TransactionModel {
    pub id: i32,
    pub amount: i32,
}
