use crate::{Account, Contract, ContractExt};
use near_contract_standards::storage_management::{
    StorageBalance, StorageBalanceBounds, StorageManagement,
};
use near_sdk::json_types::U128;
use near_sdk::{assert_one_yocto, env, log, near_bindgen, require, AccountId, Balance, Promise};

#[near_bindgen]
impl StorageManagement for Contract {
    #[payable]
    fn storage_deposit(
        &mut self,
        account_id: Option<AccountId>,
        registration_only: Option<bool>,
    ) -> StorageBalance {
        let amount = env::attached_deposit();
        let account_id = account_id.unwrap_or(env::predecessor_account_id());
        let registration_only = registration_only.unwrap_or(false);

        // Check if user has registered their account
        if self.user_accounts.contains_key(&account_id) {
            if registration_only {
                // Refund the deposit
                log!(
                    "The user {} is already registered, refunding the deposit",
                    account_id
                );
                if amount > 0 {
                    Promise::new(env::predecessor_account_id()).transfer(amount);
                }
            } else {
                // Add deposit to the storage balance
                let mut storage_balance = self.storage_balances.get(&account_id).unwrap();
                storage_balance.total = Balance::from(storage_balance.total)
                    .checked_add(amount)
                    .unwrap_or_else(|| panic!("Balance overflow"))
                    .into();
                storage_balance.available = Balance::from(storage_balance.available)
                    .checked_add(amount)
                    .unwrap_or_else(|| panic!("Balance overflow"))
                    .into();
                self.storage_balances.insert(&account_id, &storage_balance);
            }
        } else {
            // Attached deposit must be enough to create an account
            let min_balance = self.storage_balance_bounds().min.0;
            require!(
                amount >= min_balance,
                "Insufficient deposit to register a user"
            );

            // Create a new user entry with an empty list of associated accounts
            self.user_accounts.insert(&account_id, &vec![]);
            if registration_only {
                // Refund excess deposit on registration only
                let refund = amount - min_balance;
                if refund > 0 {
                    Promise::new(env::predecessor_account_id()).transfer(refund);
                }
                self.storage_balances.insert(
                    &account_id,
                    &StorageBalance {
                        total: min_balance.into(),
                        available: 0.into(),
                    },
                );
            } else {
                // Add deposit to the storage balance
                let storage_balance = StorageBalance {
                    total: amount.into(),
                    available: Balance::from(amount)
                        .checked_sub(min_balance)
                        .unwrap_or_else(|| panic!("Balance overflow"))
                        .into(),
                };
                self.storage_balances.insert(&account_id, &storage_balance);
            }
        }

        self.storage_balances.get(&account_id).unwrap()
    }

    #[payable]
    fn storage_withdraw(&mut self, amount: Option<U128>) -> StorageBalance {
        assert_one_yocto();
        let predecessor_account_id = env::predecessor_account_id();

        // Check if the user is registered
        let mut storage_balance = self
            .storage_balances
            .get(&predecessor_account_id)
            .unwrap_or_else(|| panic!("The user {} is not registered", predecessor_account_id));
        match amount {
            Some(amount) => {
                // Refund the requested amount
                storage_balance.total = Balance::from(storage_balance.total)
                    .checked_sub(amount.into())
                    .unwrap_or_else(|| panic!("Balance overflow"))
                    .into();
                storage_balance.available = Balance::from(storage_balance.available)
                    .checked_sub(amount.into())
                    .unwrap_or_else(|| panic!("Balance overflow"))
                    .into();
                if amount > 0.into() {
                    Promise::new(predecessor_account_id.clone()).transfer(amount.0 + 1);
                }
                self.storage_balances
                    .insert(&predecessor_account_id, &storage_balance);
                storage_balance
            }
            None => {
                // Refund the entire available balance
                if storage_balance.available > 0.into() {
                    Promise::new(predecessor_account_id.clone())
                        .transfer(storage_balance.available.0 + 1);
                }
                storage_balance.total = Balance::from(storage_balance.total)
                    .checked_sub(storage_balance.available.into())
                    .unwrap_or_else(|| panic!("Balance overflow"))
                    .into();
                storage_balance.available = 0.into();
                self.storage_balances
                    .insert(&predecessor_account_id, &storage_balance);
                storage_balance
            }
        }
    }

    #[payable]
    fn storage_unregister(&mut self, force: Option<bool>) -> bool {
        assert_one_yocto();
        let account_id = env::predecessor_account_id();
        let force = force.unwrap_or(false);

        // Check if the user is registered
        if let Some(accounts) = self.user_accounts.get(&account_id) {
            let storage_balance = self.storage_balances.get(&account_id).unwrap();
            if accounts.is_empty() || force {
                // Remove user
                self.user_accounts.remove(&account_id);

                // Remove all associated accounts
                for account in accounts.iter() {
                    self.accounts.remove(&account);
                }

                // Remove storage balance
                self.storage_balances.remove(&account_id);

                // Refund entire deposit
                Promise::new(account_id.clone()).transfer(storage_balance.total.0 + 1);
                true
            } else {
                panic!("Cannot unregister the user with associated accounts");
            }
        } else {
            log!("The user {} is not registered", account_id);
            false
        }
    }

