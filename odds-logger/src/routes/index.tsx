import { lazy, ReactNode, Suspense } from 'react'
import { RouteObject } from 'react-router-dom'
import AppLayout from '../components/app_layout'
// lazy initialization
const BookMaker = lazy(() => import('../pages/bookmaker'))
const BookMakerUpdate = lazy(() => import('../pages/bookmaker_update'))
const League = lazy(() => import('../pages/league'))
const LeagueUpdate = lazy(() => import('../pages/league_update'))
const Team = lazy(() => import('../pages/team'))
const TeamUpdate = lazy(() => import('../pages/team_update'))
const Match = lazy(() => import('../pages/match'))
const MatchQuery = lazy(() => import('../pages/match_query'))
const MatchUpdate = lazy(() => import('../pages/match_update'))

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
        element: lazyLoad(<MatchQuery />),
      },
      {
        path: '/matchDetail/:id',
        element: lazyLoad(<MatchUpdate />),
      },
      {
        path: '/add_match_info',
        element: lazyLoad(<Match />),
      },
      {
        path: '/league',
        element: lazyLoad(<League />),
      },
      {
        path: '/league/:id',
        element: lazyLoad(<LeagueUpdate />),
      },
      {
        path: '/bookmaker',
        element: lazyLoad(<BookMaker />),
      },
      {
        path: '/bookmaker/:id',
        element: lazyLoad(<BookMakerUpdate />),
      },
      {
        path: '/team',
        element: lazyLoad(<Team />),
      },
      {
        path: '/team/:id',
        element: lazyLoad(<TeamUpdate />),
      },
    ],
  },
]
