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

    // User's Account ID -> User's Accounts
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
}

pub struct Account {
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
        let _contract = Contract::new(accounts(1), accounts(2));
    }

    #[test]
    #[should_panic(expected = "The contract is not initialized")]
    fn test_default() {
        let context = get_context(accounts(1));
        testing_env!(context.build());
        let _contract = Contract::default();
    }
}
