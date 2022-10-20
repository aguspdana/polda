import type { NextPage } from 'next'
import Head from 'next/head'
import { Workspace } from 'features/workspace/components/Workspace';

const Home: NextPage = () => {
  return (
    <>
      <Head>
        <title>Polda data frame app</title>
        <meta name="description" content="Explor data with no-code data frame app" />
        <link rel="icon" href="/favicon.ico" />
      </Head>

      <Workspace />
    </>
  )
}

export default Home
