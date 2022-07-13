import type { NextPage } from 'next'

import Layout from 'components/Layout'
import StorageStake from 'components/StorageStake'
import { useApp } from 'hooks/useApp'

const Home: NextPage = () => {
  const { loading, wallet } = useApp()

  return <Layout>{!loading && wallet.isSignedIn() && <StorageStake />}</Layout>
}

export default Home
