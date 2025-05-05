import { Clock } from "@phosphor-icons/react";
import { useContext, useEffect, useState } from "react";
import { GetData } from "@/lib/WebSocketData";
import Navbar from "@/components/ui/navbar";
import { Button as PresetsButton } from "@/components/ui/audio_video";
import { Badge } from "@/components/ui/badge";
import SocketContext from "@/lib/socket/context";
import { useLocation, useNavigate } from "react-router-dom";

export const Presets = () => {
    const navigate=useNavigate()
    const location=useLocation()
    const {socket,message} = useContext(SocketContext).socketState
    const [currentPresets,setCurrentPresets]=useState(0)
    const [isAvailable, setIsAvailable] = useState(true)
    const [colorNav] = useState<string>(() => location.state);
    const [labelPresets,setlabelPresets]=useState<{[key: string]: string;}>({})
    const Presets = [1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16];


    useEffect(()=>{

      const { isAvailable,currentPresets,labelPresets }=GetData(message)      
      setIsAvailable(isAvailable)
      setCurrentPresets(currentPresets)
      setlabelPresets(labelPresets)
 
    },[message])

    const handleSetPreset=(Preset:number)=>{
      const dataoutput={"section":"preset","value":Preset.toString()}
      socket?.send(JSON.stringify(dataoutput))
      colorNav==="house" ? navigate("/homeAudio") : navigate("/volume")
    }

  return (
    <>
      {isAvailable ? (
        <div className="absolute inset-0 z-10"></div>
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
        <div className="grid grid-rows-[70px,1fr,auto] h-screen relative">
          <div >
                
          </div>
          <div className="flex flex-1 px-7 pb-5 overflow-hidden relative pt-5">
                <Badge className="absolute left-12 top-3 transform -translate-x-1/2">
                  PRESETS
                </Badge>
              <div className="grid grid-cols-2 h-full w-full bg-home_colors-Navbar/Selection_Bg rounded-2xl px-10 py-10 gap-5 overflow-y-auto">
                  {Presets.map((Presets)=>(
                    <PresetsButton size={"presets"} variant={currentPresets === Presets ? "blue" : "white"} key={Presets} onClick={()=>handleSetPreset(Presets)}>{labelPresets[Presets.toString()]}</PresetsButton>
                  ))}
              </div>
          </div>
          <div className="flex justify-between items-center pb-3 gap-12 pt-3">
                <Navbar selectedColor={colorNav}/>
          </div>
        </div>
      </div>
    </>
  );
};
