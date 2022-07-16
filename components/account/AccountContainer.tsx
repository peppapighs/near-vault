import React from 'react'

import { useApp } from 'hooks/useApp'

import Account from './Account'

const AccountContainer = () => {
  const { user } = useApp()

  return (
    <React.Fragment>
      {user.accounts.map((account) => (
        <Account key={account.accountName} account={account} />
      ))}
      
    </React.Fragment>
  )
}

export default AccountContainer
