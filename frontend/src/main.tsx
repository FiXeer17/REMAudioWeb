import { StrictMode } from 'react'
import { createRoot } from 'react-dom/client'
import { createBrowserRouter,RouterProvider } from "react-router-dom"

import SignIn from "./pages/SignIn"



const router = createBrowserRouter([
  {
    path: "/login",
    element: <SignIn/> 
  }
]);


createRoot(document.getElementById('root')!).render(
  <StrictMode>
    <RouterProvider router={router}/>
  </StrictMode>,
)
