import { serialize, Schema } from 'borsh'

class TransferMessage {
  action: string
  payload: Uint8Array

  constructor({ action, payload }: { action: string; payload: Uint8Array }) {
    this.action = action
    this.payload = payload
  }
}

class DepositPayload {
  account_name: string

  constructor({ account_name }: { account_name: string }) {
    this.account_name = account_name
  }
}

const DEPOSIT_PAYLOAD_SCHEMA = new Map([
  [DepositPayload, { kind: 'struct', fields: [['account_name', 'string']] }],
])

export const depositTransferMessage = (accountName: string) => {
  const payload = serialize(
    DEPOSIT_PAYLOAD_SCHEMA,
    new DepositPayload({ account_name: accountName })
  )

  const schema = new Map([
    [
      TransferMessage,
      {
        kind: 'struct',
        fields: [
          ['action', 'string'],
          ['payload', ['u8']],
        ],
      },
    ],
  ])

  return serialize(schema, new TransferMessage({ action: 'deposit', payload }))
}
