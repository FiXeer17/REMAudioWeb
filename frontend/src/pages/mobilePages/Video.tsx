import Navbar from "@/components/ui/navbar"
import SocketContext from "@/lib/socket/context";
import { GetData } from "@/lib/WebSocketData";
import { useContext, useEffect } from "react";


export const Video=()=>{
    const {socket,message_camera} = useContext(SocketContext).socketState

    useEffect(()=>{
        if(!message_camera) return

    },[message_camera])


    return(
        <div>
            <div className="flex flex-col justify-between items-center pb-3 gap-12 pt-3 px-5 w-full">
            <Navbar selectedColor="video"/>
            </div>
        </div>
    )
} 