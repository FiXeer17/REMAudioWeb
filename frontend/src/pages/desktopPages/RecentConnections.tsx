import { ArrowLeft, Camera, HardDrive, Plus, Trash } from "@phosphor-icons/react";
import { Link,useNavigate,useLocation } from "react-router-dom";
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

export const RecentConnections=()=>{

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
                        console.log(value.status,value.request)
                        if (value.status===200){
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
            }=SwipeConnections(connections,"desktop")

    return(
        <div className="flex flex-col gap-14 pt-8">
            <div className="relative w-full h-14 flex items-center justify-center ">
                <Link to={"/Login"} className="absolute left-7" onClick={() => localStorage.removeItem("accessToken")}>
                    <ArrowLeft size={32} color="#FFFFFF" />
                </Link>
                <p className="text-white font-sans font-semibold text-center">RECENT CONNECTIONS</p>
                <Link to={"/createConnections"} className="absolute right-7">
                        <Plus color="#FFFFFF" size={32}/>
                </Link>
            </div>
        <div className="flex h-full justify-center items-start">
            <div className="grid grid-rows-4 grid-cols-2 gap-5 border-[1.5px] border-home_colors-Selected_Borders/text border-opacity-40 bg-home_colors-Navbar/Selection_Bg rounded-[60px] h-[550px] w-[710px] px-10 py-7"
             style={{
                transform: `translateX(${Offset}px)`,
                transition: Offset === 0 ? "transform 0.3s ease" : "none",
              }}
              onTouchStart={handleTouchStart}
              onTouchMove={handleTouchMove}
              onTouchEnd={handleTouchEnd}
              >
                
                {displayedConnections.map((element:Connection)=>(
                        <div className={`flex flex-col items-start justify-center text-white w-fit px-6 py-3 text-sm border-2 rounded-2xl ${
                            element.isLatestAudio
                            ? "bg-home_colors-Navbar/Selection_Bg border-home_colors-Selected_Borders/text"
                            : element.isLatestVideo ?"bg-home_colors-Navbar/Selection_Bg border-home_colors-Enabled_Channels":"bg-home_colors-Navbar/Selection_Bg border-home_colors-Border_Connections"
                        }`} key={element.ip}>
                            <div className="flex justify-between w-full items-center ">
                                <div className="flex items-center gap-3">
                                    {element.device_type==="matrix"? <HardDrive size={20}/>:<Camera size={20}/>}
                                    <p className="flex ">{element.name}</p>
                                </div>
                                <div className="bg-red-900 rounded-sm py-1 px-1 cursor-pointer" onClick={()=>handleRevome(element)}>
                                    <Trash size={22}/>
                                </div>
                            </div>
                            <div className="flex text-[12px] items-center">
                                <div className=" bg-home_colors-Navbar/Selection_Bg px-5 py-2 border-2 rounded-l-xl border-home_colors-Border_Connections ">{element.ip}</div>
                                <div className=" bg-home_colors-Navbar/Selection_Bg px-3 py-2 border-2 rounded-r-xl border-l-transparent border-home_colors-Border_Connections  ">{element.port}</div>
                                <Button size={"recentConnections"} className=" ml-2  bg-white text-black " onClick={()=>handleClick(element)}>Connect</Button>
                            </div>
                        </div>
                    ))}                   
            </div>
        </div>
        <Toaster/>
        </div>
    )
}