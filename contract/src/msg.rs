use near_sdk::serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
#[serde(crate = "near_sdk::serde")]
pub struct TransferMessage {
    pub action: String,
    pub payload: String,
}

#[derive(Deserialize, Serialize)]
#[serde(crate = "near_sdk::serde")]
pub struct DepositPayload {
    pub account_name: String,
}
