import NavbarDesktop from "@/components/ui/navbarDesktop"
import { GetData } from "@/lib/WebSocketData";
import { Button as PresetsButton } from "@/components/ui/audio_video";
import SocketContext from "@/lib/socket/context";
import { useNavigate } from "react-router-dom";
import { ButtonEdit } from "@/components/ui/button_edit";
import { useContext, useEffect, useState } from "react";
import { MatrixCameraButton } from "@/components/ui/matrix_camera";

export const PreferenciesPresets=()=>{
    const navigate=useNavigate()
    const {socket,message_matrix,message_camera,matrix_status,camera_status} = useContext(SocketContext).socketState
    const [labelPresetsMatrix,setlabelPresetsMatrix]=useState<{[key: string]: string;}>({})
    const [labelPresetsCamera,setlabelPresetsCamera]=useState<{[key: string]: string;}>({})
    const [MatrixCamera,setMatrixCamera]=useState<"MATRIX"|"CAMERA">("MATRIX")
    const [ hasLatestAudio,setHasLatestAudio ] = useState(false)
    const [ hasLatestVideo,setHasLatestVideo ] = useState(false)

    useEffect(()=>{
        if(camera_status==="connected"){
            setHasLatestVideo(true)
        }else if(camera_status==="disconnected"){
            setHasLatestVideo(false)
        }
    },[camera_status])
    useEffect(()=>{
        if(matrix_status==="connected"){
            setHasLatestAudio(true)
        }else if(matrix_status==="disconnected"){
            setHasLatestAudio(false)
        }
    },[matrix_status])

    useEffect(()=>{
        if (!message_matrix) return
        const { labelPresets} = GetData(message_matrix);
        setlabelPresetsMatrix(labelPresets)
      },[message_matrix])
    
    useEffect(()=>{
        if (!message_camera) return
        const { labelPresets} = GetData(message_camera);
        setlabelPresetsCamera(labelPresets)
    },[message_camera])

    const handleSetNamePreset=(value:string,Preset:string)=>{
        if (MatrixCamera==="MATRIX"){
            const dataoutput={"section":"matrix_preset_labels","index":Preset,"value":value}
            socket?.send(JSON.stringify(dataoutput))
        }else{
            const dataoutput={"section":"camera_preset_labels","index":Preset,"value":value}
            socket?.send(JSON.stringify(dataoutput))
        }
    }

    return(
        <div className="grid grid-cols-[100px,1fr] h-screen">
            <div>
                <NavbarDesktop selectedColor="settings" />
            </div>
            <div className="flex items-center justify-center w-full">
                <div className="flex flex-col border-[1.5px]  border-home_colors-Selected_Borders/text border-opacity-40 bg-home_colors-Navbar/Selection_Bg rounded-[60px] h-[600px] w-[500px]  px-10 py-10 ">
                    <div className="flex items-center justify-center gap-3" >
                        <PresetsButton  variant={"blue"} className="flex flex-col gap-0 items-center justify-center text-center ">
                            <span>LABELS</span> 
                            <span>PRESETS</span>
                        </PresetsButton>
                        <PresetsButton variant={"white"} onClick={()=>navigate("/preferenciesChannels")}>CHANNELS</PresetsButton>
                    </div>
                    <div className="flex flex-1 px-7 pb-5 overflow-hidden relative pt-5">
                        <div className="grid grid-cols-2 h-full w-full bg-home_colors-Navbar/Selection_Bg rounded-2xl px-10 py-10 gap-5 overflow-y-auto">
                            {MatrixCamera==="MATRIX" ?
                                Object.entries(labelPresetsMatrix).map(([key,Presets])=>(
                                    <ButtonEdit  key={key} onChange={(value)=>{handleSetNamePreset(value,key)}} Text={Presets}/>
                                    )):
                                Object.entries(labelPresetsCamera).map(([key,Presets])=>(
                                    <ButtonEdit  key={key} onChange={(value)=>{handleSetNamePreset(value,key)}} Text={Presets}/>
                                    ))
                            }
                        </div>
                    </div>
                    <div className="flex items-center justify-center">
                        <MatrixCameraButton onChange={setMatrixCamera} device_disconnected={hasLatestAudio?hasLatestVideo?"":"camera":"matrix"}/>
                    </div>
                </div>
            </div>
        </div>
    )
}