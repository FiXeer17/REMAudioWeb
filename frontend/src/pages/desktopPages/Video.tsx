import NavbarDesktop from "@/components/ui/navbarDesktop"
import SocketContext from "@/lib/socket/context"
import { useContext, useEffect } from "react"

export const Video=()=>{
  const {socket,message_camera} = useContext(SocketContext).socketState

  useEffect(()=>{
    if(!message_camera) return

  },[message_camera])
    return(
        <div className="grid grid-cols-[100px,1fr] h-screen">
                    <div>
                      <NavbarDesktop selectedColor="video" />
                    </div>
        </div>
    )
} 