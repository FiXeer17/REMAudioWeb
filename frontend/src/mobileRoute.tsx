import { createBrowserRouter,Navigate,Outlet } from "react-router-dom"

import { clientLoader } from './lib/clientLoader'
import SocketContextComponent from './lib/socket/Component'
import UUIDProvider from "./lib/socket/ComponentUuid"

import SignIn from "./pages/SignIn"
import HomeAudio from './pages/HomeAudio'
import Test from './pages/test/Test'
import Test2 from './pages/test/Test2'
import HomeVideo from "./pages/HomeVideo"
import NewConnetions from "./pages/connections_socket/NewConnections"
import CreateConnections from "./pages/connections_socket/CreateConnections"
import RecentConnections from "./pages/connections_socket/RecentConnections"


const UUIDLayout=()=>{
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


export const router = createBrowserRouter([
  {
    path:"/",
    element: <Navigate to="/login" replace/>
  },
  {
    path: "/login",
    element: <SignIn isLoading={false}/> 
  },
  {
    path: "/test",
    element: <Test/>
  },
  
  {
    element: <UUIDLayout/>,
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
        element: <NewConnetions/>
      },
      {
        path: "/createconnections",
        element: <CreateConnections/>
      },
      {
        
        element: <SocketLayout/>,
        children:[
        {
          path: "/homeAudio",
          element: <HomeAudio/>,
          //loader: clientLoader
        },
        {
          path: "/homeVideo",
          element: <HomeVideo/>
        },
        {
          path: "/test2",
          element: <Test2/>
        }]
      }
    ]
    
  }
]);