    fn storage_balance_bounds(&self) -> StorageBalanceBounds {
        let required_storage_balance =
            Balance::from(self.metadata.user_storage_usage) * env::storage_byte_cost();
        StorageBalanceBounds {
            min: required_storage_balance.into(),
            max: None,
        }
    }

    fn storage_balance_of(&self, account_id: AccountId) -> Option<StorageBalance> {
        self.storage_balances.get(&account_id)
    }
}

impl Contract {
    pub fn internal_create_account(&mut self, account_id: AccountId, account_name: String) {
        // Retrieve storage balance of the account owner
        let mut storage_balance = self
            .storage_balances
            .get(&account_id)
            .unwrap_or_else(|| panic!("The user {} is not registered", account_id));
        let amount = Balance::from(self.metadata.account_storage_usage) * env::storage_byte_cost();

        require!(
            storage_balance.available.0 >= amount,
            "Insufficient deposit to create an account"
        );

        // Subtract the account storage cost from the available balance
        storage_balance.available = Balance::from(storage_balance.available)
            .checked_sub(amount)
            .unwrap_or_else(|| panic!("Balance overflow"))
            .into();
        self.storage_balances.insert(&account_id, &storage_balance);

        // Create new empty account
        self.accounts.insert(
            &account_name,
            &(Account {
                owner_id: account_id.clone(),
                balance: 0,
            }),
        );

        // Add account to user's list of accounts
        let mut user_account = self.user_accounts.get(&account_id).unwrap();
        user_account.push(account_name.clone());
        self.user_accounts.insert(&account_id, &user_account);
    }
}

#[cfg(test)]
mod test {
    use crate::ACCOUNT_NAME_MAX_LENGTH;

    use super::*;
    use near_sdk::{
        test_utils::{accounts, VMContextBuilder},
        testing_env,
    };

    fn get_context(predecessor_account_id: AccountId) -> VMContextBuilder {
        let mut builder = VMContextBuilder::new();
        builder
            .current_account_id(accounts(0))
            .signer_account_id(predecessor_account_id.clone())
            .predecessor_account_id(predecessor_account_id);
        builder
    }

    #[test]
    fn test_storage_deposit() {
        let mut context = get_context(accounts(1));
        testing_env!(context.build());
        let mut contract = Contract::new(accounts(1), accounts(2), 1, 100);

        testing_env!(context
            .storage_usage(env::storage_usage())
            .attached_deposit(
                Balance::from(contract.metadata.user_storage_usage) * env::storage_byte_cost()
            )
            .predecessor_account_id(accounts(1))
            .build());
        contract.storage_deposit(None, Some(true));

        testing_env!(context
            .storage_usage(env::storage_usage())
            .attached_deposit(1)
            .predecessor_account_id(accounts(1))
            .build());
        contract.storage_deposit(None, None);

        let storage_balance = contract.storage_balance_of(accounts(1)).unwrap();
        assert_eq!(
            Balance::from(storage_balance.total),
            Balance::from(contract.metadata.user_storage_usage) * env::storage_byte_cost() + 1
        );
        assert_eq!(Balance::from(storage_balance.available), 1)
    }

    #[test]
    #[should_panic(expected = "Insufficient deposit to register a user")]
    fn test_storage_deposit_insufficient_deposit() {
        let mut context = get_context(accounts(1));
        testing_env!(context.build());
        let mut contract = Contract::new(accounts(1), accounts(2), 1, 100);

        testing_env!(context
            .storage_usage(env::storage_usage())
            .attached_deposit(env::storage_byte_cost())
            .predecessor_account_id(accounts(1))
            .build());
        contract.storage_deposit(None, Some(true));
    }

    #[test]
    fn test_storage_withdraw() {
        let mut context = get_context(accounts(1));
        testing_env!(context.build());
        let mut contract = Contract::new(accounts(1), accounts(2), 1, 100);

        testing_env!(context
            .storage_usage(env::storage_usage())
            .attached_deposit(
                Balance::from(contract.metadata.user_storage_usage) * env::storage_byte_cost()
            )
            .predecessor_account_id(accounts(1))
            .build());
        contract.storage_deposit(None, Some(true));

        testing_env!(context
            .storage_usage(env::storage_usage())
            .attached_deposit(2)
            .predecessor_account_id(accounts(1))
            .build());
        contract.storage_deposit(None, None);

        testing_env!(context
            .storage_usage(env::storage_usage())
            .attached_deposit(1)
            .predecessor_account_id(accounts(1))
            .build());
        contract.storage_withdraw(Some(1.into()));
        contract.storage_withdraw(None);
    }

