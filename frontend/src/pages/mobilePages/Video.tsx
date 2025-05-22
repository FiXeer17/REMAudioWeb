import Navbar from "@/components/ui/navbar"
import SocketContext from "@/lib/socket/context";
import { GetData } from "@/lib/WebSocketData";
import { useContext, useEffect, useRef, useState } from "react";
import { RecentConnections } from "./RecentConnections";
import { Clock, ArrowDown, ArrowLeft, ArrowUp, ArrowRight, ArrowsClockwise, Minus, Plus } from "@phosphor-icons/react";
import { ButtonPresets } from "@/components/ui/button_presets";
import { useNavigate } from "react-router-dom";
import { useClickAndHold, IntensityType, MovementDirection } from "@/lib/handleMovement";
import { Input } from "@/components/ui/input_email";
import { Button } from "@/components/ui/button";
import { useForm } from "react-hook-form";
import { useConnections } from "@/lib/socket/ComponentUuid";
import { toast, Toaster } from "sonner";

type FormFields = {
    port: string;
  };

export const Video = () => {
    const navigate = useNavigate()
    const {triggerRedirect}=useConnections()
    const { register:connect, handleSubmit } = useForm<FormFields>();
    const { socket, message_camera, matrix_status, camera_status } = useContext(SocketContext).socketState
    const [labelPresets, setlabelPresets] = useState<{ [key: string]: string; }>({})
    const [currentPresets, setCurrentPresets] = useState(0)
    const [showImage, setShowImage] = useState(() => {
        const saved = localStorage.getItem("showImage");
        return saved === "true"; 
    });
    const [urlSafe,setUrlSafe]=useState("")
    const [isAvailable, setIsAvailable] = useState(true)
    const [color,setColor]= useState("")

    const upArrowRef = useRef<HTMLDivElement>(null);
    const rightArrowRef = useRef<HTMLDivElement>(null);
    const leftArrowRef = useRef<HTMLDivElement>(null);
    const downArrowRef = useRef<HTMLDivElement>(null);

    useEffect(() => {
        if (!isAvailable || message_camera) return;

        const timeout = setTimeout(() => {
            navigate("/uuidprovider");
        }, 10000);

        return () => clearTimeout(timeout);
    }, [isAvailable, message_camera]);

    useEffect(() => {
        if (camera_status === "disconnected") 
            setShowImage(false);
        if (camera_status === "disconnected" && matrix_status === "connected")
            navigate("/homeAudio")
    }, [camera_status])

    useEffect(() => {
        if (!message_camera) return

        const { labelPresets, currentPresets, isAvailable } = GetData(message_camera)
        setIsAvailable(isAvailable)
        setCurrentPresets(currentPresets)
        setlabelPresets(labelPresets)
    }, [message_camera])

    const handleErrorImage=()=>{
        setShowImage(false)
        localStorage.setItem("showImage","false")
        toast.error("Error connecting with camera")
    }
    
    const upControl = useClickAndHold({
        onHold: (intensity: IntensityType) => handleMovement("up", intensity),
        onSlowClick: () => handleMovement("up", "slow"),
    });

    const rightControl = useClickAndHold({
        onHold: (intensity: IntensityType) => handleMovement("right", intensity),
        onSlowClick: () => handleMovement("right", "slow"),
    });

    const leftControl = useClickAndHold({
        onHold: (intensity: IntensityType) => handleMovement("left", intensity),
        onSlowClick: () => handleMovement("left", "slow"),
    });

    const downControl = useClickAndHold({
        onHold: (intensity: IntensityType) => handleMovement("down", intensity),
        onSlowClick: () => handleMovement("down", "slow"),
    });

    const handleMouseDown = (control: any) => (e: React.MouseEvent | React.TouchEvent) => {
        e.preventDefault();
        control.handleAction(true);
    };

    const handleMouseUp = (control: any) => (e: React.MouseEvent | React.TouchEvent) => {
        setColor("")
        e.preventDefault();
        control.handleAction(false);
    };


    const handleMovement = (direction: MovementDirection|"home", intensity: IntensityType = "slow") => {
        setColor(direction)
        if (direction==="home"){
          const data = {"section": "move_camera", "direction": direction};
          socket?.send(JSON.stringify(data));
        }else{
          const data = {"section": "move_camera", "direction": direction, "velocity": intensity};
          socket?.send(JSON.stringify(data));
        }
        
      }
    
    const handleZoomDown = (type:string) =>{
        if (type==="plus"){
            setColor("plus")
            const data = {"section": "zoom_tele"};
            socket?.send(JSON.stringify(data));
        }else {
            setColor("minus")
            const data = {"section": "zoom_wide"};
            socket?.send(JSON.stringify(data));
        }
    }
    const handleZoomUp = () =>{
        setColor("")
        const data = {"section": "zoom_stop"};
        socket?.send(JSON.stringify(data));
    }
        
    const handleConnect = async ({ port }: FormFields) => {
        const updatedSockets = await triggerRedirect();

        const latestVideoDevice = updatedSockets?.find(updatedSockets => updatedSockets.isLatestVideo);

        const base64 = btoa(`${latestVideoDevice?.ip}:${port}`);
        setShowImage(true)
        localStorage.setItem("showImage","true")
        setUrlSafe(base64.replace(/\+/g, '-').replace(/\//g, '_').replace(/=+$/, ''))
        console.log(`${latestVideoDevice?.ip}:${port}`)
        console.log(base64.replace(/\+/g, '-').replace(/\//g, '_').replace(/=+$/, ''))
        

      };

    return (
        <div className="relative min-h-svh">
            <div className="absolute inset-0 bg-black z-0">
                <div className="grid grid-rows-[80px,1fr,1fr,auto] justify-center min-h-svh">
                    <div className="flex items-center justify-center">
                        <ButtonPresets text={labelPresets[currentPresets.toString()]} onClick={() => { navigate("/presetsCamera") }} />
                    </div>
                    <div className="flex flex-col gap-3 bg-home_colors-Navbar/Selection_Bg mx-10 w-[295px] h-[256px] justify-center items-center">
                        {showImage ? <img className="w-full h-full object-cover" src={`http://localhost/stream?a=MTkyLjE2OC44OC4yNTI6ODU1NA`} onError={()=>handleErrorImage()}/>
                            :
                            <>
                            <p className="text-white font-bold text-sm">RTSP PORT</p>
                            <div className="flex gap-3">
                                <form onSubmit={handleSubmit(handleConnect)} className="flex gap-3">
                                    <Input 
                                        placeholder="port" 
                                        className="w-20" 
                                        autoComplete="off"
                                        {...connect("port", { required: true })} 
                                    />
                                    <Button className="text-black bg-white w-16" type="submit">
                                        Connect
                                    </Button>
                                </form>
                            </div>
                            </>}
                    </div>
                    <div className="grid grid-rows-[1fr,2fr]">
                        <div className="flex justify-center items-center gap-3">
                            <div className={`border-[1px] ${color==="minus"?"border-home_colors-Selected_Borders/text":"border-home_colors-Similar_White"} rounded-full cursor-pointer`}
                                onContextMenu={(e) => e.preventDefault()}
                                onTouchStart={() => { handleZoomDown("minus") }}
                                onTouchEnd={() => { handleZoomUp() }}>
                                <Minus size={22} color={color==="minus"?"#007AFF":"white"} className="m-1" />
                            </div>
                            <div className={`border-[1px] ${color==="plus"?"border-home_colors-Selected_Borders/text":"border-home_colors-Similar_White"} rounded-full cursor-pointer`}
                                onContextMenu={(e) => e.preventDefault()}
                                onTouchStart={() => { handleZoomDown("plus") }}
                                onTouchEnd={() => { handleZoomUp() }}>
                                <Plus size={22} color={color==="plus"?"#007AFF":"white"} className="m-1"/>
                            </div>
                        </div>
                        <div className="flex items-center justify-center">
                            <div className="flex flex-col w-32 h-32 border-[1px] select-none items-center rounded-2xl border-home_colors-Selected_Borders/text bg-home_colors-Navbar/Selection_Bg">
                                <div 
                                    ref={upArrowRef} 
                                    className="flex justify-center w-fit items-start cursor-pointer py-[6px]"
                                    onMouseDown={handleMouseDown(upControl)}
                                    onMouseUp={handleMouseUp(upControl)}
                                    onMouseLeave={handleMouseUp(upControl)}
                                    onTouchStart={handleMouseDown(upControl)}
                                    onTouchEnd={handleMouseUp(upControl)}
                                    onContextMenu={(e) => e.preventDefault()}
                                >
                                    <ArrowUp color={color==="up"?"#007AFF":"white"} size={36} weight="bold" />
                                </div>
                                <div className="flex justify-between px-1 w-full items-center">
                                    <div 
                                        className="cursor-pointer"
                                        ref={leftArrowRef}
                                        onMouseDown={handleMouseDown(leftControl)}
                                        onMouseUp={handleMouseUp(leftControl)}
                                        onMouseLeave={handleMouseUp(leftControl)}
                                        onTouchStart={handleMouseDown(leftControl)}
                                        onTouchEnd={handleMouseUp(leftControl)} 
                                        onContextMenu={(e) => e.preventDefault()}
                                    >
                                        <ArrowLeft color={color==="left"?"#007AFF":"white"} size={36} weight="bold" />
                                    </div>
                                    <div 
                                        onMouseDown={()=>handleMovement("home")}
                                        onMouseUp={()=>setColor("")}
                                        onTouchStart={()=>handleMovement("home")}
                                        onTouchEnd={()=>setColor("")} 
                                        onContextMenu={(e) => e.preventDefault()}>
                                        <ArrowsClockwise color={color==="home"?"#007AFF":"white"} className="cursor-pointer" size={30} weight="bold" />
                                    </div>
                                    <div 
                                        className="cursor-pointer"
                                        ref={rightArrowRef}
                                        onMouseDown={handleMouseDown(rightControl)}
                                        onMouseUp={handleMouseUp(rightControl)}
                                        onMouseLeave={handleMouseUp(rightControl)}
                                        onTouchStart={handleMouseDown(rightControl)}
                                        onTouchEnd={handleMouseUp(rightControl)}
                                        onContextMenu={(e) => e.preventDefault()}
                                    >
                                        <ArrowRight color={color==="right"?"#007AFF":"white"} size={36} weight="bold" />
                                    </div>
                                </div>
                                <div 
                                    ref={downArrowRef} 
                                    className="flex items-end cursor-pointer w-fit justify-center"
                                    onMouseDown={handleMouseDown(downControl)}
                                    onMouseUp={handleMouseUp(downControl)}
                                    onMouseLeave={handleMouseUp(downControl)}
                                    onTouchStart={handleMouseDown(downControl)}
                                    onTouchEnd={handleMouseUp(downControl)}
                                    onContextMenu={(e) => e.preventDefault()}
                                >
                                    <ArrowDown color={color==="down"?"#007AFF":"white"} size={36} weight="bold" />
                                </div>
                            </div>
                        </div>
                    </div>
                    <div className="h-16 mb-3"></div>
                    
                </div>
            </div>
            
            {!isAvailable && (
                <div className="absolute inset-0 bottom-[calc(3rem+12px)] backdrop-blur-sm flex justify-center items-center bg-black/30 z-10">
                    <div className="flex border-yellow-500 border-2 rounded-sm px-3 py-3 text-yellow-500 text-sm font-bold gap-2">
                        <div className="mt-1">
                            <Clock weight="bold"></Clock>
                        </div>
                        <div>
                            <p>Camera Unvailable</p>
                            <p>Please wait...</p>
                        </div>
                    </div>
                </div>
            )}
            
            {isAvailable && !message_camera && (
                <div className="absolute inset-0 bottom-[calc(3rem+12px)] z-10">
                    <RecentConnections isLoading={true} />
                </div>
            )}

            <div className="absolute bottom-0 left-0 right-0 z-20">
                <div className="flex flex-col justify-between items-center pb-3 gap-12 pt-3 px-5 w-full">
                    
                    <Navbar selectedColor="video" />
                </div>
                <Toaster/>
            </div>
        </div>
    )
}