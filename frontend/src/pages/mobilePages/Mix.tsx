import { Badge } from "@/components/ui/badge"
import Navbar from "@/components/ui/navbar"
import { useState } from "react";

export const Mix = () =>{
    let mix_map= {"(1,4)": false,"(6,5)": false,"(8,3)": false,"(4,6)": false,"(7,6)": false,"(8,2)": false,"(3,4)": false,"(6,1)": false,"(8,1)": false,"(2,7)": false,"(4,8)": false,"(6,7)": false,"(4,3)": false,"(1,7)": false,"(5,1)": false,"(4,1)": false,"(5,8)": false,"(6,2)": false,"(6,3)": false,"(8,8)": true,"(2,2)": true,"(5,4)": false,"(7,3)": false,"(6,4)": false,"(2,4)": false,"(4,2)": false,"(5,7)": false,"(7,1)": false,"(4,7)": false,"(5,3)": false,"(5,5)": true,"(3,3)": true,"(2,5)": false,"(5,2)": false,"(3,5)": false,"(3,8)": false,"(5,6)": false,"(1,1)": true,"(1,2)": false,"(8,5)": false,"(7,2)": false,"(4,4)": true,"(7,5)": false,"(3,7)": false,"(2,6)": false,"(8,7)": false,"(7,8)": false,"(7,4)": false,"(6,6)": true,"(2,8)": false,"(2,3)": false,"(1,6)": false,"(3,6)": false,"(6,8)": false,"(8,6)": false,"(8,4)": false,"(1,3)": false,"(3,2)": false,"(2,1)": false,"(7,7)": true,"(4,5)": false,"(1,5)": false,"(3,1)": false,"(1,8)": false}
     /* 
    const [griglia, setGriglia] = useState<string[][]>();
      const handleClick = (riga: number, colonna: number) => {
        setGriglia(prev =>
          prev.map((r, i) =>
            r.map((val, j) =>
              i === riga && j === colonna ? (val === "X" ? "" : "X") : val
            )
          )
        );
      };*/
    return(
        <div className="grid grid-rows-[50px,1fr,auto] min-h-svh relative">
            <div/>
            <div className="flex flex-1 px-7 pb-5 overflow-hidden relative pt-5">
                <Badge className="absolute left-12 top-3 transform -translate-x-1/2">
                  MIXING
                </Badge>
                <div className="flex h-full w-full bg-home_colors-Navbar/Selection_Bg rounded-2xl px-10 py-10">
                    <div className="grid grid-cols-8 my-4 gap-0 w-full h-full">
                        {Object.entries(mix_map).map(([key,value],index)=>{
                            const [row, col] = key.slice(1, -1)  .split(",") .map(Number)
                            console.log(row,col)
                            return(
                            <div key={key} className="flex border-white border-[1px] spect-w-1 aspect-h-1 items-center justify-center" style={{
                                gridRow: col,    
                                gridColumn: row  
                              }}>
                                 
                            </div>
                            )
                        })}
                    </div>
                </div>
            </div>
            <div className="flex items-center pb-3 pt-3">
                <Navbar selectedColor="settings"/>
            </div>
      </div>
    )
}