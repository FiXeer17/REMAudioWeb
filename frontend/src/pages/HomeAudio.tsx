import { Navbar } from "@/components/ui/navbar";
import { Button as Mute } from "@/components/ui/button_mute";
import { Button as Channel } from "@/components/ui/button_channels";
import { Badge } from "@/components/ui/badge";
import { Button as Audio_Video } from "@/components/ui/audio_video";
import { useNavigate } from "react-router-dom";
import { SwipeChannels } from "../lib/swipeChannels";
import { useState } from "react";

export default function Volume() {
  const [inputChannelStates, setInputChannelStates] = useState<{
    [key: string]: boolean;
  }>({});
  const [outputChannelStates, setOutputChannelStates] = useState<{
    [key: string]: boolean;
  }>({});

  const navigate = useNavigate();

  const inputChannels1 = [
    "CH1",
    "CH2",
    "CH3",
    "CH4",
    "CH5",
    "CH6",
    "CH7",
    "CH8",
  ];

  const inputChannels2 = [
    "CH9",
    "CH10",
    "CH11",
    "CH12",
    "CH13",
    "CH14",
    "CH15",
    "CH16",
  ];

  const outputChannels1 = [
    "CH1",
    "CH2",
    "CH3",
    "CH4",
    "CH5",
    "CH6",
    "CH7",
    "CH8",
  ];
  const outputChannels2 = [
    "CH9",
    "CH10",
    "CH11",
    "CH12",
    "CH13",
    "CH14",
    "CH15",
    "CH16",
  ];

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

  const handleState = (channel: string, type: string) => {
    if (type === "I") {
      setInputChannelStates((prev) => {
        return {
          ...prev,
          [channel]: !prev[channel],
        };
      });
    } else {
      setOutputChannelStates((prev) => {
        return {
          ...prev,
          [channel]: !prev[channel],
        };
      });
    }
  };

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
          className="relative w-full h-full "
          style={{
            transform: `translateX(${inputOffset}px)`,
            transition: inputOffset === 0 ? "transform 0.3s ease" : "none",
          }}
        >
          <Badge className="absolute top-[-10px] left-5 transform -translate-x-1/2">
            INPUT
          </Badge>
          <div
            className=" grid grid-cols-4  w-full h-full  items-center justify-items-center px-4 py-1 bg-home_colors-Navbar/Selection_Bg rounded-3xl"
            onTouchStart={handleInputTouchStart}
            onTouchMove={handleInputTouchMove}
            onTouchEnd={handleInputTouchEnd}
          >
            {displayedInputChannels.map((channel: string) => (
              <Channel
                key={channel}
                variant={inputChannelStates[channel] ? "channels_activated" : "channels_disabled"}
                onClick={() => handleState(channel, "I")}
              >
                {channel}
              </Channel>
            ))}
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
                variant={outputChannelStates[channel] ? "channels_activated" : "channels_disabled"
                }
                onClick={() => handleState(channel, "O")}
              >
                {channel}
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
