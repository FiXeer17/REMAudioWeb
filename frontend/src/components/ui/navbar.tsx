import { House,SpeakerHigh,VideoCamera,SlidersHorizontal } from "@phosphor-icons/react";
import { useNavigate } from "react-router-dom";

interface NavbarColor{
    selectedColor?:string
}

export default function Navbar({selectedColor}:NavbarColor){
    const navigate=useNavigate()
    return(
        <div className="flex items-center justify-around bg-home_colors-Navbar/Selection_Bg w-full mx-8 rounded-full h-16 text-center ">
            <div onClick={()=>navigate("/homeAudio")}>
            {selectedColor==="house" ? <House size={28} color="#007AFF" />
            : <House size={28} color="#FAFAFA" />}
            </div>
            <div onClick={()=>navigate("/volume")}>
            {selectedColor==="speaker" ? <SpeakerHigh size={28} color="#007AFF" />
            : <SpeakerHigh size={28} color="#FAFAFA" />}
            </div>
            <div>
            {selectedColor==="video" ? <VideoCamera size={28} color="#007AFF" />
            : <VideoCamera size={28} color="#FAFAFA" />}
            </div>
            <div onClick={()=>navigate("/settings")} >
            {selectedColor==="settings" ? <SlidersHorizontal size={28} color="#007AFF" />
            : <SlidersHorizontal size={28} color="#FAFAFA" />}
            </div>            
        </div>
    )
}
export {Navbar}
