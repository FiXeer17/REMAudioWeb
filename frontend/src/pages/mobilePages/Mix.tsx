import { Badge } from "@/components/ui/badge"
import Navbar from "@/components/ui/navbar"
import SocketContext from "@/lib/socket/context";
import { GetData } from "@/lib/WebSocketData";
import { useContext, useEffect, useState } from "react";
import { useNavigate } from "react-router-dom";

export const Mix = () =>{
    const navigate = useNavigate()
    const {socket,message_matrix,matrix_status} = useContext(SocketContext).socketState
    const [ mix_map,setMix_Map ] = useState<{[key: string]: boolean}>()
    const [labelChannelInput,setLabelChannelInput]=useState<{[key: string]: string;}>({})
    const [labelChannelOutput,setLabelChannelOutput]=useState<{[key: string]: string;}>({})
    const array = Array.from({ length: 8 }, (_, i) => i + 1);

    useEffect(()=>{
        if(matrix_status==="disconnected"){
            navigate("/settings")
        }
    },[matrix_status])


    useEffect(()=>{
        if (!message_matrix) return
        const { mix_map,labelChannelsInput,labelChannelsOutput } = GetData(message_matrix);
        setMix_Map(mix_map)
        setLabelChannelInput(labelChannelsInput)
        setLabelChannelOutput(labelChannelsOutput)

        },[message_matrix])

    const handleSetMix=(input:string , output:string,value:boolean)=>{
        if (!mix_map) return
            for (const key in mix_map) {
                const [col, row] = key.slice(1, -1).split(",").map(Number);
                
                if (col.toString()===input && mix_map[key]===true) {
                    const data = { "section": "mix_map", "channel": row.toString(), "index": col.toString(), "value": "false" };
                    socket?.send(JSON.stringify(data));
                }
                }
        const data={"section":"mix_map","channel":output,"index":input,"value":value.toString()}

        socket?.send(JSON.stringify(data))

    }

    return(
        <div className="grid grid-rows-[50px,1fr,auto] min-h-svh relative">
            <div/>
            <div className="flex flex-1 px-7 pb-5 overflow-hidden relative pt-5">
                <Badge className="absolute left-12 top-3 transform -translate-x-1/2">
                    MIXING
                </Badge>
                <div className="flex  h-full w-full bg-home_colors-Navbar/Selection_Bg rounded-2xl px-5 py-10 justify-center items-center">
                    
                    <div className="flex w-full max-w-[400px] aspect-square ">
                        <div className="flex flex-col " style={{ height: 'calc(100% - 20px)', marginTop: '26px' }}>
                        {array.map((num) => {
                            const label = labelChannelOutput[num];
                            return (
                                <div key={num} className="w-full h-full pr-2">
                                {label ? (
                                    label.length > 3 ? (
                                    <div className="relative w-full overflow-hidden">
                                        <div className="text-white text-sm font-bold whitespace-nowrap items-center justify-end animate-marquee">
                                        {label}
                                        </div>
                                    </div>
                                    ) : (
                                    <div className="flex text-white text-center items-center text-sm font-bold h-full">{label}</div>
                                    )
                                ) : null}
                                </div>
                            );
                            })}

                        </div>
                        <div className="flex flex-col flex-1 w-full">
                            <div className="grid grid-cols-8 w-full h-[20px] items-end mb-2 gap-1">
                                {array.map((num) => {
                                    const label = labelChannelInput[num];
                                    return (
                                        <div key={num} className="w-full h-full ">
                                        {label ? (
                                            label.length > 3 ? (
                                            <div className="relative w-full overflow-hidden">
                                                <div className="text-white text-sm font-bold whitespace-nowrap  animate-marquee">
                                                {label}
                                                </div>
                                            </div>
                                            ) : (
                                            <div className="flex text-white text-center text-sm font-bold h-full">{label}</div>
                                            )
                                        ) : null}
                                        </div>
                                    );
                                })}
                            </div>
                            <div className="grid grid-cols-8 grid-rows-8 w-full h-full flex-1 gap-0">
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
            </div>

            <div className="flex items-center pb-3 pt-3">
                <Navbar selectedColor="settings"/>
            </div>
      </div>
    )
}