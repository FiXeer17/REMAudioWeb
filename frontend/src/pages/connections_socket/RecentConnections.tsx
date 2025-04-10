import { ArrowLeft } from "@phosphor-icons/react";
import { Link,useNavigate } from "react-router-dom";
import { Button } from "@/components/ui/button";
import { useEffect, useState } from "react";
import { Circle } from "@phosphor-icons/react";
import { SwipeConnections } from "@/lib/swipeConnections";
import { useSockets,useUUID } from "@/lib/socket/ComponentUuid";
import { setSocket } from "@/lib/services";



type Connection = {
    name:string,
    ip: string;
    port: number;
  };

export default function RecentConnections(){
    const navigate=useNavigate()
    const [connections, setConnections] = useState<Connection[]>([]);
    const {sockets}=useSockets()
    const {uuid}=useUUID()
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
    }=SwipeConnections(connections)
    

    return(
        <div className="grid grid-rows-5 min-h-svh">
            <div className=" mt-9 ml-7">
                <Link to={"/Login"}>
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
                <div className=" flex flex-col bg-home_colors-Navbar/Selection_Bg border-2 items-start justify-center text-white text-sm border-home_colors-Border_Connections rounded-2xl" key={element.ip}>
                    <p className="flex ml-6 ">{element.name}</p>
                    <div className="flex ml-6 text-[12px] items-center">
                        <div className=" bg-home_colors-Navbar/Selection_Bg px-5 py-2 border-2 rounded-l-xl border-home_colors-Border_Connections ">{element.ip}</div>
                        <div className=" bg-home_colors-Navbar/Selection_Bg px-3 py-2 border-2 rounded-r-xl border-l-transparent border-home_colors-Border_Connections  ">{element.port}</div>
                        <Button size={"recentConnections"} className=" ml-2  bg-white text-black " onClick={()=>handleClick(element)}>Connect</Button>
                    </div>
                </div>
            ))}
            </div>
            
            <div className="flex items-center justify-center ">
                {displayedAllConnections.map((_,index)=>(
                    index===currentSet ?
                    (<Circle key={index} size={12} color="#ffffff" weight="fill"/>) :
                    (<Circle key={index} size={12} color="#ffffff"  />)
                ))}

            </div>
            
        </div>
    )
}