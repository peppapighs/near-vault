import { CurrencyDollarIcon } from '@heroicons/react/outline'
import { useRouter } from 'next/router'

import { Account as IAccount, useApp } from 'hooks/useApp'
import { formatTokenAmount } from 'utils/formatToken'

interface Props {
  account: IAccount
}

const DISPLAY_FRACTION_DIGITS = 10

const Account = (props: Props) => {
  const router = useRouter()

  const { tokenContractMetadata } = useApp()

  return (
    <div className="w-full max-w-3xl px-4 py-2">
      <div className="p-4 w-full border border-gray-400 rounded-md neumorphic-flat">
        <div className="flex flex-col justify-between">
          <div className="flex flex-col gap-2 sm:gap-4">
            <p className="text-gray-900 text-lg font-bold whitespace-nowrap text-ellipsis overflow-hidden sm:text-xl">
              <CurrencyDollarIcon className="inline-flex h-6 w-6 mb-1 mr-1" />
              {props.account.accountName}
            </p>
            <div className="text-gray-600 px-4 py-2 rounded-md neumorphic-pressed">
              <label className="text-sm font-normal">Balance:</label>
              <p className="text-lg font-bold whitespace-nowrap text-ellipsis overflow-hidden sm:text-xl">
                {`${formatTokenAmount(
                  props.account.balance,
                  tokenContractMetadata.decimals,
                  DISPLAY_FRACTION_DIGITS
                )} ${tokenContractMetadata.symbol}`}
              </p>
            </div>
          </div>
          <div className="mt-4 flex flex-col grid grid-cols-3 gap-2 sm:gap-4">
            <button
              type="button"
              onClick={() =>
                router.push({
                  pathname: '/deposit',
                  query: { accountName: props.account.accountName },
                })
              }
              className="text-gray-700 inline-flex items-center justify-center px-4 py-2 border border-gray-400 text-sm font-medium rounded-md neumorphic-flat-sm hover:neumorphic-pressed-sm focus:neumorphic-pressed-sm focus:outline-none"
            >
              Deposit
            </button>
            <button
              type="button"
              className="text-gray-700 inline-flex items-center justify-center px-4 py-2 border border-gray-400 text-sm font-medium rounded-md neumorphic-flat-sm hover:neumorphic-pressed-sm focus:neumorphic-pressed-sm focus:outline-none"
            >
              Withdraw
            </button>
            <button
              type="button"
              className="text-gray-700 inline-flex items-center justify-center px-4 py-2 border border-gray-400 text-sm font-medium rounded-md neumorphic-flat-sm hover:neumorphic-pressed-sm focus:neumorphic-pressed-sm focus:outline-none"
            >
              Transfer
            </button>
          </div>
        </div>
      </div>
    </div>
  )
}

export default Account
