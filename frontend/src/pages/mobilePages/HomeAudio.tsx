import { Navbar } from "@/components/ui/navbar";
import { Button as Mute } from "@/components/ui/button_mute";
import { Button as Channel } from "@/components/ui/button_channels";
import { Badge } from "@/components/ui/badge";
import { Button as PresetsButton } from "@/components/ui/audio_video";
import { useNavigate } from "react-router-dom";
import { SwipeChannels } from "../../lib/swipeChannels";
import { useState, useContext, useEffect } from "react";
import SocketContext from "@/lib/socket/context";
import { GetData } from "@/lib/WebSocketData";
import { Circle, Clock } from "@phosphor-icons/react";
import { ButtonPresets } from "@/components/ui/button_presets";

export const HomeAudio=() => {
  const [inputChannelStates, setInputChannelStates] = useState<{[key: string]: boolean;}>({});
  const [outputChannelStates, setOutputChannelStates] = useState<{[key: string]: boolean;}>({});
  const [inputVisibility, setInputVisibility] = useState<{[key: string]: boolean;}>({});
  const [outputVisibility, setOutputVisibility] = useState<{[key: string]: boolean;}>({});
  const {socket,message} = useContext(SocketContext).socketState
  const [isAvailable, setIsAvailable] = useState(true)
  const [currentPresets,setCurrentPresets]=useState(0)
  const [labelPresets,setlabelPresets]=useState<{[key: string]: string;}>({})

  useEffect(()=>{

    const { inputChannelStates, outputChannelStates,isAvailable,outputVisibility, inputVisibility,currentPresets,labelPresets } = GetData(message);
      setInputChannelStates(inputChannelStates);
      setOutputChannelStates(outputChannelStates);
      setInputVisibility(inputVisibility)
      setOutputVisibility(outputVisibility)
      setIsAvailable(isAvailable)
      setCurrentPresets(currentPresets)
      setlabelPresets(labelPresets)
  },[message])
  const navigate = useNavigate();

  const inputChannels1 = ["1", "2", "3", "4", "5", "6", "7", "8"];
  const inputChannels2 = ["9", "10", "11", "12", "13", "14", "15", "16"];

  const outputChannels1 = ["1", "2", "3", "4", "5", "6", "7", "8"];
  const outputChannels2 = ["9", "10", "11", "12", "13", "14", "15", "16"];

  const {
    currentSet: colorInputCircle,
    displayedChannels: displayedInputChannels,
    offset: inputOffset,
    handleTouchStart: handleInputTouchStart,
    handleTouchMove: handleInputTouchMove,
    handleTouchEnd: handleInputTouchEnd,
  } = SwipeChannels(inputChannels1, inputChannels2);
  const {
    currentSet: colorOutputCircle,
    displayedChannels: displayedOutputChannels,
    offset: outputOffset,
    handleTouchStart: handleOutputTouchStart,
    handleTouchMove: handleOutputTouchMove,
    handleTouchEnd: handleOutputTouchEnd,
  } = SwipeChannels(outputChannels1, outputChannels2);
  
  const handleState = (channel: string, type: string) => {
    if (type === "I") {;
      const data={"section":"mute","io":"input","channel":channel,"value":(!inputChannelStates[channel]).toString()}
      socket?.send(JSON.stringify(data))
    }else if(type==="O"){
      const data={"section":"mute","io":"output","channel":channel,"value":(!outputChannelStates[channel]).toString()}
      socket?.send(JSON.stringify(data))
    }else if(type==="all"){
      for (let channel=1;channel<=16;channel++){
        const dataoutput={"section":"mute","io":"output","channel":channel.toString(),"value":"false"}
        socket?.send(JSON.stringify(dataoutput))
        const datainput={"section":"mute","io":"input","channel":channel.toString(),"value":"false"}
        socket?.send(JSON.stringify(datainput))
      }
    }
  };
  

  return (
    <>
    {isAvailable ? <div className="absolute inset-0 z-10"></div>:
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
        <div
          className="relative w-full h-full"
          style={{
            transform: `translateX(${inputOffset}px)`,
            transition: inputOffset === 0 ? "transform 0.3s ease" : "none",
          }}
        >
          <Badge className="absolute top-[-10px] left-5 transform -translate-x-1/2">
            INPUT
          </Badge>

          <div
            className="grid grid-rows-[1fr,auto] w-full h-full px-4 bg-home_colors-Navbar/Selection_Bg rounded-3xl"
            onTouchStart={handleInputTouchStart}
            onTouchMove={handleInputTouchMove}
            onTouchEnd={handleInputTouchEnd}
          >
            <div className="grid grid-cols-4 w-full items-center justify-items-center pt-2">
              {displayedInputChannels.map((channel: string) => (
                
                <Channel
                  key={channel}
                  disabled={!inputVisibility[channel]}
                  variant={
                    inputVisibility[channel]?
                        inputChannelStates[channel]
                        ? "channels_activated"
                        : "channels_disabled"
                      : "channels_notVisible"
                  }
                  onClick={() => handleState(channel, "I")}
                >
                  {`CH${channel}`}
                </Channel>
                
              ))}
            </div>
            <div className="flex items-center justify-center pb-2">
              {colorInputCircle === 1 ? (
                <Circle size={12} color="#ffffff" />
              ) : (
                <Circle size={12} color="#ffffff" weight="fill" />
              )}
              {colorInputCircle === 1 ? (
                <Circle size={12} color="#ffffff" weight="fill" />
              ) : (
                <Circle size={12} color="#ffffff" />
              )}
            </div>
          </div>
        </div>
      </div>
      <div className="flex flex-col px-7 py-6">
        <div
          className="relative w-full h-full "
          style={{
            transform: `translateX(${outputOffset}px)`,
            transition: outputOffset === 0 ? "transform 0.3s ease" : "none",
          }}
        >
          <Badge className="absolute top-[-10px] left-7 transform -translate-x-1/2">
            OUTPUT
          </Badge>
          <div
            className=" grid grid-rows-[1fr,auto] w-full h-full px-4 bg-home_colors-Navbar/Selection_Bg rounded-3xl"
            onTouchStart={handleOutputTouchStart}
            onTouchMove={handleOutputTouchMove}
            onTouchEnd={handleOutputTouchEnd}
          >
            <div className="grid grid-cols-4 w-full items-center justify-items-center pt-2">
              {displayedOutputChannels.map((channel: string) => (
                <Channel
                  key={channel}
                  disabled={!outputVisibility[channel]}
                  variant={
                    outputVisibility[channel]?
                        outputChannelStates[channel]
                        ? "channels_activated"
                        : "channels_disabled"
                      : "channels_notVisible"
                  }
                  onClick={() => handleState(channel, "O")}
                >
                  {`CH${channel}`}
                </Channel>
              ))}
            </div>
            <div className="flex items-center justify-center pb-2">
              {colorOutputCircle === 1 ? (
                <Circle size={12} color="#ffffff" />
              ) : (
                <Circle size={12} color="#ffffff" weight="fill" />
              )}
              {colorOutputCircle === 1 ? (
                <Circle size={12} color="#ffffff" weight="fill" />
              ) : (
                <Circle size={12} color="#ffffff" />
              )}
            </div>
          </div>
        </div>
      </div>
      
      <div className="flex flex-col justify-between items-center pb-3 gap-12 pt-3 px-5">
        <Mute onClick={()=>handleState("","all")}>MUTE ALL</Mute>
        <Navbar selectedColor="house"/>
      </div>
      </div>
    </div>
    </>
  );
}
