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
import { RecentConnections } from "../connections_socket/RecentConnections";
import { Clock } from "@phosphor-icons/react";

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
    const {socket,message_matrix,matrix_status,camera_status} = useContext(SocketContext).socketState
    const [isAvailable, setIsAvailable] = useState(true)
    const [labelPresets,setlabelPresets]=useState<{[key: string]: string;}>({})
    const [labelChannelInput,setLabelChannelInput]=useState<{[key: string]: string;}>({})
    const [labelChannelOutput,setLabelChannelOutput]=useState<{[key: string]: string;}>({})
    const [currentPresets,setCurrentPresets]=useState(0)

    useEffect(()=>{
      if(matrix_status==="disconnected" && camera_status==="connected")
        navigate("/video")
    },[matrix_status])
    
    useEffect(()=>{
      if (!message_matrix) return
      const { inputChannelStates,outputChannelStates,inputVolumesStates, outputVolumesStates,isAvailable,outputVisibility, inputVisibility,currentPresets,labelPresets,labelChannelsInput,labelChannelsOutput } = GetData(message_matrix);

      setInputChannelStates(inputChannelStates);
      setOutputChannelStates(outputChannelStates);
      setInputVolumesStates(inputVolumesStates);
      setOutputVolumesStates(outputVolumesStates);
      setInputVisibility(inputVisibility)
      setOutputVisibility(outputVisibility)
      setIsAvailable(isAvailable)
      setCurrentPresets(currentPresets)
      setlabelPresets(labelPresets)
      setLabelChannelInput(labelChannelsInput)
      setLabelChannelOutput(labelChannelsOutput)
      
      },[message_matrix])
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
      <>
        {isAvailable ? ( message_matrix ?
            <div className="absolute inset-0 z-10"></div>:<RecentConnections isLoading={true}/> ) :
      <div className="absolute inset-0 backdrop-blur-sm flex justify-center items-center  bg-black/30 z-30">
        <div className="flex border-yellow-500 border-2 rounded-sm px-3 py-3 text-yellow-500 text-sm font-bold gap-2 ">
          <div className="mt-1">
            <Clock weight="bold"></Clock>
          </div>
          <div>
          <p>Matrix Unvailable</p>
          <p>Please wait...</p>
          </div>
        </div>
      </div>}
    
      <div className="absolute inset-0 bg-black z-20">
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
                              <div className="text-home_colors-Similar_White text-sm font-bold text-center w-14">
                                {channelSources[key]==="IN"?
                                labelChannelInput[key] ? (
                                  labelChannelInput[key].length > 3 ? (
                                    <div className="relative w-full overflow-hidden">
                                      <div className="whitespace-nowrap animate-marquee">
                                        {labelChannelInput[key]}
                                      </div>
                                    </div>
                                  ) : (
                                    labelChannelInput[key]
                                  )
                                ) : null
                                  :    
                                  labelChannelOutput[key] ? (
                                    labelChannelOutput[key].length > 3 ? (
                                      <div className="relative w-full overflow-hidden">
                                        <div className="whitespace-nowrap animate-marquee">
                                          {labelChannelOutput[key]}
                                        </div>
                                      </div>
                                    ) : (
                                      labelChannelOutput[key]
                                    )
                                  ) : null
                                  }
                              </div>
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
        </div>
      </>
    )
}