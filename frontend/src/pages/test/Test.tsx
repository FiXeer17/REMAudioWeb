import { Clock } from "@phosphor-icons/react";
import NavbarDesktop from "@/components/ui/navbarDesktop";
import { Button as Mute } from "@/components/ui/button_mute";
import { Button as Channel } from "@/components/ui/button_channels";
import { Badge } from "@/components/ui/badge";
import { Button as Audio_Video } from "@/components/ui/audio_video";
import { useNavigate } from "react-router-dom";
import { SwipeChannels } from "../../lib/swipeChannels";
import { useState, useContext, useEffect } from "react";
import SocketContext from "@/lib/socket/context";
import { GetData } from "@/lib/WebSocketData";
import { Circle } from "@phosphor-icons/react";
function App() {
  const Channels = [
    "1",
    "2",
    "3",
    "4",
    "5",
    "6",
    "7",
    "8",
    "9",
    "10",
    "11",
    "12",
    "13",
    "14",
    "15",
    "16",
  ];

  return (
    <div className="grid grid-cols-[100px,1fr] h-screen">
      <div>
        <NavbarDesktop selectedColor="house" />
      </div>
      <div className="grid grid-rows-4 items-center justify-center">
        <div className="flex items-center justify-center ">
          <Audio_Video variant={"blue"}>
            PRESET
          </Audio_Video>
        </div>
        <div className="relative px-10 py-7 bg-home_colors-Navbar/Selection_Bg rounded-xl">
          <Badge className="absolute top-[-10px] left-5 transform -translate-x-1/2">
            INPUT
          </Badge>
          <div className="grid grid-cols-8 w-full items-center justify-items-center pt-2 gap-5">
            {Channels.map((channel: string) => (
              <Channel key={channel} variant={"channels_disabled"} size={"desktop"}>
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
              <Channel key={channel} variant={"channels_disabled"} size={"desktop"}>
                {`CH${channel}`}
              </Channel>
            ))}
          </div>
        </div>
        <div className="flex items-center justify-center">
          <Mute>
            MUTE ALL
          </Mute>
        </div>
      </div>
    </div>
  );
}

export default App;
