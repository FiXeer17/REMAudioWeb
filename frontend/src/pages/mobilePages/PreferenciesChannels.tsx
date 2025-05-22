import { Eye,EyeSlash } from "@phosphor-icons/react";
import { GetData } from "@/lib/WebSocketData";
import Navbar from "@/components/ui/navbar";
import { Button as PresetsButton } from "@/components/ui/audio_video";
import SocketContext from "@/lib/socket/context";
import { useNavigate } from "react-router-dom";
import {ButtonEdit} from "@/components/ui/button_edit";
import { useContext, useEffect, useState } from "react";
import InOutButton from "@/components/ui/in_out";


export const PreferenciesChannels=()=>{
    const { socket,message_matrix } = useContext(SocketContext).socketState
    const navigate=useNavigate()
    const [InOut,setInOut]=useState<"IN"|"OUT">("IN")
    const [inputVisibility, setInputVisibility] = useState<{[key: string]: boolean;}>({});
    const [outputVisibility, setOutputVisibility] = useState<{[key: string]: boolean;}>({});
    const [labelChannelInput,setLabelChannelInput]=useState<{[key: string]: string;}>({})
    const [labelChannelOutput,setLabelChannelOutput]=useState<{[key: string]: string;}>({})

    const Presets = ["1","2","3","4","5","6","7","8"];


    useEffect(()=>{
      if (!message_matrix) return
      const { outputVisibility, inputVisibility, labelChannelsInput, labelChannelsOutput } = GetData(message_matrix);
      setInputVisibility(inputVisibility)
      setOutputVisibility(outputVisibility)
      setLabelChannelInput(labelChannelsInput)
      setLabelChannelOutput(labelChannelsOutput)

      },[message_matrix])

    const handleSetNameChannel=(value:string,channel:string)=>{
      if (InOut === "IN") {
      const data={"section":"channel_labels","io":"input","channel":channel,"value":value}
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
              <div className="flex flex-col px-6 pt-10 pb-6 w-full  bg-home_colors-Navbar/Selection_Bg rounded-2xl items-center gap-6">
                <div className="grid grid-cols-2 h-full w-full  gap-5 overflow-y-auto">
                {Presets.map((presets,index) => {

                  const right = (index+1) %2 == 0
                  return (
                    InOut === "OUT" && (index === 0 || index === 1) ? null : 
                      <div className="flex items-center gap-2 w-full py-1" key={presets}>
                        { !right && (
                          <div className="flex-shrink-0 cursor-pointer" onClick={() => handleVisibility(presets)}>
                            {InOut === "IN"
                              ? inputVisibility[presets]
                                ? <Eye color="#FFFFFF" size={22}/>
                                : <EyeSlash color="#FFFFFF" size={22}/>
                              : outputVisibility[presets]
                                ? <Eye color="#FFFFFF" size={22}/>
                                : <EyeSlash color="#FFFFFF" size={22}/>}
                          </div>
                        )}
                        <div className="flex-1 min-w-0">
                          <ButtonEdit
                            onChange={(value) => handleSetNameChannel(value, presets)}
                            Text={InOut === "IN" ? labelChannelInput[presets] : labelChannelOutput[presets]}
                          />
                        </div>
                        { right && (
                          <div className="flex-shrink-0 cursor-pointer" onClick={() => handleVisibility(presets)}>
                            {InOut === "IN"
                              ? inputVisibility[presets]
                                ? <Eye color="#FFFFFF" size={22}/>
                                : <EyeSlash color="#FFFFFF" size={22}/>
                              : outputVisibility[presets]
                                ? <Eye color="#FFFFFF" size={22}/>
                                : <EyeSlash color="#FFFFFF" size={22}/>}
                          </div>
                        )}
                      </div>
                  );
                  
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