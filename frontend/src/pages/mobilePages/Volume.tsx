import Navbar from "@/components/ui/navbar"
import InOutButton from "@/components/ui/in_out"
import { Button as PresetButton } from "@/components/ui/button_mute";
import { Button as Mute } from "@/components/ui/button_mute";
import { Slider } from "@/components/ui/slider";
import { Circle } from "@phosphor-icons/react";
import { SwipeVolumes } from "@/lib/swipeSliders";
import { useNavigate } from "react-router-dom";
import { useContext, useEffect, useState } from "react";
import { GetData } from "@/lib/WebSocketData";
import SocketContext from "@/lib/socket/context";
import { ButtonPresets } from "@/components/ui/button_presets";

export const Volume=()=>{
    const [inputChannelStates, setInputChannelStates] = useState<{[key: string]: boolean;}>({});
    const [outputChannelStates, setOutputChannelStates] = useState<{[key: string]: boolean;}>({});
    const [inputVolumesStates, setInputVolumesStates] = useState<{[key: string]: number;}>({});
    const [outputVolumesStates, setOutputVolumesStates] = useState<{[key: string]: number;}>({});

    const [channelSources, setChannelSources] = useState<{ [key: string]: "IN" | "OUT" }>(
        () => {
          const initial: { [key: string]: "IN" | "OUT" } = {};
          for (let i = 1; i <= 16; i++) {
            initial[i] = "IN";
          }
          return initial;
        }
      );
    const [inputVisibility, setInputVisibility] = useState<{[key: string]: boolean;}>({});
    const [outputVisibility, setOutputVisibility] = useState<{[key: string]: boolean;}>({});
    const {socket,message} = useContext(SocketContext).socketState
    const [isAvailable, setIsAvailable] = useState(true)
    const [labelPresets,setlabelPresets]=useState<{[key: string]: string;}>({})
    const [currentPresets,setCurrentPresets]=useState(0)
    
    useEffect(()=>{
    const { inputChannelStates,outputChannelStates,inputVolumesStates, outputVolumesStates,isAvailable,outputVisibility, inputVisibility,currentPresets,labelPresets } = GetData(message);
        setInputChannelStates(inputChannelStates);
        setOutputChannelStates(outputChannelStates);
        setInputVolumesStates(inputVolumesStates);
        setOutputVolumesStates(outputVolumesStates);
        setInputVisibility(inputVisibility)
        setOutputVisibility(outputVisibility)
        setIsAvailable(isAvailable)
        setCurrentPresets(currentPresets)
        setlabelPresets(labelPresets)
      },[message])
    const navigate = useNavigate()
    
    
    const {
        displayedInputVolumes:displayedInputVolumes,
        displayedOutputVolumes:displayedOutputVolumes,
        offset:Offset,
        handleTouchStart: handleInputTouchStart,
        handleTouchMove: handleInputTouchMove,
        handleTouchEnd: handleInputTouchEnd,
      } = SwipeVolumes(inputVolumesStates,outputVolumesStates);
      
    const handleMute=(channel: string, type: string)=>{
        if (type === "IN") {;
            const data={"section":"mute","io":"input","channel":channel,"value":(!inputChannelStates[channel]).toString()}
            socket?.send(JSON.stringify(data))
          }else if(type==="ON"){
            const data={"section":"mute","io":"output","channel":channel,"value":(!outputChannelStates[channel]).toString()}
            socket?.send(JSON.stringify(data))
          }
    }

    const handleSwipe=(newValue:number[], channel:string, source:string)=>{
      
      const data={"section":"volume","io":"input","channel":channel,"value": newValue[0].toString()}
      console.log(data)
      socket?.send(JSON.stringify(data))

    }

    return(
        <div className="grid grid-rows-[0.5fr_2fr,auto]  min-h-svh">
            <div className="flex items-top justify-center pt-4">
                <ButtonPresets text={labelPresets[currentPresets.toString()]} onClick={()=>{navigate("/presets",{state:"house"})}}/>
            </div>
            <div className="flex justify-center">
                <div className="flex gap-2 pb-3  "
                    style={{
                        transform: `translateX(${Offset}px)`,
                        transition: Offset === 0 ? "transform 0.3s ease" : "none",
                    }}
                    onTouchStart={handleInputTouchStart}
                    onTouchMove={handleInputTouchMove}
                    onTouchEnd={handleInputTouchEnd}
                >
                   {
                    
                    Object.entries(displayedInputVolumes).map(([key, inputValue]) => {
                        const source = channelSources[key];
                        const visibility= source ==="IN" ? inputVisibility: outputVisibility
                        const channelState= source ==="IN" ? inputChannelStates: outputChannelStates
                        const value = source === "IN"
                          ? inputValue
                          : displayedOutputVolumes[key];
                      
                        const handleSourceChange = () => {
                          setChannelSources(prev => ({
                            ...prev,
                            [key]: prev[key] === "IN" ? "OUT" : "IN"
                          }));
                        };
                      
                        return (
                          <div className="flex flex-col items-center gap-3" key={key}>
                            <p className="text-home_colors-Similar_White text-sm font-bold">{value} db </p>
                            <Slider orientation="vertical" className="h-full" disabled={channelState[key] ? true : false } min={-60} max={15} value={channelState[key] ? [-60] : [value]} onValueChange={(newValue) => handleSwipe(newValue, key, source)}/>
                            <p className="text-home_colors-Similar_White text-sm font-bold"> CH{key} </p>
                            <InOutButton onChange={handleSourceChange} />
                            <Mute size={"mute_preset"} 
                                variant={ visibility[key]? channelState[key]? "muted": "unmuted" : "notAvailable"} 
                                onClick={()=>handleMute(key,source)}>
                              MUTE
                            </Mute>
                          </div>
                        );
                      })
                      
                }
                
                </div>
            </div>
            <div className="flex flex-col justify-end items-center gap-4 pb-3 px-5">
                <Circle size={12} color="#ffffff" />
                <Navbar selectedColor="speaker"/>
            </div>
        </div>
    )
}