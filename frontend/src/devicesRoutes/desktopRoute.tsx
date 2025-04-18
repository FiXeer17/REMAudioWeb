import { createBrowserRouter,Navigate,Outlet } from "react-router-dom"
import SignInPage from "../pages/desktopPages/SignIn"

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
])