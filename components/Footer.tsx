const Footer = () => {
  return (
    <footer className="p-4 bg-gray-300">
      <div className="rounded-md border border-gray-400 neumorphic-flat">
        <div className="text-center mx-auto px-4 py-8 sm:px-6 lg:px-8 md:flex md:items-center md:justify-between md:text-left">
          <p className="text-base text-gray-600">
            &copy; 2022 Pontakorn Prasertsuk
          </p>
          <span className="text-xs text-gray-600 md:justify-start">
            The source code for this website is available on{' '}
            <a
              href="https://github.com/peppapighs/near-vault"
              target="_blank"
              rel="noreferrer"
              className="inline-flex font-medium text-gray-800 hover:underline"
            >
              Github
            </a>
          </span>
        </div>
      </div>
    </footer>
  )
}

export default Footer
