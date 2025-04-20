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

export default function RecentConnections(){

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

          

        const handleClick=(element:Connection)=>{
                const fetchSetSocket=async ()=>{
                    try {
                        const headers={
                            uuid:uuid,
                            socket_name:element.name,
                            socket:`${element.ip}:${element.port}`
                            
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
            }=SwipeConnections(connections,"desktop")

    return(
        <div className="flex flex-col gap-14 pt-8">
            <div className="relative w-full h-14 flex items-center justify-center ">
                <Link to={"/Login"} className="absolute left-7">
                    <ArrowLeft size={32} color="#FFFFFF" />
                </Link>
                <p className="text-white font-sans font-semibold text-center">RECENT CONNECTIONS</p>
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
                    <div className={`flex flex-col items-start justify-center text-white text-sm border-[1.5px] rounded-2xl ${
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
        </div>
        </div>
    )
}