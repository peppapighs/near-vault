#[cfg(test)]
pub mod tests {
    use crate::msg::{DepositPayload, TransferMessage};
    use crate::{Account, Contract, ContractMetadata};
    use near_contract_standards::fungible_token::receiver::FungibleTokenReceiver;
    use near_contract_standards::storage_management::StorageManagement;
    use near_sdk::borsh::BorshSerialize;
    use near_sdk::test_utils::{accounts, VMContextBuilder};
    use near_sdk::{bs58, env, testing_env, AccountId, Balance};

    fn get_context(predecessor_account_id: AccountId) -> VMContextBuilder {
        let mut builder = VMContextBuilder::new();
        builder
            .current_account_id(accounts(0))
            .signer_account_id(predecessor_account_id.clone())
            .predecessor_account_id(predecessor_account_id);
        builder
    }

    fn register_user(contract: &mut Contract, account_id: &AccountId) {
        let mut context = get_context(account_id.clone());
        testing_env!(context
            .storage_usage(env::storage_usage())
            .attached_deposit(
                Balance::from(contract.metadata.user_storage_usage.0) * env::storage_byte_cost()
            )
            .predecessor_account_id(account_id.clone())
            .build());
        contract.storage_deposit(Some(account_id.clone()), Some(true));
    }

    fn create_account(contract: &mut Contract, account_id: &AccountId, account_name: &str) {
        let mut context = get_context(account_id.clone());
        testing_env!(context
            .storage_usage(env::storage_usage())
            .attached_deposit(
                Balance::from(contract.metadata.account_storage_usage.0) * env::storage_byte_cost()
            )
            .predecessor_account_id(account_id.clone())
            .build());
        contract.create_account(account_name.to_owned());
    }

    #[test]
    fn test_new() {
        let context = get_context(accounts(1));
        testing_env!(context.build());
        Contract::new(accounts(1), accounts(2), 1.into(), 100.into());
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
        let mut context = get_context(accounts(1));
        testing_env!(context.build());
        let mut contract = Contract::new(accounts(1), accounts(2), 1.into(), 100.into());

        register_user(&mut contract, &accounts(1));
        create_account(&mut contract, &accounts(1), "account");

        testing_env!(context.is_view(true).build());
        let account = contract.accounts.get(&"account".to_owned()).unwrap();
        assert_eq!(account.owner_id, accounts(1));
        assert_eq!(account.balance, 0);
        assert_eq!(
            contract.get_balance("account".to_owned()).unwrap(),
            0.into()
        );
        assert_eq!(
            contract.get_accounts(accounts(1).clone()).unwrap(),
            vec!["account"]
        );
        assert_eq!(
            contract.get_metadata(),
            ContractMetadata {
                owner_id: accounts(1),
                token_id: accounts(2),
                transfer_fee_numerator: 1.into(),
                transfer_fee_denominator: 100.into(),
                user_storage_usage: contract.metadata.user_storage_usage,
                account_storage_usage: contract.metadata.account_storage_usage
            }
        );
    }

    #[test]
    #[should_panic(expected = "Account already exists")]
    fn test_create_duplicate_account() {
        let context = get_context(accounts(1));
        testing_env!(context.build());
        let mut contract = Contract::new(accounts(1), accounts(2), 1.into(), 100.into());

        register_user(&mut contract, &accounts(1));
        create_account(&mut contract, &accounts(1), "account");
        create_account(&mut contract, &accounts(1), "account");
    }

    #[test]
    #[should_panic(expected = "Insufficient deposit to create an account")]
    fn test_create_account_insufficient_deposit() {
        let context = get_context(accounts(1));
        testing_env!(context.build());
        let mut contract = Contract::new(accounts(1), accounts(2), 1.into(), 100.into());

        register_user(&mut contract, &accounts(1));
        contract.create_account("account".to_owned());
    }

    #[test]
    #[should_panic(expected = "The contract is not initialized")]
    fn test_default_account() {
        Account::default();
    }

    #[test]
    fn test_get_balance_non_existent_account() {
        let mut context = get_context(accounts(1));
        testing_env!(context.build());
        let contract = Contract::new(accounts(1), accounts(2), 1.into(), 100.into());

        testing_env!(context.is_view(true).build());
        assert_eq!(contract.get_balance("account".into()), None);
    }

