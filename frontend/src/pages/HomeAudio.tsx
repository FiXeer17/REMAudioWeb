import { Navbar } from "@/components/ui/navbar";
import { Button as Mute } from "@/components/ui/button_mute";
import { Button as Channel } from "@/components/ui/button_channels";
import { Badge } from "@/components/ui/badge";
import { Button as Audio_Video } from "@/components/ui/audio_video";




export default function Volume() {
  return (
    <div className="grid grid-rows-[auto,1fr,1fr,auto] min-h-svh ">
      <div className="flex justify-start px-2 pb-5 pt-3 ">
        <div className=" grid grid-cols-2 px-5 py-3 w-full items-center justify-items-center bg-home_colors-Navbar/Selection_Bg rounded-full">
        <Audio_Video variant={"audio"}>AUDIO</Audio_Video>
        <Audio_Video variant={"video"}>VIDEO</Audio_Video>
        </div>
      </div>
      <div className="flex flex-col px-7 py-6">
        <div className="relative w-full h-full ">
          <Badge className="absolute top-[-10px] left-5 transform -translate-x-1/2">INPUT</Badge>
          <div className=" grid grid-cols-4  w-full h-full  items-center justify-items-center px-4 py-1 bg-home_colors-Navbar/Selection_Bg rounded-3xl">
            <Channel variant={"channels_activated"}>CH1</Channel>
            <Channel variant={"channels_activated"}>CH2</Channel>
            <Channel variant={"channels_activated"}>CH3</Channel>
            <Channel variant={"channels_activated"}>CH4</Channel>
            <Channel variant={"channels_activated"}>CH5</Channel>
            <Channel variant={"channels_activated"}>CH6</Channel>
            <Channel variant={"channels_activated"}>CH7</Channel>
            <Channel variant={"channels_activated"}>CH8</Channel>
          </div>
        </div>
      </div>
      <div className="flex flex-col px-7 py-6">
        <div className="relative w-full h-full ">
          <Badge className="absolute top-[-10px] left-7 transform -translate-x-1/2">OUTPUT</Badge>
          <div className=" grid grid-cols-4  w-full h-full  items-center justify-items-center px-4 py-1 bg-home_colors-Navbar/Selection_Bg rounded-3xl">
            <Channel variant={"channels_disabled"}>CH1</Channel>
            <Channel variant={"channels_disabled"}>CH2</Channel>
            <Channel variant={"channels_disabled"}>CH3</Channel>
            <Channel variant={"channels_disabled"}>CH4</Channel>
            <Channel variant={"channels_disabled"}>CH5</Channel>
            <Channel variant={"channels_disabled"}>CH6</Channel>
            <Channel variant={"channels_disabled"}>CH7</Channel>
            <Channel variant={"channels_disabled"}>CH8</Channel>
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
