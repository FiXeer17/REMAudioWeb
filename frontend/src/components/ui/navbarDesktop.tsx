import { House,SpeakerHigh,VideoCamera,SlidersHorizontal } from "@phosphor-icons/react";
import { useNavigate } from "react-router-dom";

interface NavbarColor{
    selectedColor?:string
}

export default function NavbarDesktop({selectedColor}:NavbarColor){
    const navigate=useNavigate()
    return(
        <div className="flex flex-col items-center justify-around bg-home_colors-Navbar/Selection_Bg w-full h-full  text-center ">
            <div onClick={()=>navigate("/homeAudio")} className="flex flex-col items-center cursor-pointer">
            {selectedColor==="house" ? <House size={40} weight="thin" color="#007AFF" />
            : <House size={40} weight="thin" color="#FAFAFA" />}
            {selectedColor==="house" ?<p className="text-home_colors-Selected_Borders/text text-sm">Home</p>
            :<p className="text-home_colors-Similar_White text-sm">Home</p>}
            </div>
            <div onClick={()=>navigate("/volume")} className="flex flex-col items-center cursor-pointer">
            {selectedColor==="speaker" ? <SpeakerHigh size={40} weight="thin" color="#007AFF" />
            : <SpeakerHigh size={40} weight="thin" color="#FAFAFA" />}
            {selectedColor==="speaker" ?<p className="text-home_colors-Selected_Borders/text text-sm">Audio</p>
            :<p className="text-home_colors-Similar_White text-sm">Audio</p>}
            </div>
            <div onClick={()=>navigate("/video")} className="flex flex-col items-center cursor-pointer">
            {selectedColor==="video" ? <VideoCamera size={40} weight="thin" color="#007AFF" />
            : <VideoCamera size={40} weight="thin" color="#FAFAFA" />}
            {selectedColor==="video" ?<p className="text-home_colors-Selected_Borders/text text-sm">Audio</p>
            :<p className="text-home_colors-Similar_White text-sm">Video</p>}
            </div>
            <div onClick={()=>navigate("/settings")} className="flex flex-col items-center cursor-pointer">
            {selectedColor==="settings" ? <SlidersHorizontal size={40} weight="thin" color="#007AFF" />
            : <SlidersHorizontal size={40} weight="thin" color="#FAFAFA" />}
            {selectedColor==="settings" ?<p className="text-home_colors-Selected_Borders/text text-sm">Audio</p>
            :<p className="text-home_colors-Similar_White text-sm">Settings</p>}
            </div>            
        </div>
    )
}
export {NavbarDesktop}
