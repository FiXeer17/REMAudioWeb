import { ArrowLeft } from "@phosphor-icons/react";
import { Link } from "react-router-dom";


export default function NewConnetions(){

    return (
        <div className="grid grid-rows-[auto,1fr] min-h-svh">
            <div className=" mt-9 ml-7">
                <Link to={"/Login"}>
                <ArrowLeft size={32} color="#FFFFFF" />
                </Link>
            </div>
            <div className="flex justify-center items-center ">
        <div className="flex justify-center w-screen aspect-square rounded-full border-2 blur-sm border-home_colors-Selected_Borders/text  text-home_colors-Enabled_Channels">
            
        </div>
        </div>
            
        </div>
    )
}