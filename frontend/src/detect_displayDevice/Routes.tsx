import { createBrowserRouter,Navigate,Outlet } from "react-router-dom";
import { 
      SignIn,NewConnections,CreateConnections,RecentConnections,CallAdministrator,HomeAudio,Volume,Settings,Presets,PreferenciesPresets,PreferenciesChannels,Video,PresetsCamera,Mix 
       } from "./RouterSwitchers";
import UUIDProvider from "../lib/socket/ComponentUuid";
import SocketContextComponent from "../lib/socket/Component";
import { clientLoader } from "../lib/clientLoader";
import UUIDLayout from "@/lib/Redirect";
  
  const UUIDLay = () => (
    <UUIDProvider>
      <Outlet />
    </UUIDProvider>
  );
  
  const SocketLayout = () => (
    <SocketContextComponent>
      <Outlet />
    </SocketContextComponent>
  );
  
  export const AppRouter = createBrowserRouter([
    {
      path:"/",
      element: <Navigate to="/login" replace/>
    },
    {
      path: "/login",
      element: <SignIn/> 
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
          {
            path: "/volume",
            element: <Volume/>
          },
          {
            path: "/settings",
            element: <Settings/>
          },
          {
            path: "/presets",
            element: <Presets/>
          },
          {
            path: "/preferenciesChannels",
            element: <PreferenciesChannels/>
          },
          {
            path: "/preferenciesPresets",
            element: <PreferenciesPresets/>
          },
          {
            path: "/video",
            element: <Video/>
          },
          {
            path: "/presetsCamera",
            element: <PresetsCamera/>
          },
          {
            path: "/mix",
            element: <Mix/>
          }
          
        ]
        }
      ]
      ,loader:clientLoader
      
    }
  ]);