import { useEffect } from 'react'
import { useRoutes } from 'react-router-dom'
import { routes } from './routes'
import { invoke } from '@tauri-apps/api'

function App() {
  return useRoutes(routes)
}

export default App
