import { StrictMode } from 'react'
import { createRoot } from 'react-dom/client'
import { createBrowserRouter,Navigate,RouterProvider } from "react-router-dom"
import { loginAction } from './pages/SignIn'

import SignIn from "./pages/SignIn"
import Register from "./pages/Register"


const router = createBrowserRouter([
  {
    path:"/",
    element: <Navigate to="/login" replace/>
  },
  {
    path: "/login",
    element: <SignIn/> ,
    action: loginAction
  },
  {
    path: "/Register",
    element: <Register/>
  }
]);


createRoot(document.getElementById('root')!).render(
  <StrictMode>
    <RouterProvider router={router}/>
  </StrictMode>,
)
