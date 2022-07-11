use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};

#[derive(BorshDeserialize, BorshSerialize)]
pub struct TransferMessage {
    pub action: String,
    pub payload: Vec<u8>,
}

#[derive(BorshDeserialize, BorshSerialize)]
pub struct DepositPayload {
    pub account_name: String,
}
