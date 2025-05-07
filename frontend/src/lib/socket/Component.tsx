import React, { PropsWithChildren, useEffect, useReducer, useState } from "react";
import { defaultSocketContextState,SocketReducer,SocketContextProvider } from "./context";
import { useConnections } from "./ComponentUuid";
import { useNavigate } from "react-router-dom";
import { RecentConnections } from "@/pages/connections_socket/RecentConnections";



export interface ISocketContextComponentProps extends PropsWithChildren{}

const SocketContextComponent: React.FunctionComponent<ISocketContextComponentProps> = (props)=>
{
    const navigate=useNavigate()
    const { children } = props
    const [socketState, socketDispatch]=useReducer(SocketReducer,defaultSocketContextState)
    const [loading, setLoading]= useState(true)
    const { triggerRedirect } = useConnections();

    const {uuid,isAdmin}=useConnections()

    useEffect(()=>{
      
      if (!uuid) return

      const socketServerUrl = `ws://localhost:8000/ws/app?uuid=${uuid}`;  

      const socket = new WebSocket(socketServerUrl)
      let closedByServer = false
      let manuallyClosed = false;

      socket.onopen=()=>{};
      socketDispatch({type:"update_socket",payload:socket})
      socket.onmessage=(event)=>{
        
        const datajson=JSON.parse(event.data)
        console.log(datajson)
        if (!datajson.hasOwnProperty('reason')){
          socketDispatch({ type: 'new_message', payload: event.data })
          setLoading(false)
        }else{
          closedByServer=true
          
          if (isAdmin){
            const handleRedirect = async () => {
            await triggerRedirect()
            navigate("/uuidprovider",{state:{show:true}})
            }
            handleRedirect()
          }
          else
            navigate("/callAdministrator")
        }
      }
      socket.onclose=()=>{
        if (!closedByServer && !manuallyClosed) {
          
          localStorage.removeItem("accessToken");
          navigate("/login");
        }
        }

      return () => {
        manuallyClosed = true;
        socket.close();
      };
    },[uuid])

    
    if(loading) return <RecentConnections isLoading={true}/>

    return <SocketContextProvider value={{ socketState,socketDispatch }}>
        {children}
    </SocketContextProvider>

}
export default SocketContextComponent