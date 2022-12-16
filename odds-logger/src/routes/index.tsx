import { lazy, ReactNode, Suspense } from 'react'
import { RouteObject } from 'react-router-dom'
import Layout from '../components/app_layout'
// lazy initialization
const BookMaker = lazy(() => import('../pages/bookmaker'))
const League = lazy(() => import('../pages/league'))
const Team = lazy(() => import('../pages/team'))
const Odds = lazy(() => import('../pages/odds'))

const lazyLoad = (children: ReactNode): ReactNode => {
  return <Suspense fallback={<h1>Loading...</h1>}>{children}</Suspense>
}

export const routes: RouteObject[] = [
  {
    path: '/',
    element: <Layout />,
    children: [
      {
        index: true,
        element: lazyLoad(<BookMaker />),
      },
      {
        path: '/league',
        element: lazyLoad(<League />),
      },
      {
        path: '/team',
        element: lazyLoad(<Team />),
      },
      {
        path: '/logger',
        element: lazyLoad(<Odds />),
      },
    ],
  },
]
