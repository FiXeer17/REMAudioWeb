import { Navbar } from "@/components/ui/navbar";
import { Button as Mute } from "@/components/ui/button_mute";
import { Button as Channel } from "@/components/ui/button_channels"
import { Badge } from "@/components/ui/badge";
export default function Volume() {
  return (
    <div className="grid grid-rows-5 min-h-svh">
      <div></div>
      <div></div>
      <div className="flex flex-col">
        {/**<Badge variant={"modified"} >OUTPUT</Badge>*/}
        <div className=" grid grid-cols-4  ">
            <Channel variant={"channels_disabled"} >CH1</Channel>
            <Channel variant={"channels_disabled"} >CH1</Channel>
            <Channel variant={"channels_disabled"} >CH1</Channel>
            <Channel variant={"channels_disabled"} >CH1</Channel>
            <Channel variant={"channels_disabled"} >CH1</Channel>
            
        </div>
      </div>
      <div className="flex flex-col justify-center items-center">
        <Mute>MUTE ALL</Mute>
      </div>
      <div>
        <Navbar />
      </div>
    </div>
  );
}
