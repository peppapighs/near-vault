import { Html, Head as NextHead, Main, NextScript } from 'next/document'

import Head from 'components/Head'

const Document = () => {
  return (
    <Html lang="en">
      <NextHead>
        <Head />
      </NextHead>
      <body>
        <Main />
        <NextScript />
      </body>
    </Html>
  )
}

export default Document
