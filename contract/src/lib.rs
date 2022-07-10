use msg::{DepositPayload, TransferMessage};
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::LookupMap;
use near_sdk::json_types::U128;
use near_sdk::serde_json;
use near_sdk::{env, near_bindgen, require, AccountId, Balance, PanicOnDefault, PromiseOrValue};

mod msg;

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Contract {
    // Contract Owner's Account ID
    pub owner_id: AccountId,

    // NEP-21 Token Account ID
    pub token_id: AccountId,

    // Account name -> Account
    pub accounts: LookupMap<String, Account>,

    // User's Account ID -> List of account names
    pub user_accounts: LookupMap<AccountId, Vec<String>>,
}

#[near_bindgen]
impl Contract {
    // Contract initialize function
    #[init]
    pub fn new(owner_id: AccountId, token_id: AccountId) -> Self {
        require!(!env::state_exists(), "Already initialized");
        Self {
            owner_id,
            token_id,
            accounts: LookupMap::new(b"a".to_vec()),
            user_accounts: LookupMap::new(b"u".to_vec()),
        }
    }

    // Create new account with unique name
    pub fn create_account(&mut self, name: String) {
        // Account name must be unique
        require!(!self.accounts.contains_key(&name), "Account already exists");

        // Create new empty account
        self.accounts.insert(
            &name,
            &(Account {
                owner_id: env::signer_account_id().clone(),
                name: name.clone(),
                balance: 0u128,
            }),
        );

        // Add account to user's list of accounts
        let mut user_account = self
            .user_accounts
            .get(&env::signer_account_id())
            .unwrap_or(vec![]);
        user_account.push(name.clone());
        self.user_accounts
            .insert(&env::signer_account_id(), &user_account);
    }

    // Receiver for NEP-21 token transfer
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

    // Get balance of an account
    pub fn get_balance(&self, name: String) -> U128 {
        // Get account by account name
        self.accounts
            .get(&name)
            .unwrap_or_else(|| panic!("Account does not exist"))
            .balance
            .into()
    }

    // Callback function for depositing tokens
    #[private]
    fn deposit(&mut self, account_name: String, amount: U128) {
        // Get account by account name
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
    pub name: String,
    pub balance: Balance,
}

#[cfg(test)]
mod tests {
    use super::*;
    use near_sdk::test_utils::{accounts, VMContextBuilder};
    use near_sdk::{testing_env, AccountId};

    fn get_context(predecessor_account_id: AccountId) -> VMContextBuilder {
        let mut builder = VMContextBuilder::new();
        builder
            .current_account_id(accounts(0))
            .signer_account_id(predecessor_account_id.clone())
            .predecessor_account_id(predecessor_account_id);
        builder
    }

    #[test]
    fn test_new() {
        let context = get_context(accounts(1));
        testing_env!(context.build());
        Contract::new(accounts(1), accounts(2));
    }

    #[test]
    #[should_panic(expected = "The contract is not initialized")]
    fn test_default() {
        let context = get_context(accounts(1));
        testing_env!(context.build());
        Contract::default();
    }

    #[test]
    fn test_create_account() {
        let context = get_context(accounts(1));
        testing_env!(context.build());
        let mut contract = Contract::new(accounts(1), accounts(2));

        contract.create_account("account".into());
        let account = contract.accounts.get(&"account".to_owned()).unwrap();
        assert_eq!(account.owner_id, accounts(1));
        assert_eq!(account.name, "account".to_owned());
        assert_eq!(account.balance, 0u128);
        assert_eq!(contract.get_balance("account".to_owned()), 0.into());
        assert_eq!(
            contract.user_accounts.get(&accounts(1)).unwrap(),
            vec!["account"]
        );
    }

    #[test]
    #[should_panic(expected = "Account already exists")]
    fn test_create_duplicate_account() {
        let context = get_context(accounts(1));
        testing_env!(context.build());
        let mut contract = Contract::new(accounts(1), accounts(2));

        contract.create_account("account".into());
        contract.create_account("account".into());
    }

    #[test]
    #[should_panic(expected = "The contract is not initialized")]
    fn test_default_account() {
        Account::default();
    }

    #[test]
    #[should_panic(expected = "Account does not exist")]
    fn test_get_balance_non_existent_account() {
        let context = get_context(accounts(1));
        testing_env!(context.build());
        let contract = Contract::new(accounts(1), accounts(2));

        contract.get_balance("account".into());
    }

    #[test]
    fn test_deposit() {
        let context = get_context(accounts(1));
        testing_env!(context.build());
        let mut contract = Contract::new(accounts(1), accounts(2));

        contract.create_account("account".into());
        let context = get_context(accounts(2));
        testing_env!(context.build());
        contract.ft_on_transfer(
            accounts(1),
            1.into(),
            "{\"action\":\"deposit\",\"payload\":\"{\\\"account_name\\\":\\\"account\\\"}\"}"
                .to_owned(),
        );
        assert_eq!(contract.get_balance("account".to_owned()), 1.into());
    }

    #[test]
    #[should_panic(expected = "Unsupported token type")]
    fn test_ft_on_transfer_unsupported_token() {
        let context = get_context(accounts(1));
        testing_env!(context.build());
        let mut contract = Contract::new(accounts(1), accounts(2));

        contract.ft_on_transfer(accounts(1), 1.into(), "{}".to_owned());
    }

    #[test]
    #[should_panic(expected = "Invalid transfer message format")]
    fn test_ft_on_transfer_invalid_transfer_message_format() {
        let context = get_context(accounts(1));
        testing_env!(context.build());
        let mut contract = Contract::new(accounts(1), accounts(2));

        let context = get_context(accounts(2));
        testing_env!(context.build());
        contract.ft_on_transfer(accounts(1), 1.into(), "{}".to_owned());
    }

    #[test]
    #[should_panic(expected = "Unsupported action")]
    fn test_ft_on_transfer_unsupported_action() {
        let context = get_context(accounts(1));
        testing_env!(context.build());
        let mut contract = Contract::new(accounts(1), accounts(2));

        let context = get_context(accounts(2));
        testing_env!(context.build());
        contract.ft_on_transfer(
            accounts(1),
            1.into(),
            "{\"action\":\"\",\"payload\":\"\"}".to_owned(),
        );
    }

    #[test]
    #[should_panic(expected = "Invalid deposit payload format")]
    fn test_ft_on_transfer_invalid_deposit_payload_format() {
        let context = get_context(accounts(1));
        testing_env!(context.build());
        let mut contract = Contract::new(accounts(1), accounts(2));

        let context = get_context(accounts(2));
        testing_env!(context.build());
        contract.ft_on_transfer(
            accounts(1),
            1.into(),
            "{\"action\":\"deposit\",\"payload\":\"\"}".to_owned(),
        );
    }
}
