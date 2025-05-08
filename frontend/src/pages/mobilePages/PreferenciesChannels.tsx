import { Clock,Eye,EyeSlash } from "@phosphor-icons/react";
import { GetData } from "@/lib/WebSocketData";
import Navbar from "@/components/ui/navbar";
import { Button as PresetsButton } from "@/components/ui/audio_video";
import { Badge } from "@/components/ui/badge";
import SocketContext from "@/lib/socket/context";
import { useNavigate } from "react-router-dom";
import {ButtonEdit} from "@/components/ui/button_edit";
import { useContext, useEffect, useState } from "react";
import InOutButton from "@/components/ui/in_out";


export const PreferenciesChannels=()=>{
    const { socket,message } = useContext(SocketContext).socketState
    const navigate=useNavigate()
    const [InOut,setInOut]=useState<"IN"|"OUT">("IN")
    const [inputVisibility, setInputVisibility] = useState<{[key: string]: boolean;}>({});
    const [outputVisibility, setOutputVisibility] = useState<{[key: string]: boolean;}>({});
    const [labelChannelInput,setLabelChannelInput]=useState<{[key: string]: string;}>({})
    const [labelChannelOutput,setLabelChannelOutput]=useState<{[key: string]: string;}>({})

    const Presets = [1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16];


    useEffect(()=>{
      const { outputVisibility, inputVisibility,device_type } = GetData(message);
      if(device_type==="matrix"){
        setInputVisibility(inputVisibility)
        setOutputVisibility(outputVisibility)
      }
      },[message])

    const handleSetNameChannel=(value:string,channel:number)=>{
      if (InOut === "IN") {
      const data={"section":"channel_labels","io":"input","channel":channel.toString(),"value":value}
      socket?.send(JSON.stringify(data))
      }else if(InOut==="OUT"){
        const data={"section":"channel_labels","io":"output","channel":channel.toString(),"value":value}
        socket?.send(JSON.stringify(data))
      }
    }

    const handleVisibility=(presets:string)=>{
        if (InOut === "IN") {
            const data={"section":"visibility","io":"input","channel":presets,"value":(!inputVisibility[presets]).toString()}
            socket?.send(JSON.stringify(data))
          }else if(InOut==="OUT"){
            const data={"section":"visibility","io":"output","channel":presets,"value":(!outputVisibility[presets]).toString()}
            socket?.send(JSON.stringify(data))
          }
    }


    return(
        <div className="grid grid-rows-[70px,1fr,auto] h-screen relative">
          <div className="flex items-center justify-center gap-3" >
                <PresetsButton  variant={"white"} className="flex flex-col gap-0 items-center justify-center text-center " onClick={()=>navigate("/preferenciesPresets")}>
                    <span>LABELS</span> 
                    <span>PRESETS</span>
                </PresetsButton>
                <PresetsButton variant={"blue"} >CHANNELS</PresetsButton>
          </div>
          <div className="flex flex-1 px-7 pb-7 overflow-hidden relative pt-5 ">
                <Badge className="absolute left-12 top-3 transform -translate-x-1/2">
                  PRESETS
                </Badge>
              <div className="flex flex-col px-6 pt-10 pb-6 w-full  bg-home_colors-Navbar/Selection_Bg rounded-2xl items-center gap-6">
                <div className="grid grid-cols-2 h-full w-full  gap-5 overflow-y-auto">
                {Presets.map((presets,index) => {
                  const right = (index+1) %2 == 0
                  return(
                        <div className="flex items-center gap-2 w-full py-1" key={presets}>
                            { !right ? <div className="flex-shrink-0 cursor-pointer" onClick={()=>handleVisibility(presets.toString())}>
                                {InOut==="IN" ? inputVisibility[presets] ? <Eye color="#FFFFFF" size={22}/> : <EyeSlash color="#FFFFFF" size={22}/>
                                                :outputVisibility[presets] ? <Eye color="#FFFFFF" size={22}/> : <EyeSlash color="#FFFFFF" size={22}/>}                          
                            </div> : null}
                            <div className="flex-1 min-w-0"> 
                                <ButtonEdit onChange={(value)=>{handleSetNameChannel(value,presets)}} 
                                            Text={InOut==="IN"?labelChannelInput[presets.toString()]:labelChannelOutput[presets.toString()]}/>
                            </div>
                            { right ? <div className="flex-shrink-0 cursor-pointer" onClick={()=>handleVisibility(presets.toString())}>
                            {InOut==="IN" ? inputVisibility[presets] ? <Eye color="#FFFFFF" size={22}/> : <EyeSlash color="#FFFFFF" size={22}/>
                                                :outputVisibility[presets] ? <Eye color="#FFFFFF" size={22}/> : <EyeSlash color="#FFFFFF" size={22}/>}
                            </div> : null}
                        </div>
                        )
                    })}

                </div>
                <div>
                    <InOutButton onChange={setInOut}/>    
                </div>
              </div>
          </div>
          <div className="flex justify-between items-center pb-3 gap-12 pt-3">
                <Navbar selectedColor={"settings"}/>
          </div>
        </div>

    )
}