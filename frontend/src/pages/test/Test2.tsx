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

        <div>
            <p className="text-white"> 
                <strong>
                    {message}
                </strong>
            </p>
            
        </div>
    )
}