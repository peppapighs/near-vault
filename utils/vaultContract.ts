import { ConnectedWalletAccount, Contract } from 'near-api-js'

interface VaultChangeContract extends Contract {
  create_account: (
    args: {
      account_name: string
    },
    gas?: string
  ) => Promise<null>
  withdraw: (
    args: {
      account_name: string
      amount: string
    },
    gas?: string,
    amount?: string
  ) => Promise<null>
  transfer: (
    args: {
      sender_account_name: string
      receiver_account_name: string
      amount: string
    },
    gas?: string
  ) => Promise<null>
  storage_deposit: (
    args: {
      account_id?: string
      registration_only?: boolean
    },
    gas?: string,
    amount?: string
  ) => Promise<{ total: string; available: string }>
  storage_withdraw: (
    args: { amount?: string },
    gas?: string,
    amount?: string
  ) => Promise<{ total: string; available: string }>
  storage_unregister: (
    args: { force?: boolean },
    gas?: string,
    amount?: string
  ) => Promise<boolean>
}

interface VaultViewContract extends Contract {
  get_metadata: () => Promise<{
    owner_id: string
    token_id: string
    transfer_fee_numerator: string
    transfer_fee_denominator: string
    user_storage_usage: string
    account_storage_usage: string
  }>
  get_accounts: (args: { account_id: string }) => Promise<string[] | null>
  get_balance: (args: { account_name: string }) => Promise<string>
  storage_balance_bounds: () => Promise<{ min: string; max: string | null }>
  storage_balance_of: (args: {
    account_id: string
  }) => Promise<{ total: string; available: string } | null>
}

interface VaultContract extends VaultChangeContract, VaultViewContract {}

const vaultContract = (wallet: ConnectedWalletAccount) => {
  return new Contract(wallet, `${process.env.NEXT_PUBLIC_VAULT_CONTRACT}`, {
    changeMethods: [
      'create_account',
      'withdraw',
      'transfer',
      'storage_deposit',
      'storage_withdraw',
      'storage_unregister',
    ],
    viewMethods: [
      'get_metadata',
      'get_accounts',
      'get_balance',
      'storage_balance_bounds',
      'storage_balance_of',
    ],
  }) as VaultContract
}

export default vaultContract
