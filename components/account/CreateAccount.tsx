import React, { useEffect, useState } from 'react'

import { Dialog, Transition } from '@headlessui/react'

import { useApp } from 'hooks/useApp'
import { classNames } from 'utils/classNames'
import vaultContract from 'utils/vaultContract'

const CreateAccount = () => {
  const { wallet } = useApp()

  const [open, setOpen] = useState<boolean>(false)
  const [input, setInput] = useState<string>('')
  const [error, setError] = useState<string>('')
  const [disableConfirm, setDisableConfirm] = useState<boolean>(true)

  const closeModal = () => {
    setOpen(false)
    setInput('')
    setDisableConfirm(true)
    setError('')
  }

  const onSubmit = async () => {
    setError('')
    try {
      await vaultContract(wallet.account()).create_account({
        account_name: input,
      })
      closeModal()
    } catch (error) {
      if (error instanceof Error) {
        setError(error.message)
      }
    }
  }

  useEffect(() => {
    const checkUnique = async () => {
      try {
        if (
          (await vaultContract(wallet.account()).get_balance({
            account_name: input,
          })) !== null
        ) {
          throw new Error('Account name already exists')
        }
        setError('')
        setDisableConfirm(false)
      } catch (error) {
        if (error instanceof Error) {
          setDisableConfirm(true)
          setError(error.message)
        }
      }
    }

    setDisableConfirm(true)
    if (input === '') {
      setError('')
      return
    }
    checkUnique()
  }, [wallet, input])

  return (
    <React.Fragment>
      <div className="w-full max-w-3xl px-4 py-2">
        <button
          type="button"
          onClick={() => setOpen(true)}
          className="p-4 w-full h-36 text-gray-700 border-2 border-dashed border-gray-400 text-base font-bold text-center rounded-md neumorphic-flat hover:neumorphic-pressed focus:neumorphic-pressed focus:outline-none"
        >
          + Create new account
        </button>
      </div>

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
                    Create new account?
                  </Dialog.Title>
                  <div className="mt-2 flex flex-col space-y-4">
                    <p className="text-sm text-gray-600">
                      Enter your new account name below (must be unique).
                    </p>
                    <p className="text-sm text-gray-600">
                      You may need to deposit more storage if the current amount
                      is insufficient.
                    </p>
                  </div>

                  <div className="mt-2">
                    <label className="block text-sm font-medium text-gray-800">
                      Account name
                    </label>
                    <div className="mt-1">
                      <input
                        type="text"
                        value={input}
                        onChange={(e) => setInput(e.target.value)}
                        className="block bg-transparent px-3 py-2 neumorphic-pressed-sm w-full sm:text-sm border-gray-300 rounded-md focus:outline-none"
                      />
                    </div>
                  </div>

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
                      Create new account
                    </button>
                  </div>
                </Dialog.Panel>
              </Transition.Child>
            </div>
          </div>
        </Dialog>
      </Transition>
    </React.Fragment>
  )
}

export default CreateAccount
