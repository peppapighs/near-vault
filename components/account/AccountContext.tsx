import React, { useContext, useReducer } from 'react'

import { Account as IAccount } from 'hooks/useApp'

type Action = 'deposit' | 'withdraw' | 'transfer' | ''

interface AccountState {
  open: boolean
  action: Action
  account: IAccount
}

const initialState: AccountState = {
  open: false,
  action: '',
  account: {
    accountName: '',
    balance: '0',
  },
}

type AccountAction =
  | { type: 'SET_ACCOUNT_STATE'; payload: AccountState }
  | { type: 'SET_OPEN'; payload: boolean }

const reducer = (state: AccountState, action: AccountAction) => {
  switch (action.type) {
    case 'SET_ACCOUNT_STATE':
      return { ...state, ...action.payload }
    case 'SET_OPEN':
      return { ...state, open: action.payload }
  }
}

type AccountContext = AccountState & {
  dispatch: React.Dispatch<AccountAction>
}

const AccountStateContext = React.createContext<AccountContext>({
  ...initialState,
  dispatch: () => {},
})

export const useAccountState = () => useContext(AccountStateContext)

interface Props {
  children: React.ReactNode
}

export const AccountProvider = ({ children }: Props) => {
  const [state, dispatch] = useReducer(reducer, initialState)

  return (
    <AccountStateContext.Provider value={{ ...state, dispatch }}>
      {children}
    </AccountStateContext.Provider>
  )
}
