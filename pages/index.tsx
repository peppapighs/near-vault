import React from 'react'

import type { NextPage } from 'next'

import Layout from 'components/Layout'
import Spinner from 'components/Spinner'
import StorageStake from 'components/StorageStake'
import { useApp } from 'hooks/useApp'

const Home: NextPage = () => {
  const { loading, wallet } = useApp()

  return (
    <Layout>
      {loading ? (
        <div className="flex-grow flex flex-col items-center justify-center">
          <Spinner className="animate-spin text-gray-500 h-16 w-16" />
        </div>
      ) : !wallet.isSignedIn() ? (
        <React.Fragment></React.Fragment>
      ) : (
        <div className="mt-2">
          <StorageStake />
        </div>
      )}
    </Layout>
  )
}

export default Home
