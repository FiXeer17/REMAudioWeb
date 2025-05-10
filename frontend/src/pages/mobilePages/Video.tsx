import Navbar from "@/components/ui/navbar"
import SocketContext from "@/lib/socket/context";
import { GetData } from "@/lib/WebSocketData";
import { useContext, useEffect, useState } from "react";
import { RecentConnections } from "../connections_socket/RecentConnections";
import { Clock,ImageSquare,MagnifyingGlassPlus,ArrowDown,ArrowLeft,ArrowUp,ArrowRight } from "@phosphor-icons/react";
import { Slider } from "@/components/ui/slider";
import { ButtonPresets } from "@/components/ui/button_presets";
import { useNavigate } from "react-router-dom";


export const Video=()=>{
    const navigate = useNavigate()
    const {socket,message_camera} = useContext(SocketContext).socketState
    const [labelPresets,setlabelPresets]=useState<{[key: string]: string;}>({})
    const [currentPresets,setCurrentPresets]=useState(0)
    const [isAvailable, setIsAvailable] = useState(true)

    useEffect(()=>{
        if(!message_camera) return

        const { labelPresets,currentPresets,isAvailable } = GetData(message_camera)
        setIsAvailable(isAvailable)
        setCurrentPresets(currentPresets)
        setlabelPresets(labelPresets)

    },[message_camera])


    return(
        <>
            {isAvailable ? ( message_camera ?
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
                <div className="grid grid-rows-[50px,1fr,1fr,auto] min-h-svh">
                    <div/>
                    <div className="flex bg-home_colors-Navbar/Selection_Bg mx-10 justify-center items-center">
                        <ImageSquare size={60} color="white" weight="thin"/>
                    </div>
                    <div className="grid grid-rows-[1fr,2fr]">
                        <div className="flex justify-center items-center gap-3">
                            <MagnifyingGlassPlus color="white"size={32}/>
                            <Slider className="w-[250px]" />
                        </div>
                        <div className="flex items-center justify-center gap-3">
                            <div className="flex flex-col w-32 h-32 border-[1px] rounded-2xl border-home_colors-Selected_Borders/text bg-home_colors-Navbar/Selection_Bg">
                                <div className="flex justify-center items-start py-1">
                                    <ArrowUp color="white" size={36}/>
                                </div>
                                <div className="flex justify-between px-1">
                                    <ArrowLeft color="white" size={36}/>
                                    <ArrowRight color="white" size={36}/>
                                </div>
                                <div className="flex items-end justify-center">
                                    <ArrowDown color="white" size={36}/>
                                </div>
                            </div>
                            <div className="flex flex-col gap-5">
                                <div className="">
                                    <ButtonPresets text={labelPresets[currentPresets.toString()] } onClick={()=>{navigate("/presetsCamera")}}/>
                                </div>
                                <div className="flex h-11 border-[1px] rounded-sm font-bold border-home_colors-Enabled_Channels bg-home_colors-Navbar/Selection_Bg text-home_colors-Enabled_Channels justify-center items-center">
                                    LIVE
                                </div>

                            </div>
                        </div>
                    </div>
                    <div className="flex flex-col justify-between items-center pb-3 gap-12 pt-3 px-5 w-full">
                    <Navbar selectedColor="video"/>
                    </div>
                </div>
            </div>
        </>
        
    )
} 