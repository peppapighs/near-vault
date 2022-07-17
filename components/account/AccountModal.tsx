import React, { useEffect, useMemo, useState } from 'react'

import { Dialog, Transition } from '@headlessui/react'
import { BN } from 'bn.js'
import bs58 from 'bs58'

import { useApp } from 'hooks/useApp'
import { classNames } from 'utils/classNames'
import { formatTokenAmount, parseTokenAmount } from 'utils/formatToken'
import tokenContract from 'utils/tokenContract'
import { depositTransferMessage } from 'utils/transferMessage'
import vaultContract from 'utils/vaultContract'

import { useAccountState } from './AccountContext'

const DISPLAY_FRACTION_DIGITS = 6

const DEFAULT_MULTICALL_GAS = new BN('30000000000000')
  .add(new BN('1'))
  .toString()

const AccountModal = () => {
  const {
    wallet,
    user,
    vaultContractMetadata: { transfer_fee_numerator, transfer_fee_denominator },
    tokenContractMetadata: { symbol, decimals },
  } = useApp()
  const { open, action, account, dispatch } = useAccountState()

  const state = useMemo(() => {
    return {
      title:
        action === 'deposit'
          ? `Deposit ${symbol}?`
          : action === 'withdraw'
          ? `Withdraw ${symbol}?`
          : action === 'transfer'
          ? `Transfer ${symbol} to another account?`
          : '',
      description:
        action === 'deposit'
          ? [
              `Enter the amount of ${symbol} to deposit into this account below.`,
            ]
          : action === 'withdraw'
          ? [
              `Enter the amount of ${symbol} to withdraw from this account below.`,
            ]
          : action === 'transfer'
          ? [
              `Enter the name of the account and the amount of ${symbol} you want to transfer to below.`,
              `A ${new BN(transfer_fee_numerator)
                .mul(new BN(100))
                .div(new BN(transfer_fee_denominator))
                .toString()}% fee of the transferred amount will be imposed if transferring to an account of another user.`,
            ]
          : [''],
      inputLabel:
        action === 'deposit'
          ? 'Amount to deposit'
          : action === 'withdraw'
          ? 'Amount to withdraw'
          : action === 'transfer'
          ? 'Amount to transfer'
          : '',
      confirmButtonLabel:
        action === 'deposit'
          ? 'Deposit'
          : action === 'withdraw'
          ? 'Withdraw'
          : action === 'transfer'
          ? 'Transfer'
          : '',
    }
  }, [action, symbol, transfer_fee_numerator, transfer_fee_denominator])

  const [input, setInput] = useState<string>('')
  const [disableConfirm, setDisableConfirm] = useState<boolean>(true)
  const [error, setError] = useState<string>('')

  const [toAccount, setToAccount] = useState<string>('')

  const closeModal = () => {
    dispatch({ type: 'SET_OPEN', payload: false })
    setInput('')
    setDisableConfirm(true)
    setError('')
    setToAccount('')
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
      case 'transfer':
        handleTransfer()
        break
    }
  }

  const handleDeposit = async () => {
    try {
      await tokenContract(wallet.account()).ft_transfer_call(
        {
          receiver_id: `${process.env.NEXT_PUBLIC_VAULT_CONTRACT}`,
          amount: parseTokenAmount(input, decimals)!,
          memo: null,
          msg: bs58.encode(depositTransferMessage(account.accountName)),
        },
        DEFAULT_MULTICALL_GAS,
        '1'
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
      await vaultContract(wallet.account()).withdraw(
        {
          account_name: account.accountName,
          amount: parseTokenAmount(input, decimals)!,
        },
        DEFAULT_MULTICALL_GAS,
        '1'
      )
      closeModal()
    } catch (error) {
      if (error instanceof Error) {
        setError(error.message)
      }
    }
  }

  const handleTransfer = async () => {
    try {
      await vaultContract(wallet.account()).transfer({
        sender_account_name: account.accountName,
        receiver_account_name: toAccount,
        amount: parseTokenAmount(input, decimals)!,
      })
      closeModal()
    } catch (error) {
      if (error instanceof Error) {
        setError(error.message)
      }
    }
  }

  const feeText = useMemo(() => {
    if (disableConfirm) {
      return ''
    }
    if (
      user.accounts.find((userAccount) => userAccount.accountName === toAccount)
    ) {
      return 'No fee'
    }
    try {
      const amount = new BN(parseTokenAmount(input, decimals)!)
      const fee = amount
        .mul(new BN(transfer_fee_numerator))
        .div(new BN(transfer_fee_denominator))

      if (fee.isZero()) {
        return 'No fee'
      } else {
        return `Fee ${new BN(transfer_fee_numerator)
          .mul(new BN(100))
          .div(new BN(transfer_fee_denominator))
          .toString()}% = ${formatTokenAmount(
          fee.toString(),
          decimals,
          DISPLAY_FRACTION_DIGITS
        )} ${symbol} | Receive = ${formatTokenAmount(
          amount.sub(fee).toString(),
          decimals,
          DISPLAY_FRACTION_DIGITS
        )} ${symbol}`
      }
    } catch {
      return ''
    }
  }, [
    user,
    transfer_fee_numerator,
    transfer_fee_denominator,
    decimals,
    symbol,
    input,
    disableConfirm,
    toAccount,
  ])

  useEffect(() => {
    const checkInput = async () => {
      try {
        if (
          action === 'transfer' &&
          !(await vaultContract(wallet.account()).get_balance({
            account_name: toAccount,
          }))
        ) {
          throw new Error('Receiver account does not exist')
        }
        const amount = parseTokenAmount(input, decimals)
        if (!amount) {
          throw new Error('Invalid amount')
        }
        new BN(amount)
        setError('')
        setDisableConfirm(false)
      } catch (error) {
        if (error instanceof Error) {
          setError(error.message)
          setDisableConfirm(true)
        }
      }
    }

    setDisableConfirm(true)
    if (input === '' || (action === 'transfer' && toAccount === '')) {
      setError('')
      return
    }
    checkInput()
  }, [wallet, symbol, decimals, action, input, toAccount])

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
                  {state.title}
                </Dialog.Title>
                <div className="mt-2 flex flex-col space-y-4">
                  {state.description.map((line, index) => (
                    <p key={index} className="text-sm text-gray-600">
                      {line}
                    </p>
                  ))}
                </div>

                {action === 'transfer' && (
                  <React.Fragment>
                    <div className="mt-2">
                      <label className="block text-sm font-medium text-gray-800">
                        Account to transfer to
                      </label>
                      <div className="mt-1">
                        <input
                          type="text"
                          value={toAccount}
                          onChange={(e) => setToAccount(e.target.value)}
                          className="block bg-transparent px-3 py-2 neumorphic-pressed-sm w-full sm:text-sm border-gray-300 rounded-md focus:outline-none"
                        />
                      </div>
                    </div>
                  </React.Fragment>
                )}
                <div className="mt-2">
                  <label className="block text-sm font-medium text-gray-800">
                    {state.inputLabel}
                  </label>
                  <div className="mt-1 py-2 rounded-md neumorphic-pressed-sm flex space-x-2">
                    <input
                      type="text"
                      value={input}
                      onChange={(e) => setInput(e.target.value)}
                      className="block bg-transparent w-full pl-3 focus:outline-none"
                      placeholder="0.00"
                      aria-describedby="currency"
                    />
                    <div className="pr-3 flex items-center pointer-events-none">
                      <span className="text-gray-600 sm:text-sm" id="currency">
                        {symbol}
                      </span>
                    </div>
                  </div>
                </div>

                {error !== '' && (
                  <div className="mt-2 text-red-700 text-sm">{error}</div>
                )}

                {action === 'transfer' && feeText !== '' && (
                  <div className="mt-2 text-sm text-gray-600">{feeText}</div>
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
                    {state.confirmButtonLabel}
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

export default AccountModal
