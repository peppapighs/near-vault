use crate::msg::{DepositPayload, TransferMessage};
use crate::{Contract, ContractExt};
use near_contract_standards::fungible_token::receiver::FungibleTokenReceiver;
use near_sdk::borsh::BorshDeserialize;
use near_sdk::json_types::U128;
use near_sdk::{base64, env, near_bindgen, require, AccountId, PromiseOrValue};

#[near_bindgen]
impl FungibleTokenReceiver for Contract {
    // Receiver for NEP-141 token transfer
    #[allow(unused_variables)]
    fn ft_on_transfer(
        &mut self,
        sender_id: AccountId,
        amount: U128,
        msg: String,
    ) -> PromiseOrValue<U128> {
        // Contract caller must be the specified token
        require!(
            env::predecessor_account_id() == self.metadata.token_id,
            "Unsupported token type"
        );

        // Parse JSON message into TransferMessage and match each action
        let decoded_message =
            base64::decode(&msg).unwrap_or_else(|_| panic!("Invalid transfer message format"));
        let message = TransferMessage::try_from_slice(&decoded_message[..])
            .unwrap_or_else(|_| panic!("Invalid transfer message format"));
        match message.action.as_str() {
            "deposit" => {
                let payload = DepositPayload::try_from_slice(&message.payload[..])
                    .unwrap_or_else(|_| panic!("Invalid deposit payload format"));
                self.internal_deposit(payload.account_name, amount);

                // Return 0 as we transfer all the tokens to the account
                PromiseOrValue::Value(0.into())
            }
            _ => panic!("Unsupported action"),
        }
    }
}

impl Contract {
    // Callback function for depositing tokens
    pub fn internal_deposit(&mut self, account_name: String, amount: U128) {
        // Get account by account account_name
        let mut account = self
            .accounts
            .get(&account_name)
            .unwrap_or_else(|| panic!("Account does not exist"));
        // Add amount to account balance
        account.balance = account
            .balance
            .checked_add(amount.into())
            .unwrap_or_else(|| panic!("Balance overflow"));
        self.accounts.insert(&account_name, &account);
    }
}
