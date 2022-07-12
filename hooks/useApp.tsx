import React, { useContext, useEffect, useReducer } from 'react'

import {
  connect,
  ConnectConfig,
  keyStores,
  WalletConnection,
} from 'near-api-js'

import tokenContract from 'utils/tokenContract'
import vaultContract from 'utils/vaultContract'

interface Account {
  accountName: string
  balance: string
}

interface UserData {
  tokenBalance: string
  storageBalance: {
    total: string
    available: string
  }
  accounts: Account[]
}

export interface AppState {
  loading: boolean
  wallet: WalletConnection | undefined
  user: UserData
}

const initialUser: UserData = {
  tokenBalance: '0',
  storageBalance: {
    total: '0',
    available: '0',
  },
  accounts: [],
}

const initialState: AppState = {
  loading: true,
  wallet: undefined,
  user: initialUser,
}

type AppAction =
  | { type: 'LOADING_START' }
  | { type: 'LOADING_END' }
  | { type: 'SET_APP_STATE'; payload: AppState }
  | { type: 'SET_WALLET'; payload: WalletConnection }
  | { type: 'SET_USER'; payload: UserData }
  | {
      type: 'SET_USER_BALANCE'
      payload: {
        tokenBalance: string
        storageBalanceTotal: string
        storageBalanceAvailable: string
      }
    }
  | { type: 'SET_USER_ACCOUNT'; payload: Account[] }

const reducer = (state: AppState, action: AppAction) => {
  switch (action.type) {
    case 'LOADING_START':
      return { ...state, loading: true }
    case 'LOADING_END':
      return { ...state, loading: false }
    case 'SET_APP_STATE':
      return { ...state, ...action.payload }
    case 'SET_WALLET':
      return { ...state, wallet: action.payload }
    case 'SET_USER':
      return { ...state, user: action.payload }
    case 'SET_USER_BALANCE':
      return { ...state, user: { ...state.user, ...action.payload } }
    case 'SET_USER_ACCOUNT':
      return { ...state, user: { ...state.user, accounts: action.payload } }
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

export const AppProvider = (props: Props) => {
  const [state, dispatch] = useReducer(reducer, initialState)

  const fetchAppState = async () => {
    const config: ConnectConfig = {
      headers: {},
      networkId: 'testnet',
      keyStore: new keyStores.BrowserLocalStorageKeyStore(),
      nodeUrl: 'https://rpc.testnet.near.org',
      walletUrl: 'https://wallet.testnet.near.org',
      helperUrl: 'https://helper.testnet.near.org',
    }

    const wallet = new WalletConnection(await connect(config), 'near-vault')

    if (wallet.isSignedIn()) {
      const accountNames =
        (await vaultContract(wallet.account()).get_accounts({
          account_id: wallet.getAccountId(),
        })) || []

      const user: UserData = {
        tokenBalance: await tokenContract(wallet.account()).ft_balance_of({
          account_id: wallet.getAccountId(),
        }),
        storageBalance:
          (await vaultContract(wallet.account()).storage_balance_of({
            account_id: wallet.getAccountId(),
          })) || initialUser.storageBalance,
        accounts: await Promise.all(
          accountNames.map(async (accountName) => {
            const balance = await vaultContract(wallet.account()).get_balance({
              account_name: accountName,
            })
            return { accountName, balance }
          })
        ),
      }
      dispatch({
        type: 'SET_APP_STATE',
        payload: {
          loading: false,
          wallet,
          user,
        },
      })
    } else {
      dispatch({
        type: 'SET_APP_STATE',
        payload: {
          loading: false,
          wallet,
          user: initialUser,
        },
      })
    }
  }

  useEffect(() => {
    fetchAppState()
  }, [])

  return (
    <AppStateContext.Provider value={{ ...state, dispatch, fetchAppState }}>
      {props.children}
    </AppStateContext.Provider>
  )
}
