import { ArrowLeft,Plus,Trash,Camera,HardDrive } from "@phosphor-icons/react";
import { useNavigate,useLocation } from "react-router-dom";
import { Button } from "@/components/ui/button";
import { useEffect, useState } from "react";
import { Circle } from "@phosphor-icons/react";
import { SwipeConnections } from "@/lib/swipeConnections";
import { useConnections } from "@/lib/socket/ComponentUuid";
import { removeSocket, setSocket } from "@/lib/services";
import { toast, Toaster } from "sonner";



type Connection = {
    name:string,
    ip: string;
    port: number;
    device_type:string;
    isLatestAudio?: boolean;
    isLatestVideo?: boolean;
    
  };

  type RecentConnectionsProps = {
    isLoading?: boolean;
  };
export const RecentConnections=({isLoading=false}:RecentConnectionsProps)=>{
    const navigate=useNavigate()
    const location=useLocation()
    const [show] = useState<boolean>(() => location.state?.show);
    const [connections, setConnections] = useState<Connection[]>([]);
    const {uuid,sockets,triggerRedirect}=useConnections()
    
    useEffect(()=>{
        if (sockets==null){
           setConnections([]) 
        }else{
            setConnections(sockets)
        }
    },[sockets])

    useEffect(()=>{
        if(show)
            toast.error("Error with the socket, try again",{duration:2000})
    },[show])

    const handleClick=(element:Connection)=>{
        const fetchSetSocket=async ()=>{
            try {
                const headers={
                    uuid:uuid,
                    socket_name:element.name,
                    socket:`${element.ip}:${element.port}`,
                    device_type:element.device_type
                }
                const value = await setSocket(headers);
                if (value.status===200){
                    localStorage.setItem("showImage","false")
                    localStorage.setItem("urlSafe","")
                    if (element.device_type==="matrix")
                        return navigate("/homeAudio")
                    else
                        return navigate("/video")
                }
            } catch (error) {
                console.error("Error setting Socket:", error);
            }
        }
        fetchSetSocket()
    }

    const handleRevome=(element:Connection)=>{
        const fetchSetSocket=async ()=>{
            try {
                const headers={
                    uuid:uuid,
                    socket:`${element.ip}:${element.port}`,
                }
                const value = await removeSocket(headers);
                if (value.status===200 ){
                    const handleRedirect = async () => {
                        await triggerRedirect()
                        navigate("/uuidprovider")
                    }
                    handleRedirect()
                }
            } catch (error) {
                console.error("Error setting Socket:", error);
            }
        }
        fetchSetSocket()
    }

    const {
    currentSet:currentSet,
    displayedConnections:displayedConnections,
    connections:displayedAllConnections,
    offset:Offset,
    handleTouchStart:handleTouchStart,
    handleTouchMove:handleTouchMove,
    handleTouchEnd:handleTouchEnd
    }=SwipeConnections(connections,"mobile")
    

    return(
        <>
            {isLoading ? 
                <div className="absolute inset-0 backdrop-blur-sm flex justify-center items-center  bg-black/30 z-30">
                   <div className="w-10 h-10 border-4 border-white border-t-transparent rounded-full animate-spin"></div>
                </div>:<div className="absolute inset-0 z-10"></div>}
            <div className="absolute inset-0 bg-black z-20">
                <div className="grid grid-rows-6 min-h-svh">
                    <div className="flex justify-between mt-9 ml-7 mr-7">
                        <ArrowLeft size={32} color="#FFFFFF" onClick={() => {
                                if (location.state?.redirect === "settings") {
                                    navigate("/settings");
                                  } else {
                                    localStorage.removeItem("accessToken");
                                    navigate("/login");
                                  }}}/> 
                        <Plus color="#FFFFFF" size={32} onClick={()=>navigate("/createConnections",{state: { recent: true }})}/>
                    </div>
                    <div className="grid row-span-4 grid-rows-4 mx-10 gap-5 justify-center" style={{
                    transform: `translateX(${Offset}px)`,
                    transition: Offset === 0 ? "transform 0.3s ease" : "none",
                        }}
                        onTouchStart={handleTouchStart}
                        onTouchMove={handleTouchMove}
                        onTouchEnd={handleTouchEnd}
                        >
                    
                    {displayedConnections.map((element:Connection)=>(
                        <div className={`flex flex-col items-start justify-center text-white w-fit px-6 py-3 text-sm border-2 rounded-2xl max-w-[305px] ${
                            element.isLatestAudio
                            ? "bg-home_colors-Navbar/Selection_Bg border-home_colors-Selected_Borders/text": element.isLatestVideo 
                            ? "bg-home_colors-Navbar/Selection_Bg border-home_colors-Enabled_Channels":
                                "bg-home_colors-Navbar/Selection_Bg border-home_colors-Border_Connections"
                            
                        }`} key={element.ip}>
                            <div className="flex justify-between w-full items-center  ">
                                <div className="flex items-center gap-3">
                                    {element.device_type==="matrix"? <HardDrive size={20}/>:<Camera size={20}/>}
                                    <p className="flex ">{element.name}</p>
                                </div>
                                <div className="bg-red-900 rounded-sm py-1 px-1 cursor-pointer" onClick={()=>handleRevome(element)}>
                                    <Trash size={22}/>
                                </div>
                            </div>
                            <div className="flex text-[12px] items-center ">
                                <div className=" bg-home_colors-Navbar/Selection_Bg px-5 py-2 border-2 rounded-l-xl border-home_colors-Border_Connections max-w-[107px] select-none">{element.ip}</div>
                                <div className=" bg-home_colors-Navbar/Selection_Bg px-3 py-2 border-2 rounded-r-xl border-l-transparent border-home_colors-Border_Connections select-none">{element.port}</div>
                                <Button size={"recentConnections"} className=" ml-2  bg-white text-black " onClick={()=>handleClick(element)}>Connect</Button>
                            </div>
                        </div>
                    ))}
                    </div>
                    
                    {displayedAllConnections.length > 1 && (
                        <div className="flex justify-center gap-2 mt-4">
                            {displayedAllConnections.map((_, index) => (
                            index === currentSet ? (
                                <Circle key={index} size={12} color="#ffffff" weight="fill" />
                            ) : (
                                <Circle key={index} size={12} color="#ffffff" />
                            )
                            ))}
                        </div>
                        )}
                    <Toaster/>
                </div>
            </div>
        </>
    )
}