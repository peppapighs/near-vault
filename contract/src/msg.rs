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

#[derive(Deserialize, Serialize, PartialEq, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct FeeMessage {
    pub transfer_fee_numerator: u128,
    pub transfer_fee_denominator: u128,
}