import React from 'react'

import type { NextPage } from 'next'

import Layout from 'components/Layout'
import StorageStake from 'components/StorageStake'
import { useApp } from 'hooks/useApp'

const Home: NextPage = () => {
  const { loading, wallet } = useApp()

  return (
    <Layout>
      {loading ? (
        <React.Fragment></React.Fragment>
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
