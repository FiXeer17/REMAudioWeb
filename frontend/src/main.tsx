import { StrictMode } from 'react'
import { createRoot } from 'react-dom/client'
import { createBrowserRouter,Navigate,RouterProvider } from "react-router-dom"

import { clientLoader } from './lib/clientLoader'

import SignIn from "./pages/SignIn"
import Register from "./pages/Register"
import Volume from './pages/Volume'


const router = createBrowserRouter([
  {
    path:"/",
    element: <Navigate to="/login" replace/>
  },
  {
    path: "/login",
    element: <SignIn/> 
  },
  {
    path: "/register",
    element: <Register/>
  },
  {
    path: "/volume",
    element: <Volume/>,
    loader: clientLoader
  }
]);


createRoot(document.getElementById('root')!).render(
  <StrictMode>
    <RouterProvider router={router}/>
  </StrictMode>,
)
