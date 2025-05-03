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

export const Volume=()=>{
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
    const volumes=[1,2,3,4]
    const [inputVisibility, setInputVisibility] = useState<{[key: string]: boolean;}>({});
    const [outputVisibility, setOutputVisibility] = useState<{[key: string]: boolean;}>({});
    const {socket,message} = useContext(SocketContext).socketState
    const [isAvailable, setIsAvailable] = useState(true)
    const [currentPresets,setCurrentPresets]=useState(0)
    
    useEffect(()=>{
    const { inputVolumesStates, outputVolumesStates,isAvailable,outputVisibility, inputVisibility,currentPresets } = GetData(message);
        setInputVolumesStates(inputVolumesStates);
        setOutputVolumesStates(outputVolumesStates);
        setInputVisibility(inputVisibility)
        setOutputVisibility(outputVisibility)
        setIsAvailable(isAvailable)
        setCurrentPresets(currentPresets)
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
      

    return(
        <div className="grid grid-rows-[0.5fr_2fr,auto] mx-5 min-h-svh">
            <div className="flex items-center justify-center">
                <PresetButton variant={"preset"} size={"preset"} onClick={()=>{navigate("/presets",{state:"speaker"})}}>
                    PRESET
                </PresetButton>
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
                            <p className="text-home_colors-Similar_White text-sm font-bold">
                              {value ?? "-"} db
                            </p>
                            <Slider
                              orientation="vertical"
                              className="h-full"
                              min={-60}
                              max={15}
                              value={[value ?? -60]}
                            />
                            <p className="text-home_colors-Similar_White text-sm font-bold">
                              CH{key}
                            </p>
                            <InOutButton onChange={handleSourceChange} />
                            <Mute size={"mute_preset"} variant={"unmuted"}>
                              MUTE
                            </Mute>
                          </div>
                        );
                      })
                      
                }
                
                </div>
            </div>
            <div className="flex flex-col justify-end items-center gap-4 pb-3">
                <Circle size={12} color="#ffffff" />
                <Navbar selectedColor="speaker"/>
            </div>
        </div>
    )
}