import React, { useEffect } from 'react'

import { Disclosure } from '@headlessui/react'
import { ChevronDownIcon, ChevronLeftIcon } from '@heroicons/react/solid'
import { formatNearAmount } from 'near-api-js/lib/utils/format'
import { useRouter } from 'next/router'

import { nearSymbol } from 'constants/near'
import { useApp } from 'hooks/useApp'
import { classNames } from 'utils/classNames'

const DISPLAY_FRACTION_DIGITS = 6

const StorageStake = () => {
  const { user } = useApp()

  return (
    <React.Fragment>
      <Disclosure
        defaultOpen={!user.registered}
        as="div"
        className="flex justify-center p-4"
      >
        {({ open }) => (
          <div className="flex-grow max-w-5xl border border-gray-400 rounded neumorphic-flat flex flex-col">
            <Disclosure.Button className="flex justify-between space-x-3 p-4 focus:outline-none">
              <h1 className="my-auto text-normal font-bold text-gray-800">
                Your Storage Balance
              </h1>
              <div className="my-auto text-gray-700 inline-flex items-center justify-center">
                <span className="sr-only">Open storage stake</span>
                {open ? (
                  <ChevronDownIcon
                    className="block h-6 w-6"
                    aria-hidden="true"
                  />
                ) : (
                  <ChevronLeftIcon
                    className="block h-6 w-6"
                    aria-hidden="true"
                  />
                )}
              </div>
            </Disclosure.Button>

            <Disclosure.Panel className="flex flex-col px-4 pb-2">
              <div className="my-auto flex flex-col item-centers text-sm font-normal text-gray-600 p-4 my-2 rounded neumorphic-pressed">
                {!user.registered && (
                  <p>
                    Looks like you are not registered! Please register your
                    account by depositing storage below.
                  </p>
                )}
                {user.registered && (
                  <p className="font-semibold">
                    Total storage:{' '}
                    <span className="font-normal">
                      {` ${formatNearAmount(
                        user.storageBalance.total,
                        DISPLAY_FRACTION_DIGITS
                      )} ${nearSymbol}`}
                    </span>
                  </p>
                )}
                {user.registered && (
                  <p className="font-semibold">
                    Available storage:{' '}
                    <span className="font-normal">
                      {` ${formatNearAmount(
                        user.storageBalance.available,
                        DISPLAY_FRACTION_DIGITS
                      )} ${nearSymbol}`}
                    </span>
                  </p>
                )}
              </div>
              <div className="py-2 flex flex-col gap-2 sm:block sm:grid sm:grid-cols-3 sm:gap-4">
                <button
                  type="button"
                  className="text-gray-700 inline-flex items-center justify-center px-4 py-2 border border-gray-400 text-sm font-medium rounded-md neumorphic-flat-sm hover:neumorphic-pressed-sm focus:neumorphic-pressed-sm focus:outline-none"
                >
                  Deposit
                </button>
                <button
                  type="button"
                  disabled={!user.registered}
                  className={classNames(
                    user.registered
                      ? 'text-gray-700 hover:neumorphic-pressed-sm'
                      : 'text-gray-400',
                    'nline-flex items-center justify-center px-4 py-2 border border-gray-400 text-sm font-medium rounded-md neumorphic-flat-sm focus:neumorphic-pressed-sm focus:outline-none'
                  )}
                >
                  Withdraw
                </button>
                <button
                  type="button"
                  disabled={!user.registered}
                  className={classNames(
                    user.registered
                      ? 'text-gray-700 hover:neumorphic-pressed-sm'
                      : 'text-gray-400',
                    'nline-flex items-center justify-center px-4 py-2 border border-gray-400 text-sm font-medium rounded-md neumorphic-flat-sm focus:neumorphic-pressed-sm focus:outline-none'
                  )}
                >
                  Unregister
                </button>
              </div>
            </Disclosure.Panel>
          </div>
        )}
      </Disclosure>
    </React.Fragment>
  )
}

export default StorageStake
