import { useConnections } from "@/lib/socket/ComponentUuid";
import { House,SpeakerHigh,VideoCamera,SlidersHorizontal } from "@phosphor-icons/react";
import { useEffect } from "react";
import { useNavigate } from "react-router-dom";

interface NavbarColor{
    selectedColor?:string
}

export default function Navbar({selectedColor}:NavbarColor){
    const navigate=useNavigate()
    const {sockets}=useConnections()
    
    return(
        <div className="flex items-center justify-around bg-home_colors-Navbar/Selection_Bg w-full mx-5 rounded-full h-16 text-center ">
            <div onClick={()=>navigate("/homeAudio")} className="cursor-pointer">
            {selectedColor==="house" ? <House size={28} color="#007AFF" />
            : <House size={28} color="#FAFAFA" />}
            </div>
            <div onClick={()=>navigate("/volume")} className="cursor-pointer">
            {selectedColor==="speaker" ? <SpeakerHigh size={28} color="#007AFF" />
            : <SpeakerHigh size={28} color="#FAFAFA" />}
            </div>
            <div className="cursor-pointer">
            {selectedColor==="video" ? <VideoCamera size={28} color="#007AFF" />
            : <VideoCamera size={28} color="#FAFAFA" />}
            </div>
            <div onClick={()=>navigate("/settings")} className="cursor-pointer">
            {selectedColor==="settings" ? <SlidersHorizontal size={28} color="#007AFF" />
            : <SlidersHorizontal size={28} color="#FAFAFA" />}
            </div>            
        </div>
    )
}
export {Navbar}
