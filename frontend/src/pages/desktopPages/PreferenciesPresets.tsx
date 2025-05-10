import NavbarDesktop from "@/components/ui/navbarDesktop"
import { GetData } from "@/lib/WebSocketData";
import { Button as PresetsButton } from "@/components/ui/audio_video";
import { Badge } from "@/components/ui/badge";
import SocketContext from "@/lib/socket/context";
import { useLocation, useNavigate } from "react-router-dom";
import {ButtonEdit} from "@/components/ui/button_edit";
import { useContext, useEffect, useState } from "react";

export const PreferenciesPresets=()=>{
    const navigate=useNavigate()
    const {socket,message_matrix} = useContext(SocketContext).socketState
    const [labelPresets,setlabelPresets]=useState<{[key: string]: string;}>({})

    useEffect(()=>{
        if (!message_matrix) return
        const { labelPresets} = GetData(message_matrix);
        setlabelPresets(labelPresets)
      },[message_matrix])

      const handleSetNamePreset=(value:string,Preset:string)=>{
        const dataoutput={"section":"preset_labels","index":Preset,"value":value}
        socket?.send(JSON.stringify(dataoutput))
      }

    return(
        <div className="grid grid-cols-[100px,1fr] h-screen">
            <div>
                <NavbarDesktop selectedColor="settings" />
            </div>
            <div className="flex items-center justify-center w-full">
                <div className="flex flex-col border-[1.5px] border-home_colors-Selected_Borders/text border-opacity-40 bg-home_colors-Navbar/Selection_Bg rounded-[60px] h-[600px] w-[500px]  px-10 py-10 ">
                    <div className="flex items-center justify-center gap-3" >
                        <PresetsButton  variant={"blue"} className="flex flex-col gap-0 items-center justify-center text-center ">
                            <span>LABELS</span> 
                            <span>PRESETS</span>
                        </PresetsButton>
                        <PresetsButton variant={"white"} onClick={()=>navigate("/preferenciesChannels")}>CHANNELS</PresetsButton>
                    </div>
                    <div className="flex flex-1 px-7 pb-5 overflow-hidden relative pt-5">
                        <div className="grid grid-cols-2 h-full w-full bg-home_colors-Navbar/Selection_Bg rounded-2xl px-10 py-10 gap-5 overflow-y-auto">
                            {Object.entries(labelPresets).map(([key,Presets])=>(
                                <ButtonEdit  key={key} onChange={(value)=>{handleSetNamePreset(value,key)}} Text={Presets}/>
                                ))}
                        </div>
                    </div>
                </div>
            </div>
        </div>
    )
}