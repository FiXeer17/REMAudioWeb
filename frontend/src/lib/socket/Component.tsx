import React, { PropsWithChildren, useEffect, useReducer, useState } from "react";
import { defaultSocketContextState,SocketReducer,SocketContextProvider } from "./context";
import { useUUID } from "./ComponentUuid";



export interface ISocketContextComponentProps extends PropsWithChildren{}

const SocketContextComponent: React.FunctionComponent<ISocketContextComponentProps> = (props)=>
{
    
    const { children } = props
    const [socketState, socketDispatch]=useReducer(SocketReducer,defaultSocketContextState)
    const [loading, setLoading]= useState(true)
    
    const {uuid}=useUUID()


    useEffect(()=>{

      if (!uuid) return

      const socketServerUrl = `ws://localhost:8000/ws/app?uuid=${uuid}`;  

      const socket = new WebSocket(socketServerUrl)

      socket.onopen=()=>{};
      socketDispatch({type:"update_socket",payload:socket})
      socket.onmessage=(event)=>{
        socketDispatch({ type: 'new_message', payload: event.data })
        setLoading(false)
      }
      
      StartListeners()

      SendHandshake()
      return () => {
        socket.close();
      };
    },[uuid])

    const StartListeners= ()=>{

      };
    
    const SendHandshake = ()=>{}

    if(loading) return <div>Caricamento socket in corso...</div>

    return <SocketContextProvider value={{ socketState,socketDispatch }}>
        {children}
    </SocketContextProvider>

}
export default SocketContextComponent