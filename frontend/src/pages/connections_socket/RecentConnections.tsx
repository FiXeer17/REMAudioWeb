import { ArrowLeft } from "@phosphor-icons/react";
import { Link,useNavigate,useLocation } from "react-router-dom";
import { Button } from "@/components/ui/button";
import { useEffect, useState } from "react";
import { Circle } from "@phosphor-icons/react";
import { SwipeConnections } from "@/lib/swipeConnections";
import { useConnections } from "@/lib/socket/ComponentUuid";
import { setSocket } from "@/lib/services";
import { toast, Toaster } from "sonner";



type Connection = {
    name:string,
    ip: string;
    port: number;
    isLatest?: boolean;
  };

  type RecentConnectionsProps = {
    isLoading?: boolean;
  };
export const RecentConnections=({isLoading=false}:RecentConnectionsProps)=>{
    const navigate=useNavigate()
    const location=useLocation()
    const [show] = useState<boolean>(() => location.state?.show);
    const [connections, setConnections] = useState<Connection[]>([]);
    const {uuid,sockets}=useConnections()
    
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
                    device_type:"audio"
                }
                const value = await setSocket(headers);
                if (value.status===200){
                    return navigate("/homeAudio")
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
                <div className="grid grid-rows-5 min-h-svh">
                    <div className=" mt-9 ml-7">
                        <Link to={"/Login"} onClick={() => localStorage.removeItem("accessToken")}>
                            <ArrowLeft size={32} color="#FFFFFF" />
                        </Link>
                    </div>
                    <div className="grid row-span-3 grid-rows-4 mx-10 gap-4" style={{
                    transform: `translateX(${Offset}px)`,
                    transition: Offset === 0 ? "transform 0.3s ease" : "none",
                }}
                onTouchStart={handleTouchStart}
                onTouchMove={handleTouchMove}
                onTouchEnd={handleTouchEnd}
                >
                    
                    {displayedConnections.map((element:Connection)=>(
                        <div className={`flex flex-col items-start justify-center text-white text-sm border-2 rounded-2xl ${
                            element.isLatest
                            ? "bg-home_colors-Navbar/Selection_Bg border-home_colors-Selected_Borders/text"
                            : "bg-home_colors-Navbar/Selection_Bg border-home_colors-Border_Connections"
                        }`} key={element.ip}>
                            <p className="flex ml-6 ">{element.name}</p>
                            <div className="flex ml-6 text-[12px] items-center">
                                <div className=" bg-home_colors-Navbar/Selection_Bg px-5 py-2 border-2 rounded-l-xl border-home_colors-Border_Connections ">{element.ip}</div>
                                <div className=" bg-home_colors-Navbar/Selection_Bg px-3 py-2 border-2 rounded-r-xl border-l-transparent border-home_colors-Border_Connections  ">{element.port}</div>
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