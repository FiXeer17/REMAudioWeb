import SocketContext from "@/lib/socket/context"
import { useContext, useEffect, useState } from "react"

export default function Test2() {

    const {socket}=useContext(SocketContext).socketState
    const [message, setMessage]=useState("")
    
    useEffect(() => {
        if (!socket) return;
    
        socket.onmessage = (event) => {
          setMessage(event.data);
        };
    
      });

    return(

        <div className="absolute inset-0 backdrop-blur-sm flex justify-center items-center  bg-black/30 z-30">
                   <div className="w-10 h-10 border-4  border-white border-t-transparent rounded-full animate-spin"></div>
                </div>
    )
}