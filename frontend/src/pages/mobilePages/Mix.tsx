import { Badge } from "@/components/ui/badge"
import Navbar from "@/components/ui/navbar"
import SocketContext from "@/lib/socket/context";
import { GetData } from "@/lib/WebSocketData";
import { useContext, useEffect, useState } from "react";

export const Mix = () =>{
    const {socket,message_matrix} = useContext(SocketContext).socketState
    const [ mix_map,setMix_Map ] = useState<{[key: string]: boolean}>()
    
    
    useEffect(()=>{
        if (!message_matrix) return
        const { mix_map } = GetData(message_matrix);
        setMix_Map(mix_map)
        },[message_matrix])

    const handleSetMix=(input:string , output:string,value:boolean)=>{
        const data={"section":"mix_map","channel":output,"index":input,"value":value.toString()}
        console.log(data)
        socket?.send(JSON.stringify(data))
        console.log(input,output)
    }

    return(
        <div className="grid grid-rows-[50px,1fr,auto] min-h-svh relative">
            <div/>
            <div className="flex flex-1 px-7 pb-5 overflow-hidden relative pt-5">
                <Badge className="absolute left-12 top-3 transform -translate-x-1/2">
                    MIXING
                </Badge>
                <div className="flex h-full w-full bg-home_colors-Navbar/Selection_Bg rounded-2xl px-10 py-10 justify-center items-center">
                    <div className="w-full aspect-square">
                        <div className="grid grid-cols-8 grid-rows-8 w-full h-full gap-0">
                            {mix_map && Object.entries(mix_map).map(([key, value]) => {
                            const [col, row] = key.slice(1, -1).split(",").map(Number);
                            return (
                                <div key={key} className={`flex items-center justify-center border-[1px] border-white cursor-pointer`} style={{gridRow: row,gridColumn: col,}} onClick={()=>{handleSetMix(col.toString(),row.toString(),!value)}}>
                                        {value?<p className="text-white font-bold">X</p>:null}
                                </div>
                            );
                            })}
                        </div>
                    </div>
                </div>
            </div>

            <div className="flex items-center pb-3 pt-3">
                <Navbar selectedColor="settings"/>
            </div>
      </div>
    )
}