import React from 'react'

import Navbar from 'components/Navbar'
import Footer from './Footer'

interface Props {
  children: React.ReactNode
}

const Layout = ({ children }: Props) => {
  return (
    <div className="flex flex-col min-h-screen">
      <div className="flex flex-col flex-grow min-h-screen">
        <main className="flex flex-col flex-grow bg-gray-300">
          <Navbar />
          {children}
        </main>
      </div>
      <Footer />
    </div>
  )
}

export default Layout
