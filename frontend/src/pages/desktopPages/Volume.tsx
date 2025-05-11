import { ButtonPresets } from "@/components/ui/button_presets"
import NavbarDesktop from "@/components/ui/navbarDesktop"
import { useCallback, useContext, useEffect, useState } from "react";
import { useNavigate } from "react-router-dom"
import useSliderThrottle from "@/lib/handleSwipe"; 
import { GetData } from "@/lib/WebSocketData";
import SocketContext from "@/lib/socket/context";
import { Slider } from "@/components/ui/slider";
import { Button as Mute } from "@/components/ui/button_mute";
import { Clock } from "@phosphor-icons/react";
import { RecentConnections } from "./RecentConnections";
import { ButtonDb } from "@/components/ui/button_db";

export const Volume=()=>{

    const [inputChannelStates, setInputChannelStates] = useState<{[key: string]: boolean;}>({});
    const [outputChannelStates, setOutputChannelStates] = useState<{[key: string]: boolean;}>({});
    const [inputVolumesStates, setInputVolumesStates] = useState<{[key: string]: number;}>({});
    const [outputVolumesStates, setOutputVolumesStates] = useState<{[key: string]: number;}>({});

    const [inputVisibility, setInputVisibility] = useState<{[key: string]: boolean;}>({});
    const [outputVisibility, setOutputVisibility] = useState<{[key: string]: boolean;}>({});

    const {socket,message_matrix,matrix_status,camera_status} = useContext(SocketContext).socketState
    const [isAvailable, setIsAvailable] = useState(true)

    const [labelChannelsInput,setlabelChannelInput]=useState<{[key: string]: string;}>({})
    const [labelChannelsOutput,setlabelChannelOutput]=useState<{[key: string]: string;}>({})
    const [labelPresets,setlabelPresets]=useState<{[key: string]: string;}>({})
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
      setlabelChannelInput(labelChannelsInput)
      setlabelChannelOutput(labelChannelsOutput)
      
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
    const handleButtonDb =(value:string,channel:string,source:string)=>{
      
      const isNumeric = (str: string) => /^[^a-zA-Z]+$/.test(str)
      if(!isNumeric(value)) return

      let valueNum = Number(value)

      valueNum = Math.round(valueNum);

      if (valueNum < -60) {
        value = "-60";
      } else if (valueNum > 15) {
        value = "15";
      } else {
        value = valueNum.toString();
      }
      if (source==="ALL"){
        const data = {"section": "volume", "io": "output", "channel": "1", "value": value};
        socket?.send(JSON.stringify(data));
        const data1 = {"section": "volume", "io": "output", "channel": "2", "value": value};
        socket?.send(JSON.stringify(data1));
      }else{
        const io = source === "IN" ? "input" : "output";
        const data = {"section": "volume", "io": io, "channel": channel, "value": value};
        socket?.send(JSON.stringify(data));
      }
      
    }
    
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
      
        <div className="grid grid-cols-[100px_1fr_0.5fr] h-screen">
            <div>
                <NavbarDesktop selectedColor="speaker" />
            </div>
            <div className="flex flex-col items-end justify-center gap-8 mr-6">
                <div className="flex border-[1.5px] border-home_colors-Selected_Borders/text border-opacity-40 bg-home_colors-Navbar/Selection_Bg rounded-[60px] h-[330px] w-[600px] overflow-hidden px-24  ">
                  <div className="flex gap-3 pb-3 overflow-x-auto ">
                    {Object.entries(inputVolumesStates).map(([key])=>{
                            return(
                              
                              <div className="flex flex-col items-center justify-center gap-3" key={key}>
                                <div className="flex gap-1">
                                  <ButtonDb onChange={(value) => handleButtonDb(value, key,"IN")} Text={inputVolumesStates[key].toString()}/>
                                  <p className="text-home_colors-Similar_White text-sm font-bold">db</p>
                                </div>
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
                          <div className="flex flex-col items-center justify-center gap-3" key={key}>
                            <div className="flex gap-1">
                              <ButtonDb onChange={(value) => handleButtonDb(value, key,"OUT")} Text={outputVolumesStates[key].toString()}/>
                              <p className="text-home_colors-Similar_White text-sm font-bold">db</p>
                            </div>
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
                <ButtonPresets text={labelPresets[currentPresets.toString()]} onClick={()=>{navigate("/presets",{state:"volume"})}}/>
                <div className="flex flex-col items-center gap-3 border-[1.5px] border-home_colors-Selected_Borders/text border-opacity-40 bg-home_colors-Navbar/Selection_Bg rounded-[60px] h-[550px] w-[100px] py-20  ">
                        <div className="flex gap-1">
                          <ButtonDb onChange={(value) => handleButtonDb(value, "1","ALL")} Text={outputChannelStates["1"] && outputChannelStates["2"] ? "-60" : outputVolumesStates["1"]?outputVolumesStates["1"].toString():""}/>
                          <p className="text-home_colors-Similar_White text-sm font-bold">db</p>
                        </div>
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
      </div>
    </>
    )
}