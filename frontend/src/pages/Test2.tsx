import SocketContext from "@/lib/contexts/Socket/context"
import { useContext } from "react"

export default function Test2() {

    const { socket }=useContext(SocketContext).socketState

    return(
        <div>
            <p className="text-white"> ciaoooooo</p>
            <strong>{socket?.id}</strong>
        </div>
    )
}