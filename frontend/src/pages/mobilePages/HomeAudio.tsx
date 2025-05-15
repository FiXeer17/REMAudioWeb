import { Navbar } from "@/components/ui/navbar";
import { Button as Mute } from "@/components/ui/button_mute";
import { Button as Channel } from "@/components/ui/button_channels";
import { Badge } from "@/components/ui/badge";
import { useNavigate } from "react-router-dom";
import { useState, useContext, useEffect } from "react";
import SocketContext from "@/lib/socket/context";
import { GetData } from "@/lib/WebSocketData";
import { Clock } from "@phosphor-icons/react";
import { ButtonPresets } from "@/components/ui/button_presets";
import { RecentConnections } from "./RecentConnections";

export const HomeAudio=() => {
  const [inputChannelStates, setInputChannelStates] = useState<{[key: string]: boolean;}>({});
  const [outputChannelStates, setOutputChannelStates] = useState<{[key: string]: boolean;}>({});
  const [inputVisibility, setInputVisibility] = useState<{[key: string]: boolean;}>({});
  const [outputVisibility, setOutputVisibility] = useState<{[key: string]: boolean;}>({});
  const {socket,message_matrix,camera_status,matrix_status} = useContext(SocketContext).socketState
  const [isAvailable, setIsAvailable] = useState(true)
  const [currentPresets,setCurrentPresets]=useState(0)
  const [labelPresets,setlabelPresets]=useState<{[key: string]: string;}>({})
  const [labelChannelInput,setLabelChannelInput]=useState<{[key: string]: string;}>({})
  const [labelChannelOutput,setLabelChannelOutput]=useState<{[key: string]: string;}>({})

  useEffect(() => {
    if (!isAvailable || message_matrix) return;
  
    const timeout = setTimeout(() => {
      navigate("/uuidprovider");
    }, 10000);
  
    return () => clearTimeout(timeout); 
  }, [isAvailable, message_matrix]);
  
  useEffect(()=>{
    if(matrix_status==="disconnected" && camera_status==="connected")
      navigate("/video")
  },[matrix_status])

  useEffect(()=>{
    if (!message_matrix) return
      const { inputChannelStates,outputChannelStates,isAvailable,outputVisibility, inputVisibility,currentPresets,labelPresets,labelChannelsInput,labelChannelsOutput } = GetData(message_matrix);
      setInputChannelStates(inputChannelStates);
      setOutputChannelStates(outputChannelStates);
      setInputVisibility(inputVisibility)
      setOutputVisibility(outputVisibility)
      setIsAvailable(isAvailable)
      setCurrentPresets(currentPresets)
      setlabelPresets(labelPresets)
      setLabelChannelInput(labelChannelsInput)
      setLabelChannelOutput(labelChannelsOutput)

    },[message_matrix])
  const navigate = useNavigate();

  const Channels = ["1", "2", "3", "4", "5", "6", "7", "8"];

  
  const handleState = (channel: string, type: string) => {
    if (type === "I") {;
      const data={"section":"mute","io":"input","channel":channel,"value":(!inputChannelStates[channel]).toString()}
      socket?.send(JSON.stringify(data))
    }else if(type==="O"){
      const data={"section":"mute","io":"output","channel":channel,"value":(!outputChannelStates[channel]).toString()}
      socket?.send(JSON.stringify(data))
    }else if(type==="ALL"){
      for (let channel=1;channel<=16;channel++){
        const dataoutput={"section":"mute","io":"output","channel":channel.toString(),"value":"true"}
        socket?.send(JSON.stringify(dataoutput))
        const datainput={"section":"mute","io":"input","channel":channel.toString(),"value":"true"}
        socket?.send(JSON.stringify(datainput))
      }
    }
  };
  

  return (
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
    
    <div className="grid grid-rows-[auto,1fr,1fr,auto] min-h-svh" >
      
      <div className="flex justify-center  pb-5 pt-4 ">
        <ButtonPresets text={labelPresets[currentPresets.toString()]} onClick={()=>{navigate("/presets",{state:"house"})}}/>
      </div>
      <div className="flex flex-col px-7 py-6">
        <div className="relative w-full h-full">
          <Badge className="absolute top-[-10px] left-5 transform -translate-x-1/2">
            INPUT
          </Badge>

          <div className="grid grid-rows-[1fr,auto] w-full h-full px-4 bg-home_colors-Navbar/Selection_Bg rounded-3xl">
            <div className="grid grid-cols-4 w-full items-center justify-items-center pt-2">
              {Channels.map((channel: string) => (
                
                <Channel
                  key={channel}
                  disabled={!inputVisibility[channel]}
                  variant={
                    inputVisibility[channel]?
                        inputChannelStates[channel]
                        ? "channels_disabled"
                        : "channels_activated"
                      : "channels_notVisible"
                  }
                  onClick={() => handleState(channel, "I")}
                >
                  {labelChannelInput[channel] ? (
                    labelChannelInput[channel].length > 3 ? (
                      <div className="relative w-full overflow-hidden">
                        <div className="whitespace-nowrap animate-marquee">
                          {labelChannelInput[channel]}
                        </div>
                      </div>
                    ) : (
                      labelChannelInput[channel]
                    )
                  ) : null}

                </Channel>
                
              ))}
            </div>

          </div>
        </div>
      </div>
      <div className="flex flex-col px-7 py-6">
        <div
          className="relative w-full h-full ">
          <Badge className="absolute top-[-10px] left-7 transform -translate-x-1/2">
            OUTPUT
          </Badge>
          <div className=" grid grid-rows-[1fr,auto] w-full h-full px-4 bg-home_colors-Navbar/Selection_Bg rounded-3xl">
            <div className="grid grid-cols-4 w-full items-center justify-items-center pt-2">
              {Channels.map((channel: string) => (
                <Channel
                  key={channel}
                  disabled={!outputVisibility[channel]}
                  variant={
                    outputVisibility[channel]?
                        outputChannelStates[channel]
                        ? "channels_disabled"
                        : "channels_activated"
                      : "channels_notVisible"
                  }
                  onClick={() => handleState(channel, "O")}
                >
                  {labelChannelOutput[channel] ? (
                    labelChannelOutput[channel].length > 3 ? (
                      <div className="relative w-full overflow-hidden">
                        <div className="whitespace-nowrap animate-marquee">
                          {labelChannelOutput[channel]}
                        </div>
                      </div>
                    ) : (
                      labelChannelOutput[channel]
                    )
                  ) : null}
                </Channel>
              ))}
            </div>
          </div>
        </div>
      </div>
      
      <div className="flex flex-col justify-between items-center pb-3 gap-12 pt-3 px-5 w-full">
        <Mute onClick={()=>handleState("","ALL")}>MUTE ALL</Mute>
        <Navbar selectedColor="house"/>
      </div>
      </div>
    </div>
    </>
  );
}
