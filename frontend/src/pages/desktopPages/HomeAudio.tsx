import NavbarDesktop from "@/components/ui/navbarDesktop";
import { Button as Mute } from "@/components/ui/button_mute";
import { Button as Channel } from "@/components/ui/button_channels";
import { Badge } from "@/components/ui/badge";
import { Button as Audio_Video } from "@/components/ui/audio_video";
import { Button as Presets } from "@/components/ui/audio_video";
import { useNavigate } from "react-router-dom";
import { SwipeChannels } from "../../lib/swipeChannels";
import { useState, useContext, useEffect } from "react";
import SocketContext from "@/lib/socket/context";
import { GetData } from "@/lib/WebSocketData";
import { Clock } from "@phosphor-icons/react";
import { ButtonPresets } from "@/components/ui/button_presets";
import { RecentConnections } from "./RecentConnections";

export const HomeAudio=()=> {
    const [inputChannelStates, setInputChannelStates] = useState<{[key: string]: boolean;}>({});
    const [outputChannelStates, setOutputChannelStates] = useState<{[key: string]: boolean;}>({});
    const [inputVisibility, setInputVisibility] = useState<{[key: string]: boolean;}>({});
    const [outputVisibility, setOutputVisibility] = useState<{[key: string]: boolean;}>({});
    const [labelPresets,setlabelPresets]=useState<{[key: string]: string;}>({})
    const [currentPresets,setCurrentPresets]=useState(0)
    const {socket,message_matrix} = useContext(SocketContext).socketState
    const [isAvailable, setIsAvailable] = useState(true)
    const navigate=useNavigate()
    const Channels = ["1","2","3","4","5","6","7","8"];

    useEffect(()=>{
      if(!message_matrix) return 
      
      const { inputChannelStates,outputChannelStates,isAvailable,outputVisibility, inputVisibility,currentPresets,labelPresets } = GetData(message_matrix);
        setInputChannelStates(inputChannelStates);
        setOutputChannelStates(outputChannelStates);
        setInputVisibility(inputVisibility)
        setOutputVisibility(outputVisibility)
        setIsAvailable(isAvailable)
        setCurrentPresets(currentPresets)
        setlabelPresets(labelPresets)
      },[message_matrix])

    const handleState = (channel: string, type: string) => {
        if (type === "I") {

        const data = {section: "mute",io: "input",channel: channel,value: (!inputChannelStates[channel]).toString(),};
        socket?.send(JSON.stringify(data));

        } else if (type === "O") {

        const data = {section: "mute",io: "output",channel: channel,value: (!outputChannelStates[channel]).toString(),};
        socket?.send(JSON.stringify(data));

        } else if (type === "ALL") {

          for (let channel = 1; channel <= 8; channel++) {
            const dataoutput = {section: "mute",io: "output",channel: channel.toString(),value: "true",};
            socket?.send(JSON.stringify(dataoutput));
            const datainput = {section: "mute",io: "input",channel: channel.toString(),value: "true",};
            socket?.send(JSON.stringify(datainput));
        }
        }
  };

  return (
    <>
      {isAvailable ? ( message_matrix ?
        <div className="absolute inset-0 z-10"></div>:<RecentConnections isLoading={true}/>
      ) : (
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
        </div>
      )}
      <div className="absolute inset-0 bg-black z-20">
        <div className="grid grid-cols-[100px,1fr] h-screen">
            <div>
              <NavbarDesktop selectedColor="house" />
            </div>
            <div className="grid grid-rows-4 items-center justify-center">
              <div className="flex items-center justify-center ">
                  <ButtonPresets text={labelPresets[currentPresets.toString()]} onClick={()=>{navigate("/presets",{state:"house"})}}/>
              </div>
              <div className="relative px-10 py-7 bg-home_colors-Navbar/Selection_Bg rounded-xl">
                  <Badge className="absolute top-[-10px] left-5 transform -translate-x-1/2">
                  INPUT
                  </Badge>
                  <div className="grid grid-cols-8 w-full items-center justify-items-center pt-2 gap-5">
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
                      size={"desktop"}
                      onClick={() => handleState(channel, "I")}
                      
                      >
                      {`CH${channel}`}
                      </Channel>
                  ))}
                  </div>
              </div>
              <div className="relative px-10 py-7 bg-home_colors-Navbar/Selection_Bg rounded-xl">
                  <Badge className="absolute top-[-10px] left-6 transform -translate-x-1/2">
                  OUTPUT
                  </Badge>
                  <div className="grid grid-cols-8 w-full items-center justify-items-center pt-2 gap-5">
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
                      size={"desktop"}
                      onClick={() => handleState(channel, "O")}
                      >
                      {`CH${channel}`}
                      </Channel>
                  ))}
                  </div>
              </div>
              <div className="flex items-center justify-center">
                  <Mute onClick={() => handleState("", "ALL")}>MUTE ALL</Mute>
              </div>
            </div>
        </div>
      </div>
    </>
  );
}
