import { useEffect, useState } from 'react'

import {
  connect,
  ConnectConfig,
  keyStores,
  Near,
  WalletConnection,
} from 'near-api-js'

interface Props {
  near: Near | null
  wallet: WalletConnection | null
  loading: boolean
}

const initialState: Props = {
  near: null,
  wallet: null,
  loading: false,
}

export const useNearAPI = () => {
  const [state, setState] = useState<Props>(initialState)

  useEffect(() => {
    const config: ConnectConfig = {
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

      setState({ near, wallet, loading: false })
    }

    setState({ ...state, loading: true })
    connectToNear()
  }, [])

  return state
}
