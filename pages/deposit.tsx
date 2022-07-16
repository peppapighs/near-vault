import React, { useEffect } from 'react'

import type { NextPage } from 'next'
import { useRouter } from 'next/router'

import Layout from 'components/Layout'
import Spinner from 'components/Spinner'
import { useApp } from 'hooks/useApp'

const Deposit: NextPage = () => {
  const router = useRouter()

  const { loading, wallet, user } = useApp()

  useEffect(() => {
    const query = router.query as { accountName: string }
    if (
      !loading &&
      (!wallet.isSignedIn() ||
        !query.accountName ||
        !user.accounts.find(
          (account) => account.accountName === query.accountName
        ))
    ) {
      router.push('/')
    }
  }, [router, loading, wallet, user])

  return (
    <Layout>
      {loading ? (
        <div className="flex-grow flex flex-col items-center justify-center">
          <Spinner className="animate-spin text-gray-500 h-16 w-16" />
        </div>
      ) : (
        <React.Fragment></React.Fragment>
      )}
    </Layout>
  )
}

export default Deposit
