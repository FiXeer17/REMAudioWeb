import { createBrowserRouter,Navigate,Outlet } from "react-router-dom"

import { clientLoader } from './lib/clientLoader'
import SocketContextComponent from './lib/socket/Component'

import SignIn from "./pages/SignIn"
import Register from "./pages/Register"
import HomeAudio from './pages/HomeAudio'
import Test from './pages/Test'
import Test2 from './pages/Test2'
import HomeVideo from "./pages/HomeVideo"

const SocketLayout=()=>{
  return (
    <SocketContextComponent>
      <Outlet/>
    </SocketContextComponent>
  )
}


export const router = createBrowserRouter([
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
    element: <SocketLayout/>,
    children:[{
      path: "/homeAudio",
      element: <HomeAudio/>,
      //loader: clientLoader
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
    }]
  }
  
]);