import { ConnectedWalletAccount, Contract } from 'near-api-js'

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
}

interface TokenContract extends TokenChangeContract, TokenViewContract {}

const tokenContract = (wallet: ConnectedWalletAccount) => {
  return new Contract(wallet, `${process.env.NEXT_PUBLIC_TOKEN_CONTRACT}`, {
    changeMethods: ['ft_transfer_call'],
    viewMethods: ['ft_balance_of'],
  }) as TokenContract
}

export default tokenContract
