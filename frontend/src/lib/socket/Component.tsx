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
    const {uuid,isAdmin}=useConnections()
    

    useEffect(()=>{
      
      if (!uuid) return

      const socketServerUrl = `ws://172.20.10.11/ws/app?uuid=${uuid}`;  

      const socket = new WebSocket(socketServerUrl)
      
      let closedByServer = false
      let manuallyClosed = false;
      let latest_matrix = false
      let latest_camera = false

      socket.onopen=()=>{};
      socketDispatch({type:"update_socket",payload:socket})
      socket.onmessage=(event)=>{
        const datajson=JSON.parse(event.data)
        if (!datajson.hasOwnProperty('reason')){
          if(datajson.device_type==="matrix"){
            socketDispatch({ type: 'new_message_matrix', payload: event.data })
            latest_matrix=true
          }
          if (datajson.device_type==="camera"){
            socketDispatch({ type: 'new_message_camera', payload: event.data })
            latest_camera = true
          }
          setLoading(false)
          
          
        }else{
          const reason=datajson.reason
          if(reason.includes("camera")){
            latest_camera=false
            socketDispatch({ type: "device_disconnected", payload: "camera" })
          }else if(reason.includes("matrix")){
            latest_matrix=false
            socketDispatch({ type: "device_disconnected", payload: "matrix" })
          }
          if(!latest_camera && !latest_matrix){
            if (isAdmin) {
              navigate("/uuidprovider", { state: { show: true } });
            } else {
              navigate("/callAdministrator");
            }
          }
        }
        
      }
      socket.onclose=()=>{
        
        if (!manuallyClosed && !closedByServer) {
          localStorage.removeItem("accessToken");
          navigate("/login");
        }
        }

      return () => {
        manuallyClosed = true;
        socket.close();
      };
    },[uuid])

    return <SocketContextProvider value={{ socketState,socketDispatch }}>
        {children}
    </SocketContextProvider>

}
export default SocketContextComponent