    #[test]
    fn test_deposit() {
        let context = get_context(accounts(1));
        testing_env!(context.build());
        let mut contract = Contract::new(accounts(1), accounts(2), 1.into(), 100.into());

        register_user(&mut contract, &accounts(1));
        create_account(&mut contract, &accounts(1), "account");

        let context = get_context(accounts(2));
        testing_env!(context.build());
        let msg = bs58::encode(
            (TransferMessage {
                action: "deposit".to_owned(),
                payload: (DepositPayload {
                    account_name: "account".to_owned(),
                })
                .try_to_vec()
                .unwrap(),
            })
            .try_to_vec()
            .unwrap(),
        )
        .into_string();
        contract.ft_on_transfer(accounts(1), 1.into(), msg.clone());
        assert_eq!(
            contract.get_balance("account".to_owned()).unwrap(),
            1.into()
        );
    }

    #[test]
    #[should_panic(expected = "Unsupported token type")]
    fn test_ft_on_transfer_unsupported_token() {
        let context = get_context(accounts(1));
        testing_env!(context.build());
        let mut contract = Contract::new(accounts(1), accounts(2), 1.into(), 100.into());

        contract.ft_on_transfer(accounts(1), 1.into(), "{}".to_owned());
    }

    #[test]
    #[should_panic(expected = "Invalid transfer message format")]
    fn test_ft_on_transfer_invalid_transfer_message_format() {
        let context = get_context(accounts(1));
        testing_env!(context.build());
        let mut contract = Contract::new(accounts(1), accounts(2), 1.into(), 100.into());

        let context = get_context(accounts(2));
        testing_env!(context.build());
        contract.ft_on_transfer(accounts(1), 1.into(), "{}".to_owned());
    }

    #[test]
    #[should_panic(expected = "Unsupported action")]
    fn test_ft_on_transfer_unsupported_action() {
        let context = get_context(accounts(1));
        testing_env!(context.build());
        let mut contract = Contract::new(accounts(1), accounts(2), 1.into(), 100.into());

        let context = get_context(accounts(2));
        testing_env!(context.build());
        let msg = bs58::encode(
            (TransferMessage {
                action: String::new(),
                payload: vec![],
            })
            .try_to_vec()
            .unwrap(),
        )
        .into_string();
        contract.ft_on_transfer(accounts(1), 1.into(), msg.clone());
    }

    #[test]
    #[should_panic(expected = "Invalid deposit payload format")]
    fn test_ft_on_transfer_invalid_deposit_payload_format() {
        let context = get_context(accounts(1));
        testing_env!(context.build());
        let mut contract = Contract::new(accounts(1), accounts(2), 1.into(), 100.into());

        let context = get_context(accounts(2));
        testing_env!(context.build());
        let msg = bs58::encode(
            (TransferMessage {
                action: "deposit".to_owned(),
                payload: vec![],
            })
            .try_to_vec()
            .unwrap(),
        )
        .into_string();
        contract.ft_on_transfer(accounts(1), 1.into(), msg.clone());
    }

    #[test]
    fn test_withdraw() {
        let context = get_context(accounts(1));
        testing_env!(context.build());
        let mut contract = Contract::new(accounts(1), accounts(2), 1.into(), 100.into());

        register_user(&mut contract, &accounts(1));
        create_account(&mut contract, &accounts(1), "account");

        let context = get_context(accounts(2));
        testing_env!(context.build());
        let msg = bs58::encode(
            (TransferMessage {
                action: "deposit".to_owned(),
                payload: (DepositPayload {
                    account_name: "account".to_owned(),
                })
                .try_to_vec()
                .unwrap(),
            })
            .try_to_vec()
            .unwrap(),
        )
        .into_string();
        contract.ft_on_transfer(accounts(1), 1.into(), msg.clone());

        let mut context = get_context(accounts(1));
        testing_env!(context.attached_deposit(1).build());
        contract.withdraw("account".into(), 1.into());

        assert_eq!(
            contract.get_balance("account".to_owned()).unwrap(),
            0.into()
        );
    }

    #[test]
    #[should_panic(expected = "Unauthorized access to account")]
    fn test_withdraw_unauthorized_access() {
        let context = get_context(accounts(1));
        testing_env!(context.build());
        let mut contract = Contract::new(accounts(1), accounts(2), 1.into(), 100.into());

        register_user(&mut contract, &accounts(1));
        register_user(&mut contract, &accounts(3));
        create_account(&mut contract, &accounts(1), "account");

        let context = get_context(accounts(2));
        testing_env!(context.build());
        let msg = bs58::encode(
            (TransferMessage {
                action: "deposit".to_owned(),
                payload: (DepositPayload {
                    account_name: "account".to_owned(),
                })
                .try_to_vec()
                .unwrap(),
            })
            .try_to_vec()
            .unwrap(),
        )
        .into_string();
        contract.ft_on_transfer(accounts(1), 1.into(), msg.clone());

        let mut context = get_context(accounts(3));
        testing_env!(context.attached_deposit(1).build());
        contract.withdraw("account".into(), 1.into());
    }

