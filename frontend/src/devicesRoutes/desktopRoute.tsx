import { createBrowserRouter,Navigate,Outlet } from "react-router-dom"
import SignInPage from "../pages/mobilePages/SignIn"

import Test from "../pages/test/Test"


export const DesktopRouter = createBrowserRouter([
    {
        path:"/",
        element: <Navigate to="/test" replace/>
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