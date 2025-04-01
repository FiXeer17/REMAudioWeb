import React, { PropsWithChildren, useEffect, useReducer, useState } from "react";
import { defaultSocketContextState,SocketReducer,SocketContextProvider } from "./context";
import SignIn from "@/pages/SignIn";
import { getUUID } from "../services";
import axios from "axios";
import { useSocket } from "@/lib/useSocket";


export interface ISocketContextComponentProps extends PropsWithChildren{}

const SocketContextComponent: React.FunctionComponent<ISocketContextComponentProps> = (props)=>
{
    
    const { children } = props
    const [socketState, socketDispatch]=useReducer(SocketReducer,defaultSocketContextState)
    const [loading, setLoading]= useState(true)
    const [uuid, setUuid] = useState<string>()
    
  /*
    const fetchUUID = async () => {
      try {
        const value = await getUUID();
        setUuid(value.data.uuid); // Imposta il valore dell'UUID
        console.log(value.data.uuid)
      } catch (error) {
        console.error("Error fetching UUID:", error);
      }
    };
    */
   useEffect(()=>{
    const fetchUUID = async () => {
      try {
        const value = await getUUID();
        setUuid(value.data.uuid); // Imposta il valore dell'UUID
        console.log(value.data.uuid)
      } catch (error) {
        console.error("Error fetching UUID:", error);
      }
    }
    fetchUUID()
   },[])
   /*
    const fetchUUID = () => {
      getUUID()
        .then(value => {
          setUuid(value.data.uuid); // Imposta il valore dell'UUID
          console.log(value.data.uuid);
        })
        .catch(error => {
          console.error("Error fetching UUID:", error);
        });
    };
    */
    

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
        console.log("Chiusura WebSocket...");
        socket.close();
      };
    },[uuid])

    const StartListeners= ()=>{

      };
    
    const SendHandshake = ()=>{}

    if(loading) return <SignIn isLoading={true}/>

    return <SocketContextProvider value={{ socketState,socketDispatch }}>
        {children}
    </SocketContextProvider>

}
export default SocketContextComponent