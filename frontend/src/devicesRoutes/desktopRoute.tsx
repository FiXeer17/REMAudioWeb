import { createBrowserRouter,Navigate,Outlet } from "react-router-dom"
import SignInPage from "../pages/desktopPages/SignIn"
import NewConnections from "@/pages/desktopPages/NewConnections"
import RecentConnections from "@/pages/desktopPages/RecentConnections"
import CreateConnections from "@/pages/desktopPages/CreateConnections"

import Test from "../pages/test/Test"


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
        path: "/newconnections",
        element: <NewConnections/>
      },
      {
        path: "/recentconnections",
        element: <RecentConnections/>
      },
      {
        path: "/createconnections",
        element: <CreateConnections/>
      },
])