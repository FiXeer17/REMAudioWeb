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
    const {uuid,sockets,isAdmin,triggerRedirect}=useConnections()
    const [ socketOpen, setSocketOpen ] = useState(false)
    const [ latest_matrix,setLatest_Matrix ]= useState(false)
    const [ latest_camera,setLatest_Camera ]= useState(false)

    useEffect(()=>{
      
      if (!uuid) return

      const socketServerUrl = `ws://localhost:8000/ws/app?uuid=${uuid}`;  

      const socket = new WebSocket(socketServerUrl)
      
      let closedByServer = false
      let manuallyClosed = false;

      socket.onopen=()=>{setSocketOpen(true)};
      socketDispatch({type:"update_socket",payload:socket})
      socket.onmessage=(event)=>{
        
        const datajson=JSON.parse(event.data)
        console.log(datajson)
        if (!datajson.hasOwnProperty('reason')){
          socketDispatch({ type: 'new_message', payload: event.data })
          
          setLoading(false)
          
        }else{
          const handleRedirect = async () => {
            await triggerRedirect()
          }
          handleRedirect()
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

    useEffect(() => {
      if (!sockets || !socketOpen) {
        if (socketOpen) {
          if (isAdmin) {
            navigate("/uuidprovider", { state: { show: true } });
          } else {
            navigate("/callAdministrator");
          }
        }
        return;
      }
    
      const hasLatestMatrix = sockets.some(socket => socket.isLatestAudio);
      const hasLatestCamera = sockets.some(socket => socket.isLatestVideo);
    
      setLatest_Matrix(hasLatestMatrix);
      setLatest_Camera(hasLatestCamera);
    
      if (!hasLatestMatrix && !hasLatestCamera) {
        if (isAdmin) {
          navigate("/uuidprovider", { state: { show: true } });
        } else {
          navigate("/callAdministrator");
        }
      }
    
    }, [sockets]);
    

    
    if(loading) return <RecentConnections isLoading={true}/>

    return <SocketContextProvider value={{ socketState,socketDispatch }}>
        {children}
    </SocketContextProvider>

}
export default SocketContextComponent