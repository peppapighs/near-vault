use msg::{DepositPayload, FeeMessage, TransferMessage};
use near_contract_standards::fungible_token::core::ext_ft_core;
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::LookupMap;
use near_sdk::json_types::U128;
use near_sdk::{
    env, near_bindgen, require, serde_json, AccountId, Balance, PanicOnDefault, Promise,
    PromiseOrValue,
};

mod msg;
mod test;

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Contract {
    // Contract Owner's Account ID
    pub owner_id: AccountId,

    // NEP-141 Token Account ID
    pub token_id: AccountId,

    // Transfer fee for cross-owner transfer
    pub transfer_fee_numerator: u128,
    pub transfer_fee_denominator: u128,
    pub total_transfer_fee: Balance,

    // Account account_name -> Account
    pub accounts: LookupMap<String, Account>,

    // User's Account ID -> List of account names
    pub user_accounts: LookupMap<AccountId, Vec<String>>,
}

#[near_bindgen]
impl Contract {
    // Contract initialize function
    #[init]
    pub fn new(
        owner_id: AccountId,
        token_id: AccountId,
        transfer_fee_numerator: u128,
        transfer_fee_denominator: u128,
    ) -> Self {
        require!(!env::state_exists(), "Already initialized");
        Self {
            owner_id,
            token_id,
            transfer_fee_numerator,
            transfer_fee_denominator,
            total_transfer_fee: 0u128,
            accounts: LookupMap::new(b"a".to_vec()),
            user_accounts: LookupMap::new(b"u".to_vec()),
        }
    }

    // Create new account with unique account_name
    pub fn create_account(&mut self, account_name: String) {
        // Account account_name must be unique
        require!(
            !self.accounts.contains_key(&account_name),
            "Account already exists"
        );

        // Create new empty account
        self.accounts.insert(
            &account_name,
            &(Account {
                owner_id: env::signer_account_id().clone(),
                balance: 0u128,
            }),
        );

        // Add account to user's list of accounts
        let mut user_account = self
            .user_accounts
            .get(&env::signer_account_id())
            .unwrap_or(vec![]);
        user_account.push(account_name.clone());
        self.user_accounts
            .insert(&env::signer_account_id(), &user_account);
    }

    // Receiver for NEP-141 token transfer
    pub fn ft_on_transfer(
        &mut self,
        _sender_id: AccountId,
        amount: U128,
        msg: String,
    ) -> PromiseOrValue<U128> {
        // Contract caller must be the specified token
        require!(
            env::predecessor_account_id() == self.token_id,
            "Unsupported token type"
        );

        // Parse JSON message into TransferMessage and match each action
        let message = serde_json::from_str::<TransferMessage>(&msg[..])
            .unwrap_or_else(|_| panic!("Invalid transfer message format"));
        match message.action.as_str() {
            "deposit" => {
                let payload = serde_json::from_str::<DepositPayload>(&message.payload[..])
                    .unwrap_or_else(|_| panic!("Invalid deposit payload format"));
                self.deposit(payload.account_name, amount);

                // Return 0 as we transfer all the tokens to the account
                PromiseOrValue::Value(0.into())
            }
            _ => panic!("Unsupported action"),
        }
    }

    // Withdraw tokens from account
    pub fn withdraw(&mut self, account_name: String, amount: U128) -> Promise {
        // Get account by account_name
        let mut account = self
            .accounts
            .get(&account_name)
            .unwrap_or_else(|| panic!("Account does not exist"));

        // Check if account owner is the same as the caller
        require!(
            account.owner_id == env::signer_account_id(),
            "Unauthorized access to account"
        );

        // Subtract amount from account balance
        account.balance = account
            .balance
            .checked_sub(amount.into())
            .unwrap_or_else(|| panic!("Balance overflow"));
        self.accounts.insert(&account_name, &account);

        // Call token contract to transfer token to caller
        ext_ft_core::ext(self.token_id.clone()).ft_transfer(env::signer_account_id(), amount, None)
    }

    // Transfer tokens to another account
    pub fn transfer(
        &mut self,
        sender_account_name: String,
        receiver_account_name: String,
        amount: U128,
    ) {
        // Get sender account by account name
        let mut sender_account = self
            .accounts
            .get(&sender_account_name)
            .unwrap_or_else(|| panic!("Sender account does not exist"));
        require!(
            sender_account.owner_id == env::signer_account_id(),
            "Unauthorized access to account"
        );

        // Get receiver account by account name
        let mut receiver_account = self
            .accounts
            .get(&receiver_account_name)
            .unwrap_or_else(|| panic!("Receiver account does not exist"));

        // Transfer tokens from sender to receiver
        sender_account.balance = sender_account
            .balance
            .checked_sub(amount.into())
            .unwrap_or_else(|| panic!("Balance overflow"));
        receiver_account.balance = receiver_account
            .balance
            .checked_add(amount.into())
            .unwrap_or_else(|| panic!("Balance overflow"));

        // If accounts have different owners, subtract transfer fee from receiver
        if receiver_account.owner_id != env::signer_account_id() {
            let transfer_fee = Balance::from(amount)
                .checked_mul(self.transfer_fee_numerator)
                .unwrap_or(0u128)
                .checked_div(self.transfer_fee_denominator)
                .unwrap_or(0u128);
            receiver_account.balance = receiver_account
                .balance
                .checked_sub(transfer_fee)
                .unwrap_or_else(|| panic!("Balance overflow"));

            self.total_transfer_fee = self
                .total_transfer_fee
                .checked_add(transfer_fee)
                .unwrap_or_else(|| panic!("Balance overflow"));
        }

        // Update accounts in storage
        self.accounts.insert(&sender_account_name, &sender_account);
        self.accounts
            .insert(&receiver_account_name, &receiver_account);
    }

    // Transfer all fees to owner
    pub fn transfer_fee_to_owner(&mut self, amount: U128) -> Promise {
        require!(
            env::signer_account_id() == self.owner_id,
            "Unauthorized access"
        );

        self.total_transfer_fee = self
            .total_transfer_fee
            .checked_sub(amount.into())
            .unwrap_or_else(|| panic!("Balance overflow"));

        // Transfer fees to owner
        ext_ft_core::ext(self.token_id.clone()).ft_transfer(self.owner_id.clone(), amount, None)
    }

    // Get contract fee
    pub fn get_fees(&self) -> FeeMessage {
        FeeMessage {
            transfer_fee_numerator: self.transfer_fee_numerator,
            transfer_fee_denominator: self.transfer_fee_denominator,
        }
    }

    // Get balance of an account
    pub fn get_balance(&self, account_name: String) -> U128 {
        // Get account by account account_name
        self.accounts
            .get(&account_name)
            .unwrap_or_else(|| panic!("Account does not exist"))
            .balance
            .into()
    }

    // Callback function for depositing tokens
    #[private]
    fn deposit(&mut self, account_name: String, amount: U128) {
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

#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Account {
    pub owner_id: AccountId,
    pub balance: Balance,
}
