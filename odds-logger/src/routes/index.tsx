import { lazy, ReactNode, Suspense } from 'react'
import { RouteObject } from 'react-router-dom'
import AppLayout from '../components/app_layout'
import MatchList from '../pages/match_list'
// lazy initialization
const BookMaker = lazy(() => import('../pages/bookmaker'))
const League = lazy(() => import('../pages/league'))
const Team = lazy(() => import('../pages/team'))
const Match = lazy(() => import('../pages/match'))

const lazyLoad = (children: ReactNode): ReactNode => {
  return <Suspense fallback={<h1>Loading...</h1>}>{children}</Suspense>
}

export const routes: RouteObject[] = [
  {
    path: '/',
    element: <AppLayout />,
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
        path: '/add_match_info',
        element: lazyLoad(<Match />),
      },
      {
        path: '/match_list',
        element: lazyLoad(<MatchList />),
      },
    ],
  },
]
