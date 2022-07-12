import React from 'react'

import Navbar from 'components/Navbar'

interface Props {
  children: React.ReactNode
}

const Layout = (props: Props) => {
  return (
    <div className="flex flex-col min-h-screen">
      <div className="flex flex-col flex-grow min-h-screen">
        <main className="flex flex-col flex-grow bg-gray-300">
          <Navbar />
          {props.children}
        </main>
      </div>
    </div>
  )
}

export default Layout
