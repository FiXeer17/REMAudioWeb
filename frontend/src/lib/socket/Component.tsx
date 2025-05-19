import React, { PropsWithChildren, useEffect, useReducer, useState } from "react";
import { defaultSocketContextState,SocketReducer,SocketContextProvider } from "./context";
import { useConnections } from "./ComponentUuid";
import { useNavigate } from "react-router-dom";



export interface ISocketContextComponentProps extends PropsWithChildren{}

const SocketContextComponent: React.FunctionComponent<ISocketContextComponentProps> = (props)=>
{
    const navigate=useNavigate()
    const { children } = props
    const [socketState, socketDispatch]=useReducer(SocketReducer,defaultSocketContextState)
    const {uuid,isAdmin,triggerRedirect}=useConnections()
    const host = import.meta.env.VITE_WS_HOST

    useEffect(()=>{
      
      if (!uuid) return

      const socketServerUrl = `ws://${host}/ws/app?uuid=${uuid}`;  

      const socket = new WebSocket(socketServerUrl)
      
      let manuallyClosed = false;
      let isRefreshing = false;
      let latest_matrix = false
      let latest_camera = false

      const handleBeforeUnload = () => {
        isRefreshing = true;
      };
      window.addEventListener('beforeunload', handleBeforeUnload);

      socket.onopen=()=>{};
      socketDispatch({type:"update_socket",payload:socket})
      socket.onmessage=(event)=>{
        const datajson=JSON.parse(event.data)
        if (!datajson.hasOwnProperty('reason')){
          if(datajson.device_type==="matrix"){
            socketDispatch({ type: 'new_message_matrix', payload: event.data })
            socketDispatch({ type: "matrix_status", payload: "connected" })
            latest_matrix=true
          }
          if (datajson.device_type==="camera"){
            socketDispatch({ type: 'new_message_camera', payload: event.data })
            socketDispatch({ type: "camera_status", payload: "connected" })
            latest_camera = true
          }
          
        }else{
          const reason=datajson.reason
          if(reason.includes("camera")){
            latest_camera=false
            socketDispatch({ type: "camera_status", payload: "disconnected" })
          }else if(reason.includes("matrix")){
            latest_matrix=false
            socketDispatch({ type: "matrix_status", payload: "disconnected" })
          }
          if(!latest_camera && !latest_matrix){
            if (isAdmin) {
              const handleRedirect = async () => {
                await triggerRedirect()
                navigate("/uuidprovider",{state:{show:true}})
                }
                handleRedirect()
            } else {
              navigate("/callAdministrator");
            }
          }
        }
        
      }

      socket.onclose=()=>{
        
        if (!manuallyClosed && !isRefreshing) {

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