import React from 'react'

import { Disclosure, Menu, Transition } from '@headlessui/react'
import { MenuIcon, XIcon, UserIcon } from '@heroicons/react/outline'
import { useRouter } from 'next/router'

import { useApp } from 'hooks/useApp'
import { classNames } from 'utils/classNames'
import { formatTokenAmount } from 'utils/formatToken'
import { formatNearAmount } from 'near-api-js/lib/utils/format'
import { nearSymbol } from 'constants/near'

const DISPLAY_FRACTION_DIGITS = 6

const Navbar = () => {
  const router = useRouter()

  const { loading, wallet, tokenContractMetadata, user } = useApp()

  const handleSignIn = () => {
    wallet.requestSignIn()
  }

  const handleSignOut = () => {
    wallet.signOut()
    router.push('/')
  }

  return (
    <Disclosure as="nav" className="bg-gray-300 sm:p-4">
      {({ open }) => (
        <React.Fragment>
          <div
            className={classNames(
              !open ? 'neumorphic-flat' : 'sm:neumorphic-flat',
              'border border-gray-400 sm:rounded'
            )}
          >
            <div className="max-w-7xl mx-auto px-2 sm:px-6 lg:px-8">
              <div className="relative flex items-center justify-between h-14">
                <div className="flex-shrink-0 flex items-center">
                  <h1 className="w-36 flex items-center justify-center text-xl text-gray-800 font-bold">
                    near-vault
                    <span className="inline-flex animate-pulse">_</span>
                  </h1>
                </div>

                <div className="hidden sm:block">
                  {!loading && wallet.isSignedIn() ? (
                    <Menu as="div" className="relative">
                      <Menu.Button
                        type="button"
                        className="inline-flex items-center px-4 py-2 border border-gray-400 text-sm font-medium rounded-md text-gray-700 neumorphic-flat-sm hover:neumorphic-pressed-sm focus:neumorphic-pressed-sm focus:outline-none"
                      >
                        <p className="w-32 text-center whitespace-nowrap text-ellipsis overflow-hidden">
                          {wallet.getAccountId()}
                        </p>
                      </Menu.Button>
                      <Transition
                        as={React.Fragment}
                        enter="transition ease-out duration-100"
                        enterFrom="transform opacity-0 scale-95"
                        enterTo="transform opacity-100 scale-100"
                        leave="transition ease-in duration-75"
                        leaveFrom="transform opacity-100 scale-100"
                        leaveTo="transform opacity-0 scale-95"
                      >
                        <Menu.Items className="absolute right-0 mt-2 w-80 rounded-md bg-gray-300 border border-gray-400 neumorphic-flat pb-2 focus:outline-none">
                          <div className="border-b border-gray-400 flex items-start space-x-3 p-4 mb-2">
                            <div className="flex-shrink-0 p-2 rounded-full neumorphic-pressed-sm">
                              <UserIcon className="h-6 w-6 text-gray-700" />
                            </div>
                            <div className="flex flex-col flex-grow space-y-1">
                              <p className="text-base font-medium text-gray-800 whitespace-nowrap text-ellipsis overflow-hidden">
                                {wallet.getAccountId()}
                              </p>
                              <ul className="text-sm font-normal text-gray-700">
                                <li>
                                  Total balance:
                                  {` ${formatTokenAmount(
                                    user.tokenBalance,
                                    tokenContractMetadata.decimals,
                                    DISPLAY_FRACTION_DIGITS
                                  )} ${tokenContractMetadata.symbol}`}
                                </li>
                                <li>
                                  Total storage:
                                  {` ${formatNearAmount(
                                    user.storageBalance.total,
                                    DISPLAY_FRACTION_DIGITS
                                  )} ${nearSymbol}`}
                                </li>
                                <li>
                                  Available storage:
                                  {` ${formatNearAmount(
                                    user.storageBalance.available,
                                    DISPLAY_FRACTION_DIGITS
                                  )} ${nearSymbol}`}
                                </li>
                              </ul>
                            </div>
                          </div>
                          <Menu.Item>
                            {({ active }) => (
                              <span
                                onClick={handleSignOut}
                                className={classNames(
                                  active
                                    ? 'neumorphic-pressed-sm cursor-pointer'
                                    : '',
                                  'block px-4 py-2 text-sm text-gray-700'
                                )}
                              >
                                Sign out
                              </span>
                            )}
                          </Menu.Item>
                        </Menu.Items>
                      </Transition>
                    </Menu>
                  ) : (
                    <button
                      type="button"
                      onClick={handleSignIn}
                      disabled={loading}
                      className={classNames(
                        !loading
                          ? 'hover:neumorphic-pressed-sm text-gray-700'
                          : 'text-gray-400',
                        'inline-flex items-center px-4 py-2 border border-gray-400 text-sm font-medium rounded-md neumorphic-flat-sm focus:neumorphic-pressed-sm focus:outline-none'
                      )}
                    >
                      <p className="w-16 text-center">
                        {loading ? '...' : 'Sign in'}
                      </p>
                    </button>
                  )}
                </div>

                <div className="block sm:hidden">
                  <Disclosure.Button
                    disabled={loading}
                    className={classNames(
                      !loading
                        ? 'hover:neumorphic-pressed-sm text-gray-700'
                        : 'text-gray-400',
                      'inline-flex items-center justify-center p-2 rounded-md focus:neumorphic-pressed-sm focus:outline-none'
                    )}
                  >
                    <span className="sr-only">Open menu</span>
                    {open ? (
                      <XIcon className="block h-6 w-6" aria-hidden="true" />
                    ) : (
                      <MenuIcon className="block h-6 w-6" aria-hidden="true" />
                    )}
                  </Disclosure.Button>
                </div>
              </div>
            </div>
          </div>

          <Disclosure.Panel className="neumorphic-pressed border-b border-gray-400 sm:hidden">
            {!loading && wallet.isSignedIn() ? (
              <React.Fragment>
                <div className="pt-4 pb-3">
                  <div className="flex items-start space-x-3 px-5">
                    <div className="flex-shrink-0 p-2 rounded-full neumorphic-pressed-sm">
                      <UserIcon className="h-6 w-6 text-gray-700" />
                    </div>
                    <div className="flex flex-col flex-grow space-y-1">
                      <p className="text-base font-medium text-gray-800 whitespace-nowrap text-ellipsis overflow-hidden">
                        {wallet.getAccountId()}
                      </p>
                      <ul className="text-sm font-normal text-gray-700">
                        <li>
                          Total balance:
                          {` ${formatTokenAmount(
                            user.tokenBalance,
                            tokenContractMetadata.decimals,
                            DISPLAY_FRACTION_DIGITS
                          )} ${tokenContractMetadata.symbol}`}
                        </li>
                        <li>
                          Total storage:
                          {` ${formatNearAmount(
                            user.storageBalance.total,
                            DISPLAY_FRACTION_DIGITS
                          )} ${nearSymbol}`}
                        </li>
                        <li>
                          Available storage:
                          {` ${formatNearAmount(
                            user.storageBalance.available,
                            DISPLAY_FRACTION_DIGITS
                          )} ${nearSymbol}`}
                        </li>
                      </ul>
                    </div>
                  </div>
                  <div className="pt-3 px-2 space-y-1">
                    <Disclosure.Button
                      onClick={handleSignOut}
                      className="w-full text-left block px-3 py-2 rounded-md text-base font-medium text-gray-700 hover:neumorphic-pressed focus:neumorphic-pressed"
                    >
                      Sign out
                    </Disclosure.Button>
                  </div>
                </div>
              </React.Fragment>
            ) : (
              <div className="px-2 pt-2 pb-3 space-y-1">
                <Disclosure.Button
                  onClick={handleSignIn}
                  className="w-full text-left block px-3 py-2 rounded-md text-base font-medium text-gray-700 hover:neumorphic-pressed focus:neumorphic-pressed"
                >
                  Sign in
                </Disclosure.Button>
              </div>
            )}
          </Disclosure.Panel>
        </React.Fragment>
      )}
    </Disclosure>
  )
}

export default Navbar