    #[test]
    #[should_panic(expected = "Balance overflow")]
    fn test_withdraw_not_enough_token() {
        let context = get_context(accounts(1));
        testing_env!(context.build());
        let mut contract = Contract::new(accounts(1), accounts(2), 1.into(), 100.into());

        register_user(&mut contract, &accounts(1));
        create_account(&mut contract, &accounts(1), "account");

        let context = get_context(accounts(2));
        testing_env!(context.build());
        let msg = bs58::encode(
            (TransferMessage {
                action: "deposit".to_owned(),
                payload: (DepositPayload {
                    account_name: "account".to_owned(),
                })
                .try_to_vec()
                .unwrap(),
            })
            .try_to_vec()
            .unwrap(),
        )
        .into_string();
        contract.ft_on_transfer(accounts(1), 1.into(), msg.clone());

        let mut context = get_context(accounts(1));
        testing_env!(context.attached_deposit(1).build());
        contract.withdraw("account".into(), 2.into());
    }

    #[test]
    fn test_transfer_same_owner() {
        let context = get_context(accounts(1));
        testing_env!(context.build());
        let mut contract = Contract::new(accounts(1), accounts(2), 1.into(), 100.into());

        register_user(&mut contract, &accounts(1));
        create_account(&mut contract, &accounts(1), "account_1");
        create_account(&mut contract, &accounts(1), "account_2");

        let context = get_context(accounts(2));
        testing_env!(context.build());
        let msg = bs58::encode(
            (TransferMessage {
                action: "deposit".to_owned(),
                payload: (DepositPayload {
                    account_name: "account_1".to_owned(),
                })
                .try_to_vec()
                .unwrap(),
            })
            .try_to_vec()
            .unwrap(),
        )
        .into_string();
        contract.ft_on_transfer(accounts(1), 1.into(), msg.clone());
        let msg = bs58::encode(
            (TransferMessage {
                action: "deposit".to_owned(),
                payload: (DepositPayload {
                    account_name: "account_2".to_owned(),
                })
                .try_to_vec()
                .unwrap(),
            })
            .try_to_vec()
            .unwrap(),
        )
        .into_string();
        contract.ft_on_transfer(accounts(1), 1.into(), msg.clone());

        let context = get_context(accounts(1));
        testing_env!(context.build());
        contract.transfer("account_1".into(), "account_2".into(), 1.into());
        assert_eq!(
            contract.get_balance("account_1".to_owned()).unwrap(),
            0.into()
        );
        assert_eq!(
            contract.get_balance("account_2".to_owned()).unwrap(),
            2.into()
        );
    }

    #[test]
    fn test_transfer_different_owner() {
        let context = get_context(accounts(1));
        testing_env!(context.build());
        let mut contract = Contract::new(accounts(1), accounts(2), 1.into(), 100.into());

        register_user(&mut contract, &accounts(1));
        register_user(&mut contract, &accounts(3));
        create_account(&mut contract, &accounts(1), "account_1");
        create_account(&mut contract, &accounts(3), "account_2");

        let context = get_context(accounts(2));
        testing_env!(context.build());
        let msg = bs58::encode(
            (TransferMessage {
                action: "deposit".to_owned(),
                payload: (DepositPayload {
                    account_name: "account_1".to_owned(),
                })
                .try_to_vec()
                .unwrap(),
            })
            .try_to_vec()
            .unwrap(),
        )
        .into_string();
        contract.ft_on_transfer(accounts(1), 100.into(), msg.clone());
        let msg = bs58::encode(
            (TransferMessage {
                action: "deposit".to_owned(),
                payload: (DepositPayload {
                    account_name: "account_2".to_owned(),
                })
                .try_to_vec()
                .unwrap(),
            })
            .try_to_vec()
            .unwrap(),
        )
        .into_string();
        contract.ft_on_transfer(accounts(3), 100.into(), msg.clone());

        let context = get_context(accounts(1));
        testing_env!(context.build());
        contract.transfer("account_1".into(), "account_2".into(), 100.into());
        assert_eq!(
            contract.get_balance("account_1".to_owned()).unwrap(),
            0.into()
        );
        assert_eq!(
            contract.get_balance("account_2".to_owned()).unwrap(),
            199.into()
        );
        assert_eq!(contract.total_transfer_fee, 1);
    }

