import { useApp } from 'hooks/useApp'

const Welcome = () => {
  const { wallet } = useApp()

  const handleSignIn = () => {
    wallet.requestSignIn()
  }

  return (
    <div className="flex flex-col justify-center flex-grow relative overflow-hidden">
      <div className="relative pt-6 pb-16 sm:pb-24">
        <main className="mt-16 mx-auto max-w-5xl px-4 sm:mt-24">
          <div className="text-center">
            <h1 className="text-4xl tracking-tight font-extrabold text-gray-900 sm:text-5xl md:text-6xl">
              near-vault
            </h1>
            <p className="mt-3 max-w-md mx-auto text-base text-gray-600 sm:text-lg md:mt-5 md:text-xl md:max-w-3xl">
              Securely manage your NEP-141 token.
            </p>
            <div className="mt-5 max-w-md mx-auto flex justify-center md:mt-8">
              <button
                type="button"
                onClick={handleSignIn}
                className="flex items-center justify-center px-8 py-3 border border-gray-400 text-base font-medium rounded-md text-gray-700 bg-gray-300 neumorphic-flat hover:neumorphic-pressed focus:neumorphic-pressed focus:outline-none md:py-4 md:text-lg md:px-10"
              >
                Connect with NEAR wallet
              </button>
            </div>
          </div>
        </main>
      </div>
    </div>
  )
}

export default Welcome
