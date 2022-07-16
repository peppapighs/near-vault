import React, { useEffect, useState } from 'react'

import { Dialog, Transition } from '@headlessui/react'
import { BN } from 'bn.js'
import { NEAR_NOMINATION_EXP } from 'near-api-js/lib/utils/format'

import { StorageStakeAction } from 'components/StorageStake'
import { nearSymbol } from 'constants/near'
import { useApp } from 'hooks/useApp'
import { classNames } from 'utils/classNames'
import { parseTokenAmount } from 'utils/formatToken'
import vaultContract from 'utils/vaultContract'

interface Props {
  open: boolean
  setOpen: React.Dispatch<React.SetStateAction<boolean>>
  action: StorageStakeAction
}

const StorageStakeModal = ({ open, setOpen, action }: Props) => {
  const TITLE =
    action === 'deposit'
      ? 'Deposit Storage?'
      : action === 'withdraw'
      ? 'Withdraw Storage?'
      : action === 'unregister'
      ? 'Unregister Storage?'
      : ''

  const DESCRIPTION =
    action === 'deposit'
      ? [
          `Enter the amount of ${nearSymbol} you want to deposit below (recommended: 0.1 ${nearSymbol}).`,
        ]
      : action === 'withdraw'
      ? [
          `Enter the amount of ${nearSymbol} you want to withdraw below.`,
          'Leave the amount empty to withdraw all.',
        ]
      : action === 'unregister'
      ? [
          'This action cannot be undone! You will lose all your accounts (including stored tokens).',
          'Make sure you have withdrawn your tokens from all of your accounts.',
          'Enter your wallet ID below to proceed.',
        ]
      : ['']

  const INPUT_LABEL =
    action === 'deposit'
      ? 'Amount to deposit'
      : action === 'withdraw'
      ? 'Amount to withdraw'
      : action === 'unregister'
      ? 'Your wallet ID'
      : ''

  const CONFIRM_BUTTON_LABEL =
    action === 'deposit'
      ? 'Deposit'
      : action === 'withdraw'
      ? 'Withdraw'
      : action === 'unregister'
      ? 'Unregister'
      : ''

  const { wallet } = useApp()

  const [input, setInput] = useState<string>('')
  const [disableConfirm, setDisableConfirm] = useState<boolean>(true)
  const [error, setError] = useState<string>('')

  const closeModal = () => {
    setOpen(false)
    setInput('')
    setDisableConfirm(true)
    setError('')
  }

  const onSubmit = () => {
    setError('')
    switch (action) {
      case 'deposit':
        handleDeposit()
        break
      case 'withdraw':
        handleWithdraw()
        break
      case 'unregister':
        handleUnregister()
        break
    }
  }

  const handleDeposit = async () => {
    try {
      await vaultContract(wallet.account()).storage_deposit(
        { account_id: wallet.getAccountId() },
        undefined,
        parseTokenAmount(input, NEAR_NOMINATION_EXP)!
      )
      closeModal()
    } catch (error) {
      if (error instanceof Error) {
        setError(error.message)
      }
    }
  }

  const handleWithdraw = async () => {
    try {
      await vaultContract(wallet.account()).storage_withdraw(
        input === ''
          ? {}
          : { amount: parseTokenAmount(input, NEAR_NOMINATION_EXP)! },
        undefined,
        '1'
      )
      closeModal()
    } catch (error) {
      if (error instanceof Error) {
        setError(error.message)
      }
    }
  }

  const handleUnregister = async () => {
    try {
      await vaultContract(wallet.account()).storage_unregister(
        { force: true },
        undefined,
        '1'
      )
      closeModal()
    } catch (error) {
      if (error instanceof Error) {
        setError(error.message)
      }
    }
  }

  useEffect(() => {
    setError('')
    if (input === '') {
      setDisableConfirm(action !== 'withdraw')
      return
    }
    try {
      switch (action) {
        case 'deposit':
        case 'withdraw':
          const amount = parseTokenAmount(input, NEAR_NOMINATION_EXP)
          if (!amount) {
            throw new Error('Invalid amount')
          }
          new BN(amount)
          break
        case 'unregister':
          if (input !== wallet.getAccountId()) {
            throw new Error('Wallet ID does not match')
          }
          break
      }
      setDisableConfirm(false)
    } catch (error) {
      if (error instanceof Error) {
        setDisableConfirm(true)
        setError(error.message)
      }
    }
  }, [action, input, wallet])

  return (
    <Transition appear show={open} as={React.Fragment}>
      <Dialog as="div" className="relative z-10" onClose={closeModal}>
        <Transition.Child
          as={React.Fragment}
          enter="ease-out duration-300"
          enterFrom="opacity-0"
          enterTo="opacity-100"
          leave="ease-in duration-200"
          leaveFrom="opacity-100"
          leaveTo="opacity-0"
        >
          <div className="fixed inset-0 bg-gray-300 bg-opacity-90" />
        </Transition.Child>

        <div className="fixed inset-0 overflow-y-auto">
          <div className="flex min-h-full items-center justify-center p-4 text-center">
            <Transition.Child
              as={React.Fragment}
              enter="ease-out duration-300"
              enterFrom="opacity-0 scale-95"
              enterTo="opacity-100 scale-100"
              leave="ease-in duration-200"
              leaveFrom="opacity-100 scale-100"
              leaveTo="opacity-0 scale-95"
            >
              <Dialog.Panel className="w-full max-w-md transform overflow-hidden rounded-md bg-gray-300 p-6 text-left align-middle neumorphic-pressed transition-all">
                <Dialog.Title
                  as="h3"
                  className="text-lg font-medium leading-6 text-gray-900"
                >
                  {TITLE}
                </Dialog.Title>
                <div className="mt-2 flex flex-col space-y-4">
                  {DESCRIPTION.map((line, index) => (
                    <p key={index} className="text-sm text-gray-600">
                      {line}
                    </p>
                  ))}
                </div>

                {action === 'deposit' || action === 'withdraw' ? (
                  <div className="mt-2">
                    <label className="block text-sm font-medium text-gray-800">
                      {INPUT_LABEL}
                    </label>
                    <div
                      className={classNames(
                        error !== '' ? 'border border-red-500' : '',
                        'mt-1 py-2 rounded-md neumorphic-pressed-sm flex space-x-2'
                      )}
                    >
                      <input
                        type="text"
                        value={input}
                        onChange={(e) => setInput(e.target.value)}
                        className="block bg-transparent w-full pl-3 focus:outline-none"
                        placeholder="0.00"
                        aria-describedby="currency"
                      />
                      <div className="pr-3 flex items-center pointer-events-none">
                        <span
                          className="text-gray-600 sm:text-sm"
                          id="currency"
                        >
                          {nearSymbol}
                        </span>
                      </div>
                    </div>
                  </div>
                ) : (
                  <div className="mt-2">
                    <label className="block text-sm font-medium text-gray-800">
                      {INPUT_LABEL}
                    </label>
                    <div className="mt-1">
                      <input
                        type="text"
                        value={input}
                        onChange={(e) => setInput(e.target.value)}
                        className={classNames(
                          error !== '' ? 'border border-red-500' : '',
                          'block bg-transparent px-3 py-2 neumorphic-pressed-sm w-full sm:text-sm border-gray-300 rounded-md focus:outline-none'
                        )}
                      />
                    </div>
                  </div>
                )}

                {error && (
                  <div className="mt-2 text-red-700 text-sm">{error}</div>
                )}

                <div className="mt-4 grid grid-cols-2 gap-4">
                  <button
                    type="button"
                    onClick={closeModal}
                    className="text-gray-700 inline-flex items-center justify-center px-4 py-2 border border-gray-400 text-sm font-medium rounded-md neumorphic-flat-sm hover:neumorphic-pressed-sm focus:neumorphic-pressed-sm focus:outline-none"
                  >
                    Cancel
                  </button>
                  <button
                    type="button"
                    disabled={disableConfirm}
                    onClick={onSubmit}
                    className={classNames(
                      !disableConfirm
                        ? 'hover:neumorphic-pressed-sm '
                        : 'text-opacity-50',
                      'text-red-700 inline-flex items-center justify-center px-4 py-2 border border-gray-400 text-sm font-medium rounded-md neumorphic-flat-sm focus:neumorphic-pressed-sm focus:outline-none'
                    )}
                  >
                    {CONFIRM_BUTTON_LABEL}
                  </button>
                </div>
              </Dialog.Panel>
            </Transition.Child>
          </div>
        </div>
      </Dialog>
    </Transition>
  )
}

export default StorageStakeModal
