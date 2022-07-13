import { ConnectConfig } from 'near-api-js'

export const nearConfig: ConnectConfig = {
  headers: {},
  networkId: 'testnet',
  nodeUrl: 'https://rpc.testnet.near.org',
  walletUrl: 'https://wallet.testnet.near.org',
  helperUrl: 'https://helper.testnet.near.org',
}

export const nearSymbol = 'NEAR'
