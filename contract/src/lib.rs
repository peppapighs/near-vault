use msg::FeeMessage;
use near_contract_standards::fungible_token::core::ext_ft_core;
use near_contract_standards::storage_management::StorageBalance;
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::LookupMap;
use near_sdk::json_types::U128;
use near_sdk::{
    env, log, near_bindgen, require, AccountId, Balance, PanicOnDefault, Promise, StorageUsage,
};

pub mod msg;
pub mod receiver;
pub mod storage;
mod test;

const ACCOUNT_NAME_MAX_LENGTH: usize = 256;

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

    // Storage staking mechanism
    pub user_storage_usage: StorageUsage,
    pub account_storage_usage: StorageUsage,
    pub storage_balances: LookupMap<AccountId, StorageBalance>,
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
        let mut this = Self {
            owner_id,
            token_id,
            transfer_fee_numerator,
            transfer_fee_denominator,
            total_transfer_fee: 0,
            accounts: LookupMap::new(b"a".to_vec()),
            user_accounts: LookupMap::new(b"u".to_vec()),
            user_storage_usage: 0,
            account_storage_usage: 0,
            storage_balances: LookupMap::new(b"s".to_vec()),
        };
        this.measure_account_storage_usage();
        this
    }

    // Create new account with unique account name
    pub fn create_account(&mut self, account_name: String) {
        require!(
            self.user_accounts.contains_key(&env::signer_account_id()),
            format!("The user {} is not registered", env::signer_account_id())
        );

        // Account name must not be longer than ACCOUNT_NAME_MAX_LENGTH
        require!(
            account_name.len() <= ACCOUNT_NAME_MAX_LENGTH,
            "Account name too long"
        );

        // Account name must be unique
        require!(
            !self.accounts.contains_key(&account_name),
            "Account already exists"
        );

        self.internal_create_account(env::signer_account_id(), account_name);
    }

    // Withdraw tokens from account
    pub fn withdraw(&mut self, account_name: String, amount: U128) -> Promise {
        require!(
            self.user_accounts.contains_key(&env::signer_account_id()),
            format!("The user {} is not registered", env::signer_account_id())
        );

        // Get account by account name
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
    #[payable]
    pub fn transfer(
        &mut self,
        sender_account_name: String,
        receiver_account_name: String,
        amount: U128,
    ) {
        require!(
            self.user_accounts.contains_key(&env::signer_account_id()),
            format!("The user {} is not registered", env::signer_account_id())
        );

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
                .unwrap_or(0)
                .checked_div(self.transfer_fee_denominator)
                .unwrap_or(0);
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
}

#[near_bindgen]
impl Contract {
    // Get contract fee
    pub fn get_fees(&self) -> FeeMessage {
        FeeMessage {
            transfer_fee_numerator: self.transfer_fee_numerator,
            transfer_fee_denominator: self.transfer_fee_denominator,
        }
    }

    // Get balance of an account
    pub fn get_balance(&self, account_name: String) -> U128 {
        // Get account by account account name
        self.accounts
            .get(&account_name)
            .unwrap_or_else(|| panic!("Account does not exist"))
            .balance
            .into()
    }
}

impl Contract {
    fn measure_account_storage_usage(&mut self) {
        let initial_storage_usage = env::storage_usage();
        let tmp_account_id = AccountId::new_unchecked("a".repeat(64));
        let tmp_account_name = "a".repeat(ACCOUNT_NAME_MAX_LENGTH);

        // Calculate storage usage for new user
        self.user_accounts.insert(&tmp_account_id, &vec![]);
        self.storage_balances.insert(
            &tmp_account_id,
            &StorageBalance {
                total: 0.into(),
                available: 0.into(),
            },
        );
        self.user_storage_usage = env::storage_usage() - initial_storage_usage;

        // Calculate storage usage for new account
        let initial_storage_usage = env::storage_usage();
        self.accounts.insert(
            &tmp_account_name,
            &Account {
                owner_id: tmp_account_id.clone(),
                balance: 0,
            },
        );
        self.user_accounts
            .insert(&tmp_account_id, &vec![tmp_account_name.clone()]);
        self.account_storage_usage = env::storage_usage() - initial_storage_usage;

        // Clean up
        self.accounts.remove(&tmp_account_name);
        self.user_accounts.remove(&tmp_account_id);
        self.storage_balances.remove(&tmp_account_id);

        log!(
            "Storage usage => new user: {} bytes, new account: {} bytes, cost per byte: {}",
            self.user_storage_usage,
            self.account_storage_usage,
            env::storage_byte_cost()
        );
    }
}

#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault, PartialEq, Debug)]
pub struct Account {
    pub owner_id: AccountId,
    pub balance: Balance,
}
