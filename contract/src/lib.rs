use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::LookupMap;
use near_sdk::{env, near_bindgen, require, AccountId, Balance, PanicOnDefault};

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

    pub fn get_account(&self, name: String) -> Account {
        // Get account by account name
        self.accounts
            .get(&name)
            .unwrap_or_else(|| panic!("Account does not exist"))
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
        let account = contract.get_account("account".into());
        assert_eq!(account.owner_id, accounts(1));
        assert_eq!(account.name, "account".to_string());
        assert_eq!(account.balance, 0u128);
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
}
