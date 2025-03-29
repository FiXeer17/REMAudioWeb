import { Navbar } from "@/components/ui/navbar";
import { Button as Mute } from "@/components/ui/button_mute";
import { Button as Channel } from "@/components/ui/button_channels";
import { Badge } from "@/components/ui/badge";
import { Button as Audio_Video } from "@/components/ui/audio_video";
import { useNavigate } from "react-router-dom";
import { SwipeChannels } from "../lib/swipeChannels";
import { useState,useContext,useEffect } from "react";
import SocketContext from "@/lib/socket/context";
import { GetData } from "@/lib/WebSocketData";
import { Circle } from "@phosphor-icons/react";



export default function Volume() {
  const [inputChannelStates, setInputChannelStates] = useState<{[key: string]: boolean;}>({});
  const [outputChannelStates, setOutputChannelStates] = useState<{[key: string]: boolean;}>({});
  const {socketState,socketDispatch}=useContext(SocketContext)
  const [message, setMessage]=useState("")
  
  useEffect(() => {
    if (!socketState.socket) return;

    socketState.socket.onmessage = (event) => {
        const {inputChannelStates,outputChannelStates} = GetData(event.data);
        setInputChannelStates(inputChannelStates)
        setOutputChannelStates(outputChannelStates)        
    };

}, [socketState.socket]);


  const navigate = useNavigate();

  const inputChannels1 = ["1","2","3","4","5","6","7","8",];
  const inputChannels2 = ["9","10","11","12","13","14","15","16",];

  const outputChannels1 = ["1","2","3","4","5","6","7","8",];
  const outputChannels2 = ["9","10","11","12","13","14","15","16",];

  const {
    displayedChannels: displayedInputChannels,
    offset: inputOffset,
    handleTouchStart: handleInputTouchStart,
    handleTouchMove: handleInputTouchMove,
    handleTouchEnd: handleInputTouchEnd,
  } = SwipeChannels(inputChannels1, inputChannels2);
  const {
    displayedChannels: displayedOutputChannels,
    offset: outputOffset,
    handleTouchStart: handleOutputTouchStart,
    handleTouchMove: handleOutputTouchMove,
    handleTouchEnd: handleOutputTouchEnd,
  } = SwipeChannels(outputChannels1, outputChannels2);
  /*
  const handleState = (channel: string, type: string) => {
    if (type === "I") {;
    }
  };
  */

  return (
    <div className="grid grid-rows-[auto,1fr,1fr,auto] min-h-svh ">
      <div className="flex justify-start px-2 pb-5 pt-3 ">
        <div className=" grid grid-cols-2 px-5 py-3 w-full items-center justify-items-center bg-home_colors-Navbar/Selection_Bg rounded-full">
          <Audio_Video variant={"blue"}>AUDIO</Audio_Video>
          <Audio_Video variant={"white"} onClick={() => navigate("/homeVideo")}>
            VIDEO
          </Audio_Video>
        </div>
      </div>
      <div className="flex flex-col px-7 py-6">
      
        <div
          className="relative w-full h-full  "
          style={{
            transform: `translateX(${inputOffset}px)`,
            transition: inputOffset === 0 ? "transform 0.3s ease" : "none",
          }}
        >
            <Badge className="absolute top-[-10px] left-5 transform -translate-x-1/2">
              INPUT
            </Badge>
            <div
                className="grid grid-cols-4  w-full h-full  items-center justify-items-center px-4 py-1 bg-home_colors-Navbar/Selection_Bg rounded-3xl"
                onTouchStart={handleInputTouchStart}
                onTouchMove={handleInputTouchMove}
                onTouchEnd={handleInputTouchEnd}
              >
                
              {displayedInputChannels.map((channel: string) => (
                <Channel key={channel} variant={inputChannelStates[channel] ? "channels_activated" : "channels_disabled"}>  
                  {`CH${channel}`}
                </Channel>
              ))} 
              <div></div>
              <div></div>
              <div className="flex justify-center gap-4">
                <Circle size={14} color="#ffffff" />
                <Circle size={14} color="#ffffff" />
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
            className=" grid grid-cols-4  w-full h-full  items-center justify-items-center px-4 py-1 bg-home_colors-Navbar/Selection_Bg rounded-3xl"
            onTouchStart={handleOutputTouchStart}
            onTouchMove={handleOutputTouchMove}
            onTouchEnd={handleOutputTouchEnd}
          >
            {displayedOutputChannels.map((channel: string) => (
              <Channel
                key={channel}
                variant={outputChannelStates[channel] ? "channels_activated" : "channels_disabled"}
                
              >
                {`CH${channel}`}
              </Channel>
            ))}
          </div>
          
        </div>
      </div>
      <div className="flex flex-col justify-between items-center pb-3 gap-12 pt-3">
        <Mute>MUTE ALL</Mute>
        <Navbar />
      </div>
    </div>
  );
}
