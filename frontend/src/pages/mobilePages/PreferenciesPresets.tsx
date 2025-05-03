import { Clock } from "@phosphor-icons/react";
import { useContext, useEffect, useState } from "react";
import { GetData } from "@/lib/WebSocketData";
import Navbar from "@/components/ui/navbar";
import { Button as PresetsButton } from "@/components/ui/audio_video";
import { Badge } from "@/components/ui/badge";
import SocketContext from "@/lib/socket/context";
import { useLocation, useNavigate } from "react-router-dom";
import {ButtonEdit} from "@/components/ui/button_edit";

export const PreferenciesPresets = ()=>{
    const navigate=useNavigate()
    const location=useLocation()
    //const {socket,message} = useContext(SocketContext).socketState
    const Presets = [1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16];

    return(
        <div className="grid grid-rows-[70px,1fr,auto] h-screen relative">
          <div className="flex items-center justify-center gap-3" >
                <PresetsButton  variant={"blue"} className="flex flex-col gap-0 items-center justify-center text-center ">
                    <span>LABELS</span> 
                    <span>PRESETS</span>
                </PresetsButton>
                <PresetsButton variant={"white"}>CHANNELS</PresetsButton>
          </div>
          <div className="flex flex-1 px-7 pb-5 overflow-hidden relative pt-5">
                <Badge className="absolute left-12 top-3 transform -translate-x-1/2">
                  PRESETS
                </Badge>
              <div className="grid grid-cols-2 h-full w-full bg-home_colors-Navbar/Selection_Bg rounded-2xl px-10 py-10 gap-5 overflow-y-auto">
                  {Presets.map((presets)=>(
                    <ButtonEdit key={presets}/>
                  ))}
              </div>
          </div>
          <div className="flex justify-between items-center pb-3 gap-12 pt-3">
                <Navbar selectedColor={"settings"}/>
          </div>
        </div>
    )
}