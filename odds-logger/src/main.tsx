import ReactDOM from 'react-dom/client'
import App from './App'
import { BrowserRouter } from 'react-router-dom'

ReactDOM.createRoot(document.getElementById('root') as HTMLElement).render(
  // React.StrictMode make useEffect run twice
  <BrowserRouter>
    <App />
  </BrowserRouter>
  //   <React.StrictMode>
  //   <BrowserRouter>
  //     <App />
  //   </BrowserRouter>
  // </React.StrictMode>
)
