import { ButtonPresets } from "@/components/ui/button_presets"
import NavbarDesktop from "@/components/ui/navbarDesktop"
import { useCallback, useContext, useEffect, useRef, useState } from "react";
import { useNavigate } from "react-router-dom"
import useSliderThrottle from "@/lib/handleSwipe"; 
import { GetData } from "@/lib/WebSocketData";
import SocketContext from "@/lib/socket/context";
import { Slider } from "@/components/ui/slider";
import { Button as Mute } from "@/components/ui/button_mute";

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

    const handleMute=(channel: string, type: string)=>{
        if (type === "IN") {;
            const data={"section":"mute","io":"input","channel":channel,"value":(!inputChannelStates[channel]).toString()}
            socket?.send(JSON.stringify(data))
          }else if(type==="OUT"){
            const data={"section":"mute","io":"output","channel":channel,"value":(!outputChannelStates[channel]).toString()}
            socket?.send(JSON.stringify(data))
          }else if(type==="MASTER"){

            const channels = Object.keys(outputChannelStates);
            const values = Object.values(outputChannelStates);

            const data={"section":"mute","io":"output","channel":channels[0],"value":(values[0]&&values[1]) ? "false" : "true"}
            socket?.send(JSON.stringify(data))
            const data1={"section":"mute","io":"output","channel":channels[1],"value":(values[0]&&values[1]) ? "false" : "true"}
            socket?.send(JSON.stringify(data1))
          }

    }

    const sendVolumeUpdate = useCallback((channel: string, source: string, value: number) => {
          const io = source === "IN" ? "input" : "output";
          const data = {"section": "volume", "io": io, "channel": channel, "value": value.toString()};
          socket?.send(JSON.stringify(data));
        }, [socket]);
    
    const { handleSliderChange, handleSliderCommit } = useSliderThrottle(
        sendVolumeUpdate, { speedThreshold: 0.3, slowInterval: 50, fastInterval: 100, skipCount: 3  }
    );

    return(
        <div className="grid grid-cols-[100px_1fr_0.5fr] h-screen">
            <div>
                <NavbarDesktop selectedColor="speaker" />
            </div>
            <div className="flex flex-col items-end justify-center gap-8 mr-6">
                <div className="grid grid-rows-6 border-[1.5px] border-home_colors-Selected_Borders/text border-opacity-40 bg-home_colors-Navbar/Selection_Bg rounded-[60px] h-[330px] w-[600px]  px-24">
                
                </div>
                <div className="grid grid-rows-6 border-[1.5px] border-home_colors-Selected_Borders/text border-opacity-40 bg-home_colors-Navbar/Selection_Bg rounded-[60px] h-[330px] w-[600px]  px-24">
            
                </div>
            </div>
            <div className="flex flex-col gap-5 items-center justify-center">
                <ButtonPresets text={"Presets"} onClick={()=>{navigate("/presets",{state:"house"})}}/>
                <div className="flex flex-col items-center gap-3 border-[1.5px] border-home_colors-Selected_Borders/text border-opacity-40 bg-home_colors-Navbar/Selection_Bg rounded-[60px] h-[550px] w-[100px] py-20  ">
                    
                        <p className="text-home_colors-Similar_White text-sm font-bold">{outputChannelStates["1"] && outputChannelStates["2"] ? [-60] : [outputVolumesStates["2"]]} db </p>
                        <Slider orientation="vertical" className="h-full" 
                                min={-60} max={15} 
                                value={ outputChannelStates["1"] && outputChannelStates["2"] ? [-60] : [outputVolumesStates["2"]] } 
                                onValueChange={(newValue) => {handleSliderChange(newValue, "1", "OUT");
                                                            handleSliderChange(newValue, "2", "OUT");
                                                            }}
                                onValueCommit={(newValue) => {handleSliderCommit(newValue, "1", "OUT");
                                                            handleSliderCommit(newValue, "2", "OUT");
                                                            }}/> 
                        <p className="text-home_colors-Similar_White text-sm font-bold"> Master </p>
                        <div className="text-home_colors-Selected_Borders/text border-[0.9px] w-10 justify-center text-center text-sm border-home_colors-Selected_Borders/text font-bold">OUT</div>
                        <Mute size={"mute_preset"} 
                            disabled={!(outputVisibility["1"] || outputVisibility["2"]) }
                            variant={ outputVisibility["1"] || outputVisibility["2"] ?
                                    outputChannelStates["1"] && outputChannelStates["2"] ? "muted": "unmuted"
                                                                                    :"notAvailable" 
                            } 
                            onClick={()=>handleMute("","MASTER")}>
                            MUTE
                        </Mute>
                    
                </div>
            </div>
        </div>
    )
}