    #[test]
    #[should_panic(expected = "Sender account does not exist")]
    fn test_transfer_non_existent_sender_account() {
        let context = get_context(accounts(1));
        testing_env!(context.build());
        let mut contract = Contract::new(accounts(1), accounts(2), 1.into(), 100.into());

        register_user(&mut contract, &accounts(1));

        contract.transfer("account_1".into(), "account_2".into(), 1.into());
    }

    #[test]
    #[should_panic(expected = "Receiver account does not exist")]
    fn test_transfer_non_existent_receiver_account() {
        let context = get_context(accounts(1));
        testing_env!(context.build());
        let mut contract = Contract::new(accounts(1), accounts(2), 1.into(), 100.into());

        register_user(&mut contract, &accounts(1));
        create_account(&mut contract, &accounts(1), "account_1");

        contract.transfer("account_1".into(), "account_2".into(), 1.into());
    }

    #[test]
    #[should_panic(expected = "Balance overflow")]
    fn test_transfer_not_enough_token() {
        let context = get_context(accounts(1));
        testing_env!(context.build());
        let mut contract = Contract::new(accounts(1), accounts(2), 1.into(), 100.into());

        register_user(&mut contract, &accounts(1));
        create_account(&mut contract, &accounts(1), "account_1");
        create_account(&mut contract, &accounts(1), "account_2");

        let context = get_context(accounts(2));
        testing_env!(context.build());
        let msg = bs58::encode(
            (TransferMessage {
                action: "deposit".to_owned(),
                payload: (DepositPayload {
                    account_name: "account_1".to_owned(),
                })
                .try_to_vec()
                .unwrap(),
            })
            .try_to_vec()
            .unwrap(),
        )
        .into_string();
        contract.ft_on_transfer(accounts(1), 1.into(), msg.clone());
        let msg = bs58::encode(
            (TransferMessage {
                action: "deposit".to_owned(),
                payload: (DepositPayload {
                    account_name: "account_2".to_owned(),
                })
                .try_to_vec()
                .unwrap(),
            })
            .try_to_vec()
            .unwrap(),
        )
        .into_string();
        contract.ft_on_transfer(accounts(1), 1.into(), msg.clone());

        let context = get_context(accounts(1));
        testing_env!(context.build());
        contract.transfer("account_1".into(), "account_2".into(), 2.into());
    }

    #[test]
    fn test_withdraw_transfer_fee() {
        let context = get_context(accounts(1));
        testing_env!(context.build());
        let mut contract = Contract::new(accounts(1), accounts(2), 1.into(), 100.into());

        register_user(&mut contract, &accounts(1));
        register_user(&mut contract, &accounts(3));
        create_account(&mut contract, &accounts(1), "account_1");
        create_account(&mut contract, &accounts(3), "account_2");

        let context = get_context(accounts(2));
        testing_env!(context.build());
        let msg = bs58::encode(
            (TransferMessage {
                action: "deposit".to_owned(),
                payload: (DepositPayload {
                    account_name: "account_1".to_owned(),
                })
                .try_to_vec()
                .unwrap(),
            })
            .try_to_vec()
            .unwrap(),
        )
        .into_string();
        contract.ft_on_transfer(accounts(1), 100.into(), msg.clone());
        let msg = bs58::encode(
            (TransferMessage {
                action: "deposit".to_owned(),
                payload: (DepositPayload {
                    account_name: "account_2".to_owned(),
                })
                .try_to_vec()
                .unwrap(),
            })
            .try_to_vec()
            .unwrap(),
        )
        .into_string();
        contract.ft_on_transfer(accounts(3), 100.into(), msg.clone());

        let mut context = get_context(accounts(1));
        testing_env!(context.build());
        contract.transfer("account_1".into(), "account_2".into(), 100.into());

        testing_env!(context.attached_deposit(1).build());
        contract.withdraw_transfer_fee(1.into());
        assert_eq!(contract.total_transfer_fee, 0);
    }

    #[test]
    #[should_panic(expected = "Unauthorized access")]
    fn test_withdraw_transfer_fee_unauthorized_access() {
        let context = get_context(accounts(1));
        testing_env!(context.build());
        let mut contract = Contract::new(accounts(1), accounts(2), 1.into(), 100.into());

        let mut context = get_context(accounts(2));
        testing_env!(context.attached_deposit(1).build());
        contract.withdraw_transfer_fee(1.into());
    }

    #[test]
    #[should_panic(expected = "Balance overflow")]
    fn test_withdraw_transfer_fee_not_enough_token() {
        let mut context = get_context(accounts(1));
        testing_env!(context.build());
        let mut contract = Contract::new(accounts(1), accounts(2), 1.into(), 100.into());

        testing_env!(context.attached_deposit(1).build());
        contract.withdraw_transfer_fee(1.into());
    }
}
