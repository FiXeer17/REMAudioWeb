import { StrictMode } from 'react'
import { createRoot } from 'react-dom/client'
import { createBrowserRouter,RouterProvider } from "react-router-dom"

import LoginPage from "./pages/login"

import './index.css'


const router = createBrowserRouter([
  {
    path: "/login",
    element: <LoginPage/>
  }
]);


createRoot(document.getElementById('root')!).render(
  <StrictMode>
    <RouterProvider router={router}/>
  </StrictMode>,
)
