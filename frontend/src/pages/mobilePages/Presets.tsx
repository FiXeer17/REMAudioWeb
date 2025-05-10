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
    const [isLoading,setIsLoading] = useState(false)
    const {socket,message_matrix} = useContext(SocketContext).socketState
    const [currentPresets,setCurrentPresets]=useState(0)
    const [colorNav] = useState<string>(() => location.state);
    const [labelPresets,setlabelPresets]=useState<{[key: string]: string;}>({})
    const Presets = [1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16];


    useEffect(()=>{
      const { currentPresets,labelPresets } = GetData(message_matrix);
        setCurrentPresets(currentPresets)
        setlabelPresets(labelPresets)
      },[message_matrix])

    useEffect(()=>{
      if (isLoading===true){
        setIsLoading(false)
        colorNav==="house" ? navigate("/homeAudio") : navigate("/volume")
        }
    },[currentPresets])

    const handleSetPreset=(Preset:number)=>{
      const dataoutput={"section":"matrix_preset","value":Preset.toString()}
      socket?.send(JSON.stringify(dataoutput))
      setIsLoading(true)
    }

  return (
    <>
      {isLoading ? 
                <div className="absolute inset-0 backdrop-blur-sm flex justify-center items-center  bg-black/30 z-30">
                   <div className="w-10 h-10 border-4 border-white border-t-transparent rounded-full animate-spin"></div>
                </div>:<div className="absolute inset-0 z-10"></div>}
      <div className="absolute inset-0 bg-black z-20">
        <div className="grid grid-rows-[70px,1fr,auto] h-screen relative">
          <div >
                
          </div>
          <div className="flex flex-1 px-7 pb-5 overflow-hidden relative pt-5">
                <Badge className="absolute left-12 top-3 transform -translate-x-1/2">
                  PRESETS
                </Badge>
              <div className="grid grid-cols-2 h-full w-full bg-home_colors-Navbar/Selection_Bg rounded-2xl px-10 py-10 gap-5 overflow-y-auto scroll">
                  {Presets.map((Presets)=>(
                    <PresetsButton size={"presets"} variant={currentPresets === Presets ? "blue" : "white"} key={Presets} onClick={()=>handleSetPreset(Presets)}>{labelPresets[Presets.toString()]}</PresetsButton>
                  ))}
              </div>
          </div>
          <div className="flex items-center pb-3 pt-3">
                <Navbar selectedColor={colorNav}/>
          </div>
        </div>
      </div>
    </>
  );
};
