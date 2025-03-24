import React, { PropsWithChildren, useEffect, useReducer, useState } from "react";
import { defaultSocketContextState,SocketContextProvider,SocketReducer } from "./context";
import { useSocket } from "@/lib/useSocket";

export interface ISocketContextComponentProps extends PropsWithChildren{}

const SocketContextComponent: React.FunctionComponent<ISocketContextComponentProps> = (props)=>
{

    const { children } = props
    const [socketState, socketDispatch]=useReducer(SocketReducer,defaultSocketContextState)
    const [loading, setLoading]= useState(true)

    const socketServerUrl = "ws://5177-151-42-176-148.ngrok-free.app/ws/app";  // Cambia con l'URL del server esterno

    const token = "Bearer eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJOYXRpdmUiOnsic3ViIjozOSwic2Vzc2lvbl90eXBlIjoibmF0aXZlIn19.HG40BXb_BsUplDUEgZkpmLZNcM-o0D9YHTBJ6dKhNxM";

    const socket = useSocket('wss://5177-151-42-176-148.ngrok-free.app/ws/app',token.toString(),{
        reconnectionAttempts: 5,
        reconnectionDelay: 5000,
        autoConnect: false
    })

    useEffect(()=>{
      socket.connect();
      
      socketDispatch({ type: 'update_socket', payload: socket })

      StartListeners()

      SendHandshake()
    },[])

    const StartListeners= ()=>{
        socket.io.on("reconnect",(attempt)=>{console.info("Reconnected on attempt"+attempt)})
        socket.io.on("reconnect_attempt",(attempt)=>{console.info("Reconnection attempt"+attempt)})
        socket.io.on("reconnect_error",(error)=>{console.info("Reconnected on attempt"+error)})
        socket.io.on("reconnect_failed",()=>{console.info("Reconnection failure"),alert("Unable to reach")})
    }
    const SendHandshake = ()=>{}

    if(loading) return <p className="text-white">Loading socket</p>

    return <SocketContextProvider value={{ socketState,socketDispatch }}>
        {children}
    </SocketContextProvider>

}
export default SocketContextComponent