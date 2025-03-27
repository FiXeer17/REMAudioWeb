import React, { PropsWithChildren, useEffect, useReducer, useState } from "react";
import { defaultSocketContextState,SocketReducer,SocketContextProvider } from "./context";
import { useSocket } from "@/lib/useSocket";

export interface ISocketContextComponentProps extends PropsWithChildren{}

const SocketContextComponent: React.FunctionComponent<ISocketContextComponentProps> = (props)=>
{

    const { children } = props
    const [socketState, socketDispatch]=useReducer(SocketReducer,defaultSocketContextState)
    const [loading, setLoading]= useState(true)

    const socketServerUrl = "ws://localhost:8000/ws/app?uuid=d091a356-251c-487e-a665-a951bb6cc373";  // Cambia con l'URL del server esterno

    const socket = new WebSocket(socketServerUrl)

    useEffect(()=>{
      socket.onopen=()=>{
        setLoading(false)

      };
      
      socketDispatch({ type: 'update_socket', payload: socket })

      StartListeners()

      SendHandshake()
    },[])

    const StartListeners= ()=>{
      socket.onclose = () => {
        console.log("WebSocket chiuso, riconnessione...");
        socketDispatch({ type: "update_socket", payload: socket });
      };
    }
    const SendHandshake = ()=>{}

    if(loading) return <p className="text-white">Loading socket</p>

    return <SocketContextProvider value={{ socketState,socketDispatch }}>
        {children}
    </SocketContextProvider>

}
export default SocketContextComponent