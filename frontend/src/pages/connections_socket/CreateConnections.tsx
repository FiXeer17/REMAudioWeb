import { ArrowLeft } from "@phosphor-icons/react";
import { Link } from "react-router-dom";
import { Input } from "@/components/ui/input_email";
import { Button } from "@/components/ui/button";




export default function CreateConnections(){
    return (
        <div className="grid grid-rows-[1fr,2fr,1fr]  min-h-svh">
            <div className="flex mt-9 mx-7 justify-between items-start"  >
                <Link to={"/Login"}>
                <ArrowLeft size={32} color="#FFFFFF" />
                </Link>
                <p className="flex text-white font-sans font-semibold flex-grow items-end justify-center">CREATE CONNECTIONS</p>
            </div>
            <div className="flex flex-col mx-12 gap-4 justify-start">
                <div className="flex flex-col gap-1 ">
                    <p className="text-white font-sans">MATRIX NAME</p>
                    <Input placeholder="name" className="w-full"/>
                </div>
                <div className="flex flex-col gap-1 ">
                    <p className="text-white font-sans">MATRIX IP</p>
                    <Input placeholder="ip" className="w-full"/>
                </div>
                <div className="flex flex-col mt-5 ">
                    <p className="text-white font-sans">MATRIX PORT</p>
                    <Input placeholder="port" className="w-1/3"/>
                </div>
                <div className="flex justify-center mt-7">
                    <Button className="text-black bg-white">
                        Continue
                    </Button> 
                </div>
            
            </div>
            <div/>

        </div>
    )
}