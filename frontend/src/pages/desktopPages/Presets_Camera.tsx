import { Button as PresetsButton } from "@/components/ui/audio_video";
import { Badge } from "@/components/ui/badge";
import SocketContext from "@/lib/socket/context";
import { useNavigate } from "react-router-dom";
import { useContext, useEffect, useState } from "react";
import { GetData } from "@/lib/WebSocketData";
import NavbarDesktop from "@/components/ui/navbarDesktop";
import { toast, Toaster } from "sonner";

export const Presets_Camera = () => {
    const navigate=useNavigate()
    const [isLoading,setIsLoading] = useState(false)
    const {socket,message_camera} = useContext(SocketContext).socketState
    const [currentPresets,setCurrentPresets]=useState(0)
    const [labelPresets,setlabelPresets]=useState<{[key: string]: string;}>({})

    useEffect(() => {
        const timeout = setTimeout(() => {
            setIsLoading(false)
            toast.error("Error setting preset",{duration:1000})
        }, 10000);
  
        return () => clearTimeout(timeout);
    }, [message_camera]);

    useEffect(()=>{
        if (!message_camera) return
        const { currentPresets,labelPresets } = GetData(message_camera);
        setCurrentPresets(currentPresets)
        setlabelPresets(labelPresets)
      },[message_camera])

    useEffect(()=>{
      if (isLoading===true){
        setIsLoading(false)
        navigate("/video")
        }
    },[currentPresets])

    const handleSetPreset=(Preset:string)=>{
      const dataoutput={"section":"camera_preset","value":Preset}
      if(Preset===String(currentPresets)){
        navigate("/video")
      }else{
        socket?.send(JSON.stringify(dataoutput))
        setIsLoading(true)
      }
    }
    return(
        <>
            {isLoading ? 
                <div className="absolute inset-0 backdrop-blur-sm flex justify-center items-center  bg-black/30 z-30">
                   <div className="w-10 h-10 border-4 border-white border-t-transparent rounded-full animate-spin"></div>
                </div>:<div className="absolute inset-0 z-10"></div>}
            <div className="absolute inset-0 bg-black z-20">
                <div className="grid grid-cols-[100px,1fr] h-screen">
                    <div>
                        <NavbarDesktop selectedColor="video" />
                    </div>
                    <div className="flex items-center justify-center w-full">
                        <div className="flex flex-col border-[1.5px] border-home_colors-Selected_Borders/text border-opacity-40 bg-home_colors-Navbar/Selection_Bg rounded-[60px] h-[600px] w-[500px]  px-10 py-10 ">
                            <div className="flex flex-1 px-7 pb-3 overflow-hidden relative pt-5">
                                <Badge className="absolute left-12 top-3 transform -translate-x-1/2">
                                    PRESETS
                                </Badge>
                                <div className="grid grid-cols-2 h-full w-full bg-home_colors-Navbar/Selection_Bg rounded-2xl px-10 py-10 gap-5 overflow-y-auto scroll">
                                    {Object.entries(labelPresets).map(([key,Presets],index)=>(
                                        <PresetsButton size={"presets"} variant={currentPresets === index ? "blue" : "white"} key={key} onClick={()=>handleSetPreset(key)}>{Presets}</PresetsButton>
                                    ))}
                                </div>
                            </div>
                        </div>
                        <Toaster/>
                    </div>
                </div>
            </div>
        </>
    )
}