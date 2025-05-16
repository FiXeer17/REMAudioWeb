import Navbar from "@/components/ui/navbar"
import SocketContext from "@/lib/socket/context";
import { GetData } from "@/lib/WebSocketData";
import { useContext, useEffect, useRef, useState } from "react";
import { RecentConnections } from "./RecentConnections";
import { Clock, ImageSquare, MagnifyingGlassPlus, ArrowDown, ArrowLeft, ArrowUp, ArrowRight, ArrowsClockwise } from "@phosphor-icons/react";
import { Slider } from "@/components/ui/slider";
import { ButtonPresets } from "@/components/ui/button_presets";
import { useNavigate } from "react-router-dom";
import { WideTeleButton } from "@/components/ui/wide_tele";
import { useClickAndHold, IntensityType, MovementDirection } from "@/lib/handleMovement";

export const Video = () => {
    const navigate = useNavigate()
    const { socket, message_camera, matrix_status, camera_status } = useContext(SocketContext).socketState
    const [labelPresets, setlabelPresets] = useState<{ [key: string]: string; }>({})
    const [WideTele, setWideTele] = useState<"WIDE" | "TELE">("WIDE")
    const [currentPresets, setCurrentPresets] = useState(0)
    const [isAvailable, setIsAvailable] = useState(true)

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
        e.preventDefault();
        control.handleAction(false);
    };


    const handleMovement = (direction: MovementDirection, intensity: IntensityType = "slow") => {
        const data = {"section": "move_camera", "direction": direction, "velocity": intensity};
        socket?.send(JSON.stringify(data));
    }


    return (
        <>
            {isAvailable ? (message_camera ?
                <div className="absolute inset-0 z-10"></div> : <RecentConnections isLoading={true} />
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
                <div className="grid grid-rows-[80px,1fr,1fr,auto] min-h-svh">
                    <div className="flex items-center justify-center">
                        <ButtonPresets text={labelPresets[currentPresets.toString()]} onClick={() => { navigate("/presetsCamera") }} />
                    </div>
                    <div className="flex bg-home_colors-Navbar/Selection_Bg mx-10 justify-center items-center">
                        <ImageSquare size={60} color="white" weight="thin" />
                    </div>
                    <div className="grid grid-rows-[0.5fr_1fr_2fr]">
                        <div className="flex items-center justify-center">
                            <WideTeleButton onChange={setWideTele} />
                        </div>
                        <div className="flex justify-center items-center gap-3">
                            <MagnifyingGlassPlus color="white" size={32} />
                            <Slider className="w-[250px]" />
                        </div>
                        <div className="flex items-center justify-center">
                            <div className="flex flex-col w-32 h-32 border-[1px] rounded-2xl border-home_colors-Selected_Borders/text bg-home_colors-Navbar/Selection_Bg">
                                <div 
                                    ref={upArrowRef} 
                                    className="flex justify-center items-start py-[6px]"
                                    onMouseDown={handleMouseDown(upControl)}
                                    onMouseUp={handleMouseUp(upControl)}
                                    onMouseLeave={handleMouseUp(upControl)}
                                    onTouchStart={handleMouseDown(upControl)}
                                    onTouchEnd={handleMouseUp(upControl)}
                                    onContextMenu={(e) => e.preventDefault()}
                                >
                                    <ArrowUp color="white" size={36} weight="bold" />
                                </div>
                                <div className="flex justify-between px-1 items-center">
                                    <div 
                                        ref={leftArrowRef}
                                        onMouseDown={handleMouseDown(leftControl)}
                                        onMouseUp={handleMouseUp(leftControl)}
                                        onMouseLeave={handleMouseUp(leftControl)}
                                        onTouchStart={handleMouseDown(leftControl)}
                                        onTouchEnd={handleMouseUp(leftControl)} 
                                        onContextMenu={(e) => e.preventDefault()}
                                    >
                                        <ArrowLeft color="white" size={36} weight="bold" />
                                    </div>
                                    <ArrowsClockwise color="white" size={30} weight="bold" />
                                    <div 
                                        ref={rightArrowRef}
                                        onMouseDown={handleMouseDown(rightControl)}
                                        onMouseUp={handleMouseUp(rightControl)}
                                        onMouseLeave={handleMouseUp(rightControl)}
                                        onTouchStart={handleMouseDown(rightControl)}
                                        onTouchEnd={handleMouseUp(rightControl)}
                                        onContextMenu={(e) => e.preventDefault()}
                                    >
                                        <ArrowRight color="white" size={36} weight="bold" />
                                    </div>
                                </div>
                                <div 
                                    ref={downArrowRef} 
                                    className="flex items-end justify-center"
                                    onMouseDown={handleMouseDown(downControl)}
                                    onMouseUp={handleMouseUp(downControl)}
                                    onMouseLeave={handleMouseUp(downControl)}
                                    onTouchStart={handleMouseDown(downControl)}
                                    onTouchEnd={handleMouseUp(downControl)}
                                    onContextMenu={(e) => e.preventDefault()}
                                >
                                    <ArrowDown color="white" size={36} weight="bold" />
                                </div>
                            </div>
                        </div>
                    </div>
                    <div className="flex flex-col justify-between items-center pb-3 gap-12 pt-3 px-5 w-full">
                        <Navbar selectedColor="video" />
                    </div>
                </div>
            </div>
        </>
    )
}