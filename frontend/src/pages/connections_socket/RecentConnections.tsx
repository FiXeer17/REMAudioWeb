import { ArrowLeft } from "@phosphor-icons/react";
import { Link } from "react-router-dom";


export default function RecentConnections(){
    return(
        <div className="grid grid-rows-5 min-h-svh">
            <div className=" mt-9 ml-7">
                <Link to={"/Login"}>
                <ArrowLeft size={32} color="#FFFFFF" />
                </Link>
            </div>
            <div className="grid row-span-3 grid-rows-4 mx-12">
                <div className=" flex flex-col bg-home_colors-Navbar/Selection_Bg border-2 items-start justify-center text-white text-sm border-home_colors-Border_Connections rounded-2xl">
                    <p className="flex ml-6 ">M. SALA MATRIMONI</p>
                    <div className="flex ml-6 text-[12px] w-full">
                        <div className=" bg-home_colors-Navbar/Selection_Bg px-5 py-2 border-2 rounded-l-xl border-home_colors-Border_Connections ">192.168.35.3</div>
                        <div className=" bg-home_colors-Navbar/Selection_Bg px-3 py-2 border-2 rounded-r-xl border-l-transparent border-home_colors-Border_Connections  ">8234</div>
                    </div>
                </div>
                <div></div>
                <div></div>
                <div></div>
            </div>
            <div></div>
        </div>
    )
}