import React, { useContext, useEffect } from 'react'

import BN from 'bn.js'
import { connect, Contract, keyStores, WalletConnection } from 'near-api-js'

import {
  CONTRACT_CHANGE_METHODS,
  CONTRACT_VIEW_METHODS,
  TOKEN_CHANGE_METHODS,
  TOKEN_VIEW_METHODS,
} from 'constants/contract'

interface ContractMetadata {
  owner_id: string
  token_id: string
  transfer_fee_numerator: BN
  transfer_fee_denominator: BN
}

interface TokenContractMetadata {
  spec: string
  name: string
  symbol: string
  icon: string | null
  reference: string | null
  reference_hash: string | null
  decimals: number
}

interface ContractData {
  contract: any
  tokenContract: any
  contractMetadata: ContractMetadata
  tokenContractMetadata: TokenContractMetadata
}

interface AppState {
  wallet: WalletConnection | null
  contractData: ContractData | null
  loading: boolean
}

type AppAction =
  | { type: 'RECEIVE_WALLET_CONTEXT'; payload: WalletConnection }
  | { type: 'RECEIVE_CONTRACT_CONTEXT'; payload: ContractData }
  | { type: 'LOADING_DONE' }
  | { type: 'LOADING_START' }

type appContext = AppState & {
  dispatch: React.Dispatch<AppAction> | null
}

const initialState: AppState = {
  wallet: null,
  contractData: null,
  loading: true,
}

const reducer = (state: AppState, action: AppAction): AppState => {
  switch (action.type) {
    case 'RECEIVE_WALLET_CONTEXT':
      return Object.assign({}, state, {
        wallet: action.payload,
      })
    case 'RECEIVE_CONTRACT_CONTEXT':
      return Object.assign({}, state, {
        contractData: action.payload,
      })
    case 'LOADING_DONE':
      return Object.assign({}, state, {
        loading: false,
      })
    case 'LOADING_START':
      return Object.assign({}, state, {
        loading: true,
      })
    default:
      return state
  }
}

const AppStateContext = React.createContext<appContext>({
  ...initialState,
  dispatch: null,
})

export const useApp = () => useContext(AppStateContext)

interface Props {
  children: React.ReactNode
}

const AppContext = (props: Props) => {
  const [appState, dispatch] = React.useReducer(reducer, initialState)

  useEffect(() => {
    const config = {
      headers: {},
      networkId: 'testnet',
      keyStore: new keyStores.BrowserLocalStorageKeyStore(),
      nodeUrl: 'https://rpc.testnet.near.org',
      walletUrl: 'https://wallet.testnet.near.org',
      helperUrl: 'https://helper.testnet.near.org',
    }

    const connectToNear = async () => {
      const near = await connect(config)
      const wallet = new WalletConnection(near, 'near-vault')

      dispatch({
        type: 'RECEIVE_WALLET_CONTEXT',
        payload: wallet,
      })
    }

    connectToNear()
  }, [])

  useEffect(() => {
    const fetchContract = async () => {
      const { wallet } = appState

      if (wallet && wallet.isSignedIn()) {
        const contract = new Contract(
          wallet.account(),
          `${process.env.NEXT_PUBLIC_CONTRACT_ACCOUNT_ID}`,
          {
            changeMethods: CONTRACT_CHANGE_METHODS,
            viewMethods: CONTRACT_VIEW_METHODS,
          }
        ) as any
        const contractMetadata: ContractMetadata = await contract.get_metadata()

        const tokenContract = new Contract(
          wallet.account(),
          contractMetadata.token_id,
          {
            changeMethods: TOKEN_CHANGE_METHODS,
            viewMethods: TOKEN_VIEW_METHODS,
          }
        ) as any
        const tokenContractMetadata = await tokenContract.ft_metadata()

        dispatch({
          type: 'RECEIVE_CONTRACT_CONTEXT',
          payload: {
            contract,
            tokenContract,
            contractMetadata,
            tokenContractMetadata,
          },
        })
      }
      dispatch({
        type: 'LOADING_DONE',
      })
    }

    fetchContract()
  }, [appState.wallet])

  return (
    <AppStateContext.Provider value={{ ...appState, dispatch }}>
      {props.children}
    </AppStateContext.Provider>
  )
}

export default AppContext
