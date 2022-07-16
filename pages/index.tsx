import React from 'react'

import type { NextPage } from 'next'

import Account from 'components/Account'
import CreateAccount from 'components/CreateAccount'
import Layout from 'components/Layout'
import Spinner from 'components/Spinner'
import StorageStake from 'components/StorageStake'
import { useApp } from 'hooks/useApp'

const Home: NextPage = () => {
  const { loading, wallet, user } = useApp()

  return (
    <Layout>
      {loading ? (
        <div className="flex-grow flex flex-col items-center justify-center">
          <Spinner className="animate-spin text-gray-500 h-16 w-16" />
        </div>
      ) : !wallet.isSignedIn() ? (
        <React.Fragment></React.Fragment>
      ) : (
        <div className="flex flex-col items-center mt-2">
          <StorageStake />
          {user.registered && (
            <React.Fragment>
              {user.accounts.map((account) => (
                <Account key={account.accountName} account={account} />
              ))}
              <CreateAccount />
            </React.Fragment>
          )}
        </div>
      )}
    </Layout>
  )
}

export default Home
