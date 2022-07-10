#[cfg(test)]
pub mod tests {
    use crate::{Account, Contract, FeeMessage};
    use near_contract_standards::fungible_token::receiver::FungibleTokenReceiver;
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
        Contract::new(accounts(1), accounts(2), 1, 100);
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
        let mut contract = Contract::new(accounts(1), accounts(2), 1, 100);

        contract.create_account("account".into());
        let account = contract.accounts.get(&"account".to_owned()).unwrap();
        assert_eq!(account.owner_id, accounts(1));
        assert_eq!(account.balance, 0);
        assert_eq!(contract.get_balance("account".to_owned()), 0.into());
        assert_eq!(
            contract.user_accounts.get(&accounts(1)).unwrap(),
            vec!["account"]
        );
        assert_eq!(
            contract.get_fees(),
            FeeMessage {
                transfer_fee_numerator: 1,
                transfer_fee_denominator: 100,
            }
        );
    }

    #[test]
    #[should_panic(expected = "Account already exists")]
    fn test_create_duplicate_account() {
        let context = get_context(accounts(1));
        testing_env!(context.build());
        let mut contract = Contract::new(accounts(1), accounts(2), 1, 100);

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
        let contract = Contract::new(accounts(1), accounts(2), 1, 100);

        contract.get_balance("account".into());
    }

    #[test]
    fn test_deposit() {
        let context = get_context(accounts(1));
        testing_env!(context.build());
        let mut contract = Contract::new(accounts(1), accounts(2), 1, 100);

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
        let mut contract = Contract::new(accounts(1), accounts(2), 1, 100);

        contract.ft_on_transfer(accounts(1), 1.into(), "{}".to_owned());
    }

    #[test]
    #[should_panic(expected = "Invalid transfer message format")]
    fn test_ft_on_transfer_invalid_transfer_message_format() {
        let context = get_context(accounts(1));
        testing_env!(context.build());
        let mut contract = Contract::new(accounts(1), accounts(2), 1, 100);

        let context = get_context(accounts(2));
        testing_env!(context.build());
        contract.ft_on_transfer(accounts(1), 1.into(), "{}".to_owned());
    }

    #[test]
    #[should_panic(expected = "Unsupported action")]
    fn test_ft_on_transfer_unsupported_action() {
        let context = get_context(accounts(1));
        testing_env!(context.build());
        let mut contract = Contract::new(accounts(1), accounts(2), 1, 100);

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
        let mut contract = Contract::new(accounts(1), accounts(2), 1, 100);

        let context = get_context(accounts(2));
        testing_env!(context.build());
        contract.ft_on_transfer(
            accounts(1),
            1.into(),
            "{\"action\":\"deposit\",\"payload\":\"\"}".to_owned(),
        );
    }

    #[test]
    fn test_withdraw() {
        let context = get_context(accounts(1));
        testing_env!(context.build());
        let mut contract = Contract::new(accounts(1), accounts(2), 1, 100);

        contract.create_account("account".into());
        let context = get_context(accounts(2));
        testing_env!(context.build());
        contract.ft_on_transfer(
            accounts(1),
            1.into(),
            "{\"action\":\"deposit\",\"payload\":\"{\\\"account_name\\\":\\\"account\\\"}\"}"
                .to_owned(),
        );

        let context = get_context(accounts(1));
        testing_env!(context.build());
        contract.withdraw("account".into(), 1.into());

        assert_eq!(contract.get_balance("account".to_owned()), 0.into());
    }

    #[test]
    #[should_panic(expected = "Unauthorized access to account")]
    fn test_withdraw_unauthorized_access() {
        let context = get_context(accounts(1));
        testing_env!(context.build());
        let mut contract = Contract::new(accounts(1), accounts(2), 1, 100);

        contract.create_account("account".into());
        let context = get_context(accounts(2));
        testing_env!(context.build());
        contract.ft_on_transfer(
            accounts(1),
            1.into(),
            "{\"action\":\"deposit\",\"payload\":\"{\\\"account_name\\\":\\\"account\\\"}\"}"
                .to_owned(),
        );

        let context = get_context(accounts(3));
        testing_env!(context.build());
        contract.withdraw("account".into(), 1.into());
    }

    #[test]
    #[should_panic(expected = "Balance overflow")]
    fn test_withdraw_not_enough_token() {
        let context = get_context(accounts(1));
        testing_env!(context.build());
        let mut contract = Contract::new(accounts(1), accounts(2), 1, 100);

        contract.create_account("account".into());
        let context = get_context(accounts(2));
        testing_env!(context.build());
        contract.ft_on_transfer(
            accounts(1),
            1.into(),
            "{\"action\":\"deposit\",\"payload\":\"{\\\"account_name\\\":\\\"account\\\"}\"}"
                .to_owned(),
        );

        let context = get_context(accounts(1));
        testing_env!(context.build());
        contract.withdraw("account".into(), 2.into());
    }

    #[test]
    fn test_transfer_same_owner() {
        let context = get_context(accounts(1));
        testing_env!(context.build());
        let mut contract = Contract::new(accounts(1), accounts(2), 1, 100);

        contract.create_account("account_1".into());
        contract.create_account("account_2".into());
        let context = get_context(accounts(2));
        testing_env!(context.build());
        contract.ft_on_transfer(
            accounts(1),
            1.into(),
            "{\"action\":\"deposit\",\"payload\":\"{\\\"account_name\\\":\\\"account_1\\\"}\"}"
                .to_owned(),
        );
        contract.ft_on_transfer(
            accounts(1),
            1.into(),
            "{\"action\":\"deposit\",\"payload\":\"{\\\"account_name\\\":\\\"account_2\\\"}\"}"
                .to_owned(),
        );

        let context = get_context(accounts(1));
        testing_env!(context.build());
        contract.transfer("account_1".into(), "account_2".into(), 1.into());
        assert_eq!(contract.get_balance("account_1".to_owned()), 0.into());
        assert_eq!(contract.get_balance("account_2".to_owned()), 2.into());
    }

    #[test]
    fn test_transfer_different_owner() {
        let context = get_context(accounts(1));
        testing_env!(context.build());
        let mut contract = Contract::new(accounts(1), accounts(2), 1, 100);

        contract.create_account("account_1".into());
        let context = get_context(accounts(3));
        testing_env!(context.build());
        contract.create_account("account_2".into());
        let context = get_context(accounts(2));
        testing_env!(context.build());
        contract.ft_on_transfer(
            accounts(1),
            100.into(),
            "{\"action\":\"deposit\",\"payload\":\"{\\\"account_name\\\":\\\"account_1\\\"}\"}"
                .to_owned(),
        );
        contract.ft_on_transfer(
            accounts(3),
            100.into(),
            "{\"action\":\"deposit\",\"payload\":\"{\\\"account_name\\\":\\\"account_2\\\"}\"}"
                .to_owned(),
        );

        let context = get_context(accounts(1));
        testing_env!(context.build());
        contract.transfer("account_1".into(), "account_2".into(), 100.into());
        assert_eq!(contract.get_balance("account_1".to_owned()), 0.into());
        assert_eq!(contract.get_balance("account_2".to_owned()), 199.into());
        assert_eq!(contract.total_transfer_fee, 1);
    }

    #[test]
    #[should_panic(expected = "Sender account does not exist")]
    fn test_transfer_non_existent_sender_account() {
        let context = get_context(accounts(1));
        testing_env!(context.build());
        let mut contract = Contract::new(accounts(1), accounts(2), 1, 100);

        contract.transfer("account_1".into(), "account_2".into(), 1.into());
    }

    #[test]
    #[should_panic(expected = "Receiver account does not exist")]
    fn test_transfer_non_existent_receiver_account() {
        let context = get_context(accounts(1));
        testing_env!(context.build());
        let mut contract = Contract::new(accounts(1), accounts(2), 1, 100);

        contract.create_account("account_1".into());
        contract.transfer("account_1".into(), "account_2".into(), 1.into());
    }

    #[test]
    #[should_panic(expected = "Balance overflow")]
    fn test_transfer_not_enough_token() {
        let context = get_context(accounts(1));
        testing_env!(context.build());
        let mut contract = Contract::new(accounts(1), accounts(2), 1, 100);

        contract.create_account("account_1".into());
        contract.create_account("account_2".into());
        let context = get_context(accounts(2));
        testing_env!(context.build());
        contract.ft_on_transfer(
            accounts(1),
            1.into(),
            "{\"action\":\"deposit\",\"payload\":\"{\\\"account_name\\\":\\\"account_1\\\"}\"}"
                .to_owned(),
        );
        contract.ft_on_transfer(
            accounts(1),
            1.into(),
            "{\"action\":\"deposit\",\"payload\":\"{\\\"account_name\\\":\\\"account_2\\\"}\"}"
                .to_owned(),
        );

        let context = get_context(accounts(1));
        testing_env!(context.build());
        contract.transfer("account_1".into(), "account_2".into(), 2.into());
    }

    #[test]
    fn test_transfer_fee_to_owner() {
        let context = get_context(accounts(1));
        testing_env!(context.build());
        let mut contract = Contract::new(accounts(1), accounts(2), 1, 100);

        contract.create_account("account_1".into());
        let context = get_context(accounts(3));
        testing_env!(context.build());
        contract.create_account("account_2".into());
        let context = get_context(accounts(2));
        testing_env!(context.build());
        contract.ft_on_transfer(
            accounts(1),
            100.into(),
            "{\"action\":\"deposit\",\"payload\":\"{\\\"account_name\\\":\\\"account_1\\\"}\"}"
                .to_owned(),
        );
        contract.ft_on_transfer(
            accounts(3),
            100.into(),
            "{\"action\":\"deposit\",\"payload\":\"{\\\"account_name\\\":\\\"account_2\\\"}\"}"
                .to_owned(),
        );

        let context = get_context(accounts(1));
        testing_env!(context.build());
        contract.transfer("account_1".into(), "account_2".into(), 100.into());
        contract.transfer_fee_to_owner(1.into());
        assert_eq!(contract.total_transfer_fee, 0);
    }

    #[test]
    #[should_panic(expected = "Unauthorized access")]
    fn test_transfer_fee_to_owner_unauthorized_access() {
        let context = get_context(accounts(1));
        testing_env!(context.build());
        let mut contract = Contract::new(accounts(1), accounts(2), 1, 100);

        let context = get_context(accounts(2));
        testing_env!(context.build());
        contract.transfer_fee_to_owner(1.into());
    }

    #[test]
    #[should_panic(expected = "Balance overflow")]
    fn test_transfer_fee_to_owner_not_enough_token() {
        let context = get_context(accounts(1));
        testing_env!(context.build());
        let mut contract = Contract::new(accounts(1), accounts(2), 1, 100);

        contract.transfer_fee_to_owner(1.into());
    }
}
