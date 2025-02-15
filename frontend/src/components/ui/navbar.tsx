import exp from "constants"
import * as React from "react"
import { House,SpeakerHigh,VideoCamera,SlidersHorizontal } from "@phosphor-icons/react";
import { useState } from "react";

export default function Navbar(){
    const [color,setColor] = useState("house")
    return(
        <div className="flex items-center justify-around bg-home_colors-Navbar/Selection_Bg max-w-screen mx-9 rounded-full h-16 text-center ">
            <div>
            {color==="house" ? <House size={28} color="#007AFF" />
            : <House size={28} color="#FAFAFA" onClick={()=>setColor("house")}/>}
            </div>
            <div>
            {color==="speaker" ? <SpeakerHigh size={28} color="#007AFF" />
            : <SpeakerHigh size={28} color="#FAFAFA" onClick={()=>setColor("speaker")}/>}
            </div>
            <div>
            {color==="video" ? <VideoCamera size={28} color="#007AFF" />
            : <VideoCamera size={28} color="#FAFAFA" onClick={()=>setColor("video")}/>}
            </div>
            <div>
            {color==="settings" ? <SlidersHorizontal size={28} color="#007AFF" />
            : <SlidersHorizontal size={28} color="#FAFAFA" onClick={()=>setColor("settings")}/>}
            </div>
            
        </div>
    )
}
export {Navbar}
//bg-home_colors-Navbar/Selection_Bg