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

    const [inputVisibility, setInputVisibility] = useState<{[key: string]: boolean;}>({});
    const [outputVisibility, setOutputVisibility] = useState<{[key: string]: boolean;}>({});

    const {socket,message_matrix} = useContext(SocketContext).socketState
    const [isAvailable, setIsAvailable] = useState(true)

    const [labelChannelsInput,setlabelChannelInput]=useState<{[key: string]: string;}>({})
    const [labelChannelsOutput,setlabelChannelOutput]=useState<{[key: string]: string;}>({})
    const [labelPresets,setlabelPresets]=useState<{[key: string]: string;}>({})
    const [currentPresets,setCurrentPresets]=useState(0)

    
    useEffect(()=>{
      const { inputChannelStates,outputChannelStates,inputVolumesStates, outputVolumesStates,isAvailable,outputVisibility, inputVisibility,currentPresets,labelPresets } = GetData(message_matrix);

      setInputChannelStates(inputChannelStates);
      setOutputChannelStates(outputChannelStates);
      setInputVolumesStates(inputVolumesStates);
      setOutputVolumesStates(outputVolumesStates);
      setInputVisibility(inputVisibility)
      setOutputVisibility(outputVisibility)
      setIsAvailable(isAvailable)
      setCurrentPresets(currentPresets)
      setlabelPresets(labelPresets)
      
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
        <div className="grid grid-cols-[100px_1fr_0.5fr] h-screen">
            <div>
                <NavbarDesktop selectedColor="speaker" />
            </div>
            <div className="flex flex-col items-end justify-center gap-8 mr-6">
                <div className="flex border-[1.5px] border-home_colors-Selected_Borders/text border-opacity-40 bg-home_colors-Navbar/Selection_Bg rounded-[60px] h-[330px] w-[600px] overflow-hidden px-24  ">
                  <div className="flex gap-3 pb-3 overflow-x-auto ">
                    {Object.entries(inputVolumesStates).map(([key])=>{

                            return(
                              
                              <div className="flex flex-col items-center justify-center gap-3">
                                <p className="text-home_colors-Similar_White text-sm font-bold">{ inputVolumesStates[key] } db </p>
                                <Slider orientation="vertical" className="h-full" 
                                        disabled={inputVisibility[key] ? inputChannelStates[key] : false} min={-60} max={15} 
                                        value={ inputVisibility[key] ? inputChannelStates[key] ? [-60] : [inputVolumesStates[key]] : [-60] } 
                                        onValueChange={(newValue) => handleSliderChange(newValue, key, "IN")} 
                                        onValueCommit={(newValue) => handleSliderCommit(newValue, key, "IN")}/>
                                <p className="text-home_colors-Similar_White text-sm font-bold"> {labelChannelsInput[key]} </p>
                                <div className="text-home_colors-Selected_Borders/text border-[0.9px] w-10 justify-center text-center text-sm border-home_colors-Selected_Borders/text font-bold">IN</div>
                                <Mute size={"mute_preset"} 
                                    disabled={!(inputVisibility[key]) }
                                    variant={ inputVisibility[key] ?
                                              inputChannelStates[key] ? "muted": "unmuted"
                                                                    :"notAvailable" 
                                    } 
                                    onClick={()=>handleMute(key,"IN")}>
                                    MUTE
                                </Mute>
                              </div>
                                  
                            )
                          })}
                  </div>
                </div>
                <div className="flex border-[1.5px] border-home_colors-Selected_Borders/text border-opacity-40 bg-home_colors-Navbar/Selection_Bg rounded-[60px] h-[330px] w-[600px] overflow-hidden px-24">
                    <div className="flex gap-3 pb-3 overflow-x-auto ">
                      {Object.entries(outputVolumesStates).map(([key])=>{
                        if (key === "1" || key === "2") return null;
                        return(
                          
                          <div className="flex flex-col items-center justify-center gap-3">
                            <p className="text-home_colors-Similar_White text-sm font-bold">{ outputVolumesStates[key] } db </p>
                            <Slider orientation="vertical" className="h-full" 
                                    disabled={outputVisibility[key] ? outputChannelStates[key] : false} min={-60} max={15} 
                                    value={ outputVisibility[key] ? outputChannelStates[key] ? [-60] : [outputVolumesStates[key]] : [-60] } 
                                    onValueChange={(newValue) => handleSliderChange(newValue, key, "OUT")} 
                                    onValueCommit={(newValue) => handleSliderCommit(newValue, key, "OUT")}/>
                            <p className="text-home_colors-Similar_White text-sm font-bold"> {labelChannelsOutput[key]} </p>
                            <div className="text-home_colors-Selected_Borders/text border-[0.9px] w-10 justify-center text-center text-sm border-home_colors-Selected_Borders/text font-bold">OUT</div>
                            <Mute size={"mute_preset"} 
                                disabled={!(outputVisibility[key]) }
                                variant={ outputVisibility[key] ?
                                          outputChannelStates[key] ? "muted": "unmuted"
                                                                :"notAvailable" 
                                } 
                                onClick={()=>handleMute(key,"OUT")}>
                                MUTE
                            </Mute>
                          </div>
                              
                        )
                      })}
                  </div>
                </div>
            </div>
            <div className="flex flex-col gap-5 items-center justify-center">
                <ButtonPresets text={labelPresets[currentPresets.toString()]} onClick={()=>{navigate("/presets",{state:"house"})}}/>
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