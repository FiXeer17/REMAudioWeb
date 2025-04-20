import { createBrowserRouter,Navigate,Outlet } from "react-router-dom"
import { clientLoader } from '../lib/clientLoader'
import SocketContextComponent from '../lib/socket/Component'
import UUIDProvider from "../lib/socket/ComponentUuid"
import UUIDLayout from "../pages/connections_socket/Redirect"

import SignInPage from "../pages/desktopPages/SignIn"
import NewConnections from "@/pages/desktopPages/NewConnections"
import RecentConnections from "@/pages/desktopPages/RecentConnections"
import CreateConnections from "@/pages/desktopPages/CreateConnections"
import CallAdministrator from "@/pages/desktopPages/CallAdministrator"
import HomeAudio from "@/pages/desktopPages/HomeAudio"

import Test from "../pages/test/Test"

const UUIDLay=()=>{
  return(
    <UUIDProvider>
      <Outlet/>
    </UUIDProvider>
  )
}

const SocketLayout=()=>{
  return (
    <SocketContextComponent>
      <Outlet/>
    </SocketContextComponent>
  )
}



export const DesktopRouter = createBrowserRouter([
    {
        path:"/",
        element: <Navigate to="/login" replace/>
      },
      {
        path: "/login",
        element: <SignInPage/> 
      },
      {
        path: "/test",
        element: <Test/>
      },
      {
        element: <UUIDLay/>,
        children:[
          {
            path: "/uuidprovider",
            element: <UUIDLayout/>
          },
          
          {
            path: "/recentconnections",
            element: <RecentConnections/>
          },
          {
            path: "/newconnections",
            element: <NewConnections/>
          },
          {
            path: "/createconnections",
            element: <CreateConnections/>
          },
          {
            path: "/callAdministrator",
            element: <CallAdministrator/>
          },
          {
            
            element: <SocketLayout/>,
            children:[
            {
              path: "/homeAudio",
              element: <HomeAudio/>,
            },
          ]
          }
        ]
        ,loader:clientLoader
        
      }
])