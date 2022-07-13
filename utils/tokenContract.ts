import { ConnectedWalletAccount, Contract } from 'near-api-js'

export interface TokenContractMetadata {
  spec: string
  name: string
  symbol: string
  icon: string | null
  reference: string | null
  reference_hash: string | null
  decimals: number
}

interface TokenChangeContract extends Contract {
  ft_transfer_call: (
    args: {
      receiver_id: string
      amount: string
      memo: string | null
      msg: string
    },
    gas?: string,
    amount?: string
  ) => Promise<null>
}

interface TokenViewContract extends Contract {
  ft_balance_of: (args: { account_id: string }) => Promise<string>
  ft_metadata: () => Promise<TokenContractMetadata>
}

interface TokenContract extends TokenChangeContract, TokenViewContract {}

const tokenContract = (wallet: ConnectedWalletAccount) => {
  return new Contract(wallet, `${process.env.NEXT_PUBLIC_TOKEN_CONTRACT}`, {
    changeMethods: ['ft_transfer_call'],
    viewMethods: ['ft_balance_of', 'ft_metadata'],
  }) as TokenContract
}

export default tokenContract
