import SocketContext from "@/lib/socket/context";
import { House,SpeakerHigh,Camera,SlidersHorizontal } from "@phosphor-icons/react";
import { useContext, useEffect, useState } from "react";
import { useNavigate } from "react-router-dom";

interface NavbarColor{
    selectedColor?:string
}

export default function NavbarDesktop({selectedColor}:NavbarColor){
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
        <div className="flex flex-col items-center justify-around bg-home_colors-Navbar/Selection_Bg w-full h-full text-center">
            <div onClick={hasLatestAudio ? () => navigate("/homeAudio") : undefined} className="flex select-none flex-col items-center cursor-pointer">
                <House size={40} weight="thin" color={hasLatestAudio ? (selectedColor === "house" ? "#007AFF" : "#FAFAFA") : "#A1A1AA"} />
                <p className={`text-sm ${selectedColor === "house" ? "text-home_colors-Selected_Borders/text" : hasLatestAudio ? "text-home_colors-Similar_White" : "text-zinc-400"}`}>
                Home
                </p>
            </div>

            <div onClick={hasLatestAudio ? () => navigate("/volume") : undefined} className="flex flex-col select-none items-center cursor-pointer">
                <SpeakerHigh size={40} weight="thin" color={hasLatestAudio ? (selectedColor === "speaker" ? "#007AFF" : "#FAFAFA") : "#A1A1AA"} />
                <p className={`text-sm ${selectedColor === "speaker" ? "text-home_colors-Selected_Borders/text" : hasLatestAudio ? "text-home_colors-Similar_White" : "text-zinc-400"}`}>
                Audio
                </p>
            </div>

            <div onClick={hasLatestVideo ? () => navigate("/video") : undefined} className="flex flex-col select-none items-center cursor-pointer">
                <Camera size={40} weight="thin" color={hasLatestVideo ? (selectedColor === "video" ? "#007AFF" : "#FAFAFA") : "#A1A1AA"} />
                <p className={`text-sm ${selectedColor === "video" ? "text-home_colors-Selected_Borders/text" : hasLatestVideo ? "text-home_colors-Similar_White" : "text-zinc-400"}`}>
                Video
                </p>
            </div>

            <div onClick={() => navigate("/settings")} className="flex flex-col items-center select-none cursor-pointer">
                <SlidersHorizontal size={40} weight="thin" color={selectedColor === "settings" ? "#007AFF" : "#FAFAFA"} />
                <p className={`text-sm ${selectedColor === "settings" ? "text-home_colors-Selected_Borders/text" : "text-home_colors-Similar_White"}`}>
                Settings
                </p>
            </div>
        </div>

    )
}
export {NavbarDesktop}
