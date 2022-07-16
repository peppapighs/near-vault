import React, { useState } from 'react'

import { Disclosure } from '@headlessui/react'
import { ChevronDownIcon, ChevronLeftIcon } from '@heroicons/react/solid'
import { formatNearAmount } from 'near-api-js/lib/utils/format'

import { nearSymbol } from 'constants/near'
import { useApp } from 'hooks/useApp'
import { classNames } from 'utils/classNames'

import StorageStakeModal from './StorageStakeModal'

const DISPLAY_FRACTION_DIGITS = 6

export type StorageStakeAction = 'deposit' | 'withdraw' | 'unregister' | ''

const StorageStake = () => {
  const { user } = useApp()

  const [open, setOpen] = useState<boolean>(false)
  const [action, setAction] = useState<StorageStakeAction>('')

  return (
    <React.Fragment>
      <Disclosure
        defaultOpen={!user.registered}
        as="div"
        className="w-full max-w-3xl p-4"
      >
        {({ open }) => (
          <div className="border border-gray-400 rounded-md neumorphic-flat flex flex-col">
            <Disclosure.Button className="flex justify-between space-x-3 p-4 focus:outline-none">
              <h1 className="my-auto text-base font-bold text-gray-900">
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
              <div className="my-auto flex flex-col gap-2 item-centers text-gray-600 my-2 sm:grid sm:grid-cols-2 sm:gap-4">
                {!user.registered && (
                  <div className="px-4 py-2 rounded-md neumorphic-pressed col-span-2">
                    <p className="text-sm font-normal">
                      Looks like you are not registered! Please register your
                      account by depositing storage below.
                    </p>
                  </div>
                )}
                {user.registered && (
                  <div className="px-4 py-2 rounded-md neumorphic-pressed">
                    <label className="text-sm font-normal">
                      Total storage:
                    </label>
                    <p className="text-lg font-semibold sm:text-xl">
                      {`${formatNearAmount(
                        user.storageBalance.total,
                        DISPLAY_FRACTION_DIGITS
                      )} ${nearSymbol}`}
                    </p>
                  </div>
                )}
                {user.registered && (
                  <div className="px-4 py-2 rounded-md neumorphic-pressed">
                    <label className="text-sm font-normal">
                      Available storage:
                    </label>
                    <p className="text-lg font-semibold sm:text-xl">
                      {`${formatNearAmount(
                        user.storageBalance.available,
                        DISPLAY_FRACTION_DIGITS
                      )} ${nearSymbol}`}
                    </p>
                  </div>
                )}
              </div>
              <div className="py-2 grid grid-cols-3 gap-2 sm:gap-4">
                <button
                  type="button"
                  onClick={() => {
                    setAction('deposit')
                    setOpen(true)
                  }}
                  className="text-gray-700 inline-flex items-center justify-center px-4 py-2 border border-gray-400 text-sm font-medium rounded-md neumorphic-flat-sm hover:neumorphic-pressed-sm focus:neumorphic-pressed-sm focus:outline-none"
                >
                  Deposit
                </button>
                <button
                  type="button"
                  disabled={!user.registered}
                  onClick={() => {
                    setAction('withdraw')
                    setOpen(true)
                  }}
                  className={classNames(
                    user.registered
                      ? 'hover:neumorphic-pressed-sm'
                      : 'text-opacity-50',
                    'text-gray-700 inline-flex items-center justify-center px-4 py-2 border border-gray-400 text-sm font-medium rounded-md neumorphic-flat-sm focus:neumorphic-pressed-sm focus:outline-none'
                  )}
                >
                  Withdraw
                </button>
                <button
                  type="button"
                  disabled={!user.registered}
                  onClick={() => {
                    setAction('unregister')
                    setOpen(true)
                  }}
                  className={classNames(
                    user.registered
                      ? 'hover:neumorphic-pressed-sm'
                      : 'text-opacity-50',
                    'text-gray-700 inline-flex items-center justify-center px-4 py-2 border border-gray-400 text-sm font-medium rounded-md neumorphic-flat-sm focus:neumorphic-pressed-sm focus:outline-none'
                  )}
                >
                  Unregister
                </button>
              </div>
            </Disclosure.Panel>
          </div>
        )}
      </Disclosure>

      <StorageStakeModal open={open} setOpen={setOpen} action={action} />
    </React.Fragment>
  )
}

export default StorageStake
