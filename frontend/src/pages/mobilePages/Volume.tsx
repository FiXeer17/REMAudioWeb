import Navbar from "@/components/ui/navbar"
import InOutButton from "@/components/ui/in_out"
import { Button as Mute } from "@/components/ui/button_mute";
import { Slider } from "@/components/ui/slider";
import { useNavigate } from "react-router-dom";
import { useCallback, useContext, useEffect, useRef, useState } from "react";
import { GetData } from "@/lib/WebSocketData";
import SocketContext from "@/lib/socket/context";
import { ButtonPresets } from "@/components/ui/button_presets";
import useSliderThrottle from "@/lib/handleSwipe"; 

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
    const [matrixMessage,setMatrixMessage]=useState()

    
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
        <div className="grid grid-rows-[0.5fr_2fr,auto]  min-h-svh">
            <div className="flex items-top justify-center pt-4">
                <ButtonPresets text={labelPresets[currentPresets.toString()]} onClick={()=>{navigate("/presets",{state:"house"})}}/>
            </div>
            <div className="flex justify-center overflow-hidden items-center mx-5">
                <div className="flex gap-3 pb-3 overflow-x-auto h-[400px] ">
                  <div className="flex flex-col items-center gap-3 pr-3 border-r-2">
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
                   {
                    
                    Object.entries(inputVolumesStates).map(([key]) => {

                        const source = channelSources[key];
                        const visibility= source ==="IN" ? inputVisibility: outputVisibility
                        const channelState= source ==="IN" ? inputChannelStates: outputChannelStates
                        
                        const value = source === "IN"
                          ? inputVolumesStates[key]
                          : outputVolumesStates[key];
                        const handleSourceChange = () => {
                          setChannelSources(prev => ({
                            ...prev,
                            [key]: prev[key] === "IN" ? "OUT" : "IN"
                          }));
                        };
                      
                        return (
                          <div className="flex flex-col items-center gap-3" key={key}>
                            <p className="text-home_colors-Similar_White text-sm font-bold">{value} db </p>
                            <Slider orientation="vertical" className="h-full" 
                                disabled={visibility[key] ? channelState[key] : false} min={-60} max={15} 
                                value={visibility[key] ? channelState[key] ? [-60] : [value] : [-60]} 
                                onValueChange={(newValue) => handleSliderChange(newValue, key, source)} 
                                onValueCommit={(newValue) => handleSliderCommit(newValue, key, source)}/> 
                            <p className="text-home_colors-Similar_White text-sm font-bold"> CH{key} </p>
                            { key==="1"||key==="2" ? 
                                                    <div className="text-home_colors-Selected_Borders/text border-[0.9px] w-10 justify-center text-center text-sm border-home_colors-Selected_Borders/text font-bold">
                                                      IN
                                                    </div> : <InOutButton onChange={handleSourceChange} />}
                            <Mute size={"mute_preset"} 
                                disabled={!visibility[key]}
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
            <div className="flex flex-col justify-end items-center gap-4 pt-5 pb-3 px-5">
                <Navbar selectedColor="speaker"/>
            </div>
        </div>
    )
}