use near_contract_standards::fungible_token::core::ext_ft_core;
use near_contract_standards::storage_management::{StorageBalance, StorageManagement};
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::LookupMap;
use near_sdk::json_types::{U128, U64};
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::{
    assert_one_yocto, env, near_bindgen, require, AccountId, Balance, PanicOnDefault, Promise,
};

pub mod msg;
pub mod receiver;
pub mod storage;
mod test;

const ACCOUNT_NAME_MAX_LENGTH: usize = 256;

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Contract {
    // Contract metadata
    pub metadata: ContractMetadata,

    // Total transfer fee from cross-owner transfers
    pub total_transfer_fee: Balance,

    // Account account_name -> Account
    pub accounts: LookupMap<String, Account>,

    // User's Account ID -> List of account names
    pub user_accounts: LookupMap<AccountId, Vec<String>>,

    // Storage staking balance
    pub storage_balances: LookupMap<AccountId, StorageBalance>,
}

#[near_bindgen]
impl Contract {
    // Contract initialize function
    #[init]
    pub fn new(
        owner_id: AccountId,
        token_id: AccountId,
        transfer_fee_numerator: U128,
        transfer_fee_denominator: U128,
    ) -> Self {
        require!(!env::state_exists(), "Already initialized");
        let mut this = Self {
            metadata: ContractMetadata {
                owner_id,
                token_id,
                transfer_fee_numerator,
                transfer_fee_denominator,
                user_storage_usage: 0.into(),
                account_storage_usage: 0.into(),
            },
            total_transfer_fee: 0,
            accounts: LookupMap::new(b"a".to_vec()),
            user_accounts: LookupMap::new(b"u".to_vec()),
            storage_balances: LookupMap::new(b"s".to_vec()),
        };
        this.measure_account_storage_usage();
        this
    }

    // Create new account with unique account name
    #[payable]
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

        // User may attach deposit to create new account
        self.storage_deposit(Some(env::signer_account_id()), None);

        // Create new account
        self.internal_create_account(env::signer_account_id(), account_name);
    }

    // Withdraw tokens from account
    #[payable]
    pub fn withdraw(&mut self, account_name: String, amount: U128) -> Option<Promise> {
        assert_one_yocto();
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

        // Contract owner cannot withdraw tokens from itself
        if env::current_account_id() != env::signer_account_id() {
            // Call token contract to transfer token to caller
            Some(
                ext_ft_core::ext(self.metadata.token_id.clone())
                    .with_attached_deposit(1)
                    .ft_transfer(env::signer_account_id(), amount, None),
            )
        } else {
            None
        }
    }

    // Transfer tokens to another account
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
                .checked_mul(self.metadata.transfer_fee_numerator.into())
                .unwrap_or(0)
                .checked_div(self.metadata.transfer_fee_denominator.into())
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

    // Withdraw all fees to contract owner
    #[payable]
    pub fn withdraw_transfer_fee(&mut self, amount: U128) -> Option<Promise> {
        assert_one_yocto();
        require!(
            env::signer_account_id() == self.metadata.owner_id,
            "Unauthorized access"
        );
        require!(
            env::current_account_id() != self.metadata.owner_id,
            "Contract cannot withdraw from itself"
        );

        self.total_transfer_fee = self
            .total_transfer_fee
            .checked_sub(amount.into())
            .unwrap_or_else(|| panic!("Balance overflow"));

        // Contract owner cannot withdraw tokens from itself
        if env::current_account_id() != env::signer_account_id() {
            // Transfer fees to owner
            Some(
                ext_ft_core::ext(self.metadata.token_id.clone())
                    .with_attached_deposit(1)
                    .ft_transfer(self.metadata.owner_id.clone(), amount, None),
            )
        } else {
            None
        }
    }
}

#[near_bindgen]
impl Contract {
    // Get contract metadata
    pub fn get_metadata(&self) -> ContractMetadata {
        self.metadata.clone()
    }

    // Get list of all accounts owned by a user
    pub fn get_accounts(&self, account_id: AccountId) -> Option<Vec<String>> {
        self.user_accounts.get(&account_id)
    }

    // Get balance of an account
    pub fn get_balance(&self, account_name: String) -> Option<U128> {
        // Get account by account account name
        self.accounts
            .get(&account_name)
            .map(|account| account.balance.into())
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
        self.metadata.user_storage_usage = (env::storage_usage() - initial_storage_usage).into();

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
        self.metadata.account_storage_usage = (env::storage_usage() - initial_storage_usage).into();

        // Clean up
        self.accounts.remove(&tmp_account_name);
        self.user_accounts.remove(&tmp_account_id);
        self.storage_balances.remove(&tmp_account_id);
    }
}

#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault, PartialEq, Debug)]
pub struct Account {
    pub owner_id: AccountId,
    pub balance: Balance,
}

#[derive(
    BorshDeserialize,
    BorshSerialize,
    Serialize,
    Deserialize,
    Clone,
    PanicOnDefault,
    PartialEq,
    Debug,
)]
#[serde(crate = "near_sdk::serde")]
pub struct ContractMetadata {
    // Contract Owner's Account ID
    pub owner_id: AccountId,

    // NEP-141 Token Account ID
    pub token_id: AccountId,

    // Transfer fee for cross-owner transfer
    pub transfer_fee_numerator: U128,
    pub transfer_fee_denominator: U128,

    // Storage usage
    pub user_storage_usage: U64,
    pub account_storage_usage: U64,
}
