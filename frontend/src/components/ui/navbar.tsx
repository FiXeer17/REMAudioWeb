import SocketContext from "@/lib/socket/context";
import { House,SpeakerHigh,VideoCamera,SlidersHorizontal } from "@phosphor-icons/react";
import { useContext, useEffect, useState } from "react";
import { useNavigate } from "react-router-dom";

interface NavbarColor{
    selectedColor?:string
}

export default function Navbar({selectedColor}:NavbarColor){
    const navigate=useNavigate()
    const {camera_status,matrix_status}= useContext(SocketContext).socketState
    const [ hasLatestAudio,setHasLatestAudio ] = useState(false)
    const [ hasLatestVideo,setHasLatestVideo ] = useState(false)

    useEffect(()=>{
        if(camera_status==="connected"){
            setHasLatestVideo(true)
        }else if(camera_status==="disconnected"){
            setHasLatestVideo(false)
        }
    },[camera_status])
    useEffect(()=>{
        if(matrix_status==="connected"){
            setHasLatestAudio(true)
        }else if(matrix_status==="disconnected"){
            setHasLatestAudio(false)
        }
    },[matrix_status])

    
    return(
        <div className="flex items-center justify-around bg-home_colors-Navbar/Selection_Bg w-full mx-5 rounded-full h-16 text-center ">
            <div onClick={hasLatestAudio ? () => navigate("/homeAudio") : undefined} className="cursor-pointer">
            { hasLatestAudio ? selectedColor==="house" ? <House size={28} color="#007AFF" />
                : <House size={28} color="#FAFAFA"/>
                : <House size={28} color="#A1A1AA"/>}
            </div>
            <div onClick={hasLatestAudio ? () => navigate("/volume") : undefined} className="cursor-pointer">
            { hasLatestAudio ? selectedColor==="speaker" ? <SpeakerHigh size={28} color="#007AFF" />
                : <SpeakerHigh size={28} color="#FAFAFA"/>
                : <SpeakerHigh size={28} color="#A1A1AA"/>}
            </div>
            <div onClick={hasLatestVideo ? () => navigate("/video") : undefined}  className="cursor-pointer">
            { hasLatestVideo ? selectedColor==="video" ? <VideoCamera size={28} color="#007AFF" />
                : <VideoCamera size={28} color="#FAFAFA"/>
                : <VideoCamera size={28} color="#A1A1AA"/>}
            </div>
            <div onClick={()=>navigate("/settings")} className="cursor-pointer">
            {selectedColor==="settings" ? <SlidersHorizontal size={28} color="#007AFF" />
            : <SlidersHorizontal size={28} color="#FAFAFA" />}
            </div>            
        </div>
    )
}
export {Navbar}
