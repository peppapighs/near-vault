import React, { useContext, useEffect, useReducer } from 'react'

import { connect, keyStores, WalletConnection } from 'near-api-js'

import { nearConfig } from 'constants/near'
import tokenContract, { TokenContractMetadata } from 'utils/tokenContract'
import vaultContract, {
  StorageBalance,
  VaultContractMetadata,
} from 'utils/vaultContract'

export interface Account {
  accountName: string
  balance: string
}

interface UserData {
  registered: boolean
  tokenBalance: string
  storageBalance: StorageBalance
  accounts: Account[]
}

export interface AppState {
  loading: boolean
  wallet: WalletConnection
  vaultContractMetadata: VaultContractMetadata
  tokenContractMetadata: TokenContractMetadata
  user: UserData
}

const initialState: AppState = {
  loading: true,
  wallet: {} as WalletConnection,
  vaultContractMetadata: {
    owner_id: '',
    token_id: '',
    transfer_fee_numerator: '0',
    transfer_fee_denominator: '0',
    user_storage_usage: '0',
    account_storage_usage: '0',
  },
  tokenContractMetadata: {
    spec: '',
    name: '',
    symbol: '',
    icon: null,
    reference: null,
    reference_hash: null,
    decimals: 0,
  },
  user: {
    registered: false,
    tokenBalance: '0',
    storageBalance: {
      total: '0',
      available: '0',
    },
    accounts: [],
  },
}

type AppAction =
  | { type: 'LOADING_START' }
  | { type: 'LOADING_END' }
  | { type: 'SET_APP_STATE'; payload: AppState }

const reducer = (state: AppState, action: AppAction) => {
  switch (action.type) {
    case 'LOADING_START':
      return { ...state, loading: true }
    case 'LOADING_END':
      return { ...state, loading: false }
    case 'SET_APP_STATE':
      return { ...state, ...action.payload }
  }
}

type AppContext = AppState & {
  dispatch: React.Dispatch<AppAction>
  fetchAppState: () => void
}

const AppStateContext = React.createContext<AppContext>({
  ...initialState,
  dispatch: () => {},
  fetchAppState: () => {},
})

export const useApp = () => useContext(AppStateContext)

interface Props {
  children: React.ReactNode
}

export const AppProvider = ({ children }: Props) => {
  const [state, dispatch] = useReducer(reducer, initialState)

  const fetchAppState = async () => {
    const config = {
      ...nearConfig,
      keyStore: new keyStores.BrowserLocalStorageKeyStore(),
    }

    const wallet = new WalletConnection(await connect(config), 'near-vault')

    if (wallet.isSignedIn()) {
      const accountNames =
        (await vaultContract(wallet.account()).get_accounts({
          account_id: wallet.getAccountId(),
        })) || []

      const user: UserData = {
        registered:
          (await vaultContract(wallet.account()).storage_balance_of({
            account_id: wallet.getAccountId(),
          })) !== null,
        tokenBalance: await tokenContract(wallet.account()).ft_balance_of({
          account_id: wallet.getAccountId(),
        }),
        storageBalance:
          (await vaultContract(wallet.account()).storage_balance_of({
            account_id: wallet.getAccountId(),
          })) || initialState.user.storageBalance,
        accounts: await Promise.all(
          accountNames.map(async (accountName) => {
            const balance =
              (await vaultContract(wallet.account()).get_balance({
                account_name: accountName,
              })) || '0'
            return { accountName, balance }
          })
        ),
      }

      const vaultContractMetadata = await vaultContract(
        wallet.account()
      ).get_metadata()
      const tokenContractMetadata = await tokenContract(
        wallet.account()
      ).ft_metadata()

      dispatch({
        type: 'SET_APP_STATE',
        payload: {
          loading: false,
          wallet,
          vaultContractMetadata,
          tokenContractMetadata,
          user,
        },
      })
    } else {
      dispatch({
        type: 'SET_APP_STATE',
        payload: {
          ...initialState,
          loading: false,
          wallet,
        },
      })
    }
  }

  useEffect(() => {
    fetchAppState()
  }, [])

  return (
    <AppStateContext.Provider value={{ ...state, dispatch, fetchAppState }}>
      {children}
    </AppStateContext.Provider>
  )
}