    #[test]
    #[should_panic(expected = "Balance overflow")]
    fn test_storage_withdraw_not_enough_available_token() {
        let mut context = get_context(accounts(1));
        testing_env!(context.build());
        let mut contract = Contract::new(accounts(1), accounts(2), 1, 100);

        testing_env!(context
            .storage_usage(env::storage_usage())
            .attached_deposit(
                Balance::from(contract.metadata.user_storage_usage) * env::storage_byte_cost()
            )
            .predecessor_account_id(accounts(1))
            .build());
        contract.storage_deposit(None, Some(true));

        testing_env!(context
            .storage_usage(env::storage_usage())
            .attached_deposit(1)
            .predecessor_account_id(accounts(1))
            .build());
        contract.storage_deposit(None, None);

        testing_env!(context
            .storage_usage(env::storage_usage())
            .attached_deposit(1)
            .predecessor_account_id(accounts(1))
            .build());
        contract.storage_withdraw(Some(2.into()));
    }

    #[test]
    fn test_storage_unregister() {
        let mut context = get_context(accounts(1));
        testing_env!(context.build());
        let mut contract = Contract::new(accounts(1), accounts(2), 1, 100);
        let tmp_account_id = AccountId::new_unchecked("a".repeat(64));

        testing_env!(context
            .storage_usage(env::storage_usage())
            .attached_deposit(
                Balance::from(contract.metadata.user_storage_usage) * env::storage_byte_cost()
            )
            .predecessor_account_id(tmp_account_id.clone())
            .build());
        contract.storage_deposit(None, None);

        let storage_balance = contract.storage_balance_of(tmp_account_id.clone()).unwrap();
        let initial_storage_usage = env::storage_usage();

        testing_env!(context
            .storage_usage(env::storage_usage())
            .attached_deposit(1)
            .predecessor_account_id(tmp_account_id.clone())
            .build());
        contract.storage_unregister(None);

        assert_eq!(contract.user_accounts.get(&tmp_account_id).is_none(), true);
        assert_eq!(
            contract
                .storage_balance_of(tmp_account_id.clone())
                .is_none(),
            true
        );
        assert_eq!(
            storage_balance.total.0,
            Balance::from(initial_storage_usage - env::storage_usage()) * env::storage_byte_cost()
        );
    }

    #[test]
    fn test_storage_unregister_user_with_accounts() {
        let mut context = get_context(accounts(1));
        testing_env!(context.build());
        let mut contract = Contract::new(accounts(1), accounts(2), 1, 100);
        let tmp_account_id = AccountId::new_unchecked("a".repeat(64));
        let account_name = "b".repeat(ACCOUNT_NAME_MAX_LENGTH);

        testing_env!(context
            .storage_usage(env::storage_usage())
            .attached_deposit(
                Balance::from(
                    contract.metadata.user_storage_usage + contract.metadata.account_storage_usage
                ) * env::storage_byte_cost()
            )
            .predecessor_account_id(tmp_account_id.clone())
            .build());
        contract.storage_deposit(None, None);
        contract.internal_create_account(tmp_account_id.clone(), account_name.clone());

        let storage_balance = contract.storage_balance_of(tmp_account_id.clone()).unwrap();
        let initial_storage_usage = env::storage_usage();

        testing_env!(context
            .storage_usage(env::storage_usage())
            .attached_deposit(1)
            .predecessor_account_id(tmp_account_id.clone())
            .build());
        contract.storage_unregister(Some(true));

        assert_eq!(contract.accounts.get(&account_name).is_none(), true);
        assert_eq!(
            contract
                .user_accounts
                .get(&tmp_account_id.clone())
                .is_none(),
            true
        );
        assert_eq!(
            contract
                .storage_balance_of(tmp_account_id.clone())
                .is_none(),
            true
        );
        assert_eq!(
            storage_balance.total.0,
            Balance::from(initial_storage_usage - env::storage_usage()) * env::storage_byte_cost()
        );
    }

    #[test]
    fn test_storage_balance_bounds() {
        let mut context = get_context(accounts(1));
        testing_env!(context.build());
        let contract = Contract::new(accounts(1), accounts(2), 1, 100);

        testing_env!(context.is_view(true).build());
        let storage_balance_bounds = contract.storage_balance_bounds();
        assert_eq!(
            storage_balance_bounds.min.0,
            Balance::from(contract.metadata.user_storage_usage) * env::storage_byte_cost()
        );
        assert_eq!(storage_balance_bounds.max.is_none(), true);
    }
}
