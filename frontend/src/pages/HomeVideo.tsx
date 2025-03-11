import { Navbar } from "@/components/ui/navbar";
import { Button as Mute } from "@/components/ui/button_mute";
import { Button as Channel } from "@/components/ui/button_channels";
import { Badge } from "@/components/ui/badge";
import { Button as Audio_Video } from "@/components/ui/audio_video";
import { useNavigate } from "react-router-dom"




export default function Volume() {
  const channels= ["CH1","CH2","CH3","CH4","CH5","CH6","CH7","CH8"]
  const navigate= useNavigate();
  const handleState=(channel:string,type:string)=>{
    channel
    console.log(type)
  }

  return (
    <div className="grid grid-rows-[auto,1fr,1fr,auto] min-h-svh ">
      <div className="flex justify-start px-2 pb-5 pt-3 ">
        <div className=" grid grid-cols-2 px-5 py-3 w-full items-center justify-items-center bg-home_colors-Navbar/Selection_Bg rounded-full">
        <Audio_Video variant={"white"} onClick={()=>navigate("/homeAudio")}>AUDIO</Audio_Video>
        <Audio_Video variant={"blue"}>VIDEO</Audio_Video>
        </div>
      </div>
      <div className="flex flex-col px-7 py-6">
        <div className="relative w-full h-full ">
          <Badge className="absolute top-[-10px] left-5 transform -translate-x-1/2">INPUT</Badge>
          <div className=" grid grid-cols-4  w-full h-full  items-center justify-items-center px-4 py-1 bg-home_colors-Navbar/Selection_Bg rounded-3xl">
          {channels.map((channel:string)=>(<Channel key={channel} variant={"channels_activated"} onClick={()=>handleState(channel,"I")}>{channel}</Channel>))}
          </div>
        </div>
      </div>
      <div className="flex flex-col px-7 py-6">
        <div className="relative w-full h-full ">
          <Badge className="absolute top-[-10px] left-7 transform -translate-x-1/2">OUTPUT</Badge>
          <div className=" grid grid-cols-4  w-full h-full  items-center justify-items-center px-4 py-1 bg-home_colors-Navbar/Selection_Bg rounded-3xl">
          {channels.map((channel:string)=>(<Channel key={channel} variant={"channels_activated"} onClick={()=>handleState(channel,"O")}>{channel}</Channel>))}
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