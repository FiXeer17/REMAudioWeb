import { StrictMode } from 'react'
import { createRoot } from 'react-dom/client'
import { createBrowserRouter,Navigate,RouterProvider } from "react-router-dom"
import SocketContextComponent from './lib/contexts/Socket/Component'

import { clientLoader } from './lib/clientLoader'

import SignIn from "./pages/SignIn"
import Register from "./pages/Register"
import HomeAudio from './pages/HomeAudio'
import Test from './pages/Test'
import Test2 from './pages/Test2'
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
  },
  {
    path: "/test2",
    element: <Test2/>
  }
]);


createRoot(document.getElementById('root')!).render(
  <StrictMode>
 
      <RouterProvider router={router}/>
    
  </StrictMode>,
)
