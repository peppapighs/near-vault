import React, { useContext, useReducer } from 'react'

import StorageStake from './StorageStake'

type Action = 'deposit' | 'withdraw' | 'unregister' | ''

interface StorageStakeState {
  open: boolean
  action: Action
}

const initialState: StorageStakeState = {
  open: false,
  action: '',
}

type StorageStakeAction =
  | { type: 'SET_STORAGE_STAKE_STATE'; payload: StorageStakeState }
  | { type: 'SET_OPEN'; payload: boolean }

const reducer = (state: StorageStakeState, action: StorageStakeAction) => {
  switch (action.type) {
    case 'SET_STORAGE_STAKE_STATE':
      return { ...state, ...action.payload }
    case 'SET_OPEN':
      return { ...state, open: action.payload }
  }
}

type StorageStakeContext = StorageStakeState & {
  dispatch: React.Dispatch<StorageStakeAction>
}

const StorageStakeStateContext = React.createContext<StorageStakeContext>({
  ...initialState,
  dispatch: () => {},
})

export const useStorageStakeState = () => useContext(StorageStakeStateContext)

export const StorageStakeContainer = () => {
  const [state, dispatch] = useReducer(reducer, initialState)

  return (
    <StorageStakeStateContext.Provider value={{ ...state, dispatch }}>
      <StorageStake />
    </StorageStakeStateContext.Provider>
  )
}
