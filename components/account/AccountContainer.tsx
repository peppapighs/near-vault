import React from 'react'

import { useApp } from 'hooks/useApp'

import Account from './Account'
import { AccountProvider } from './AccountContext'
import AccountModal from './AccountModal'

const AccountContainer = () => {
  const { user } = useApp()

  return (
    <AccountProvider>
      {user.accounts.map((account) => (
        <Account key={account.accountName} account={account} />
      ))}

      <AccountModal />
    </AccountProvider>
  )
}

export default AccountContainer
