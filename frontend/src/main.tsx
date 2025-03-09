import { StrictMode } from 'react'
import { createRoot } from 'react-dom/client'
import { createBrowserRouter,Navigate,RouterProvider } from "react-router-dom"

import { clientLoader } from './lib/clientLoader'

import SignIn from "./pages/SignIn"
import Register from "./pages/Register"
import HomeAudio from './pages/HomeAudio'
import Test from './pages/Test'
import HomeVideo from "./pages/HomeVideo"


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
    path: "/homeAudio",
    element: <HomeAudio/>
  },
  {
    path: "/homeVideo",
    element: <HomeVideo/>
  },
  {
    path: "/test",
    element: <Test/>
  }
]);


createRoot(document.getElementById('root')!).render(
  <StrictMode>
    <RouterProvider router={router}/>
  </StrictMode>,
)
