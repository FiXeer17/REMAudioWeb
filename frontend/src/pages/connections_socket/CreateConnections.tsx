import { ArrowLeft } from "@phosphor-icons/react";
import { Link } from "react-router-dom";
import { Input } from "@/components/ui/input_email";
import { Button } from "@/components/ui/button";




export default function CreateConnections(){
    return (
        <div className="grid grid-rows-[auto,1fr,1fr]  min-h-svh">
            <div className="flex mt-9 mx-7 justify-between items-center"  >
                <Link to={"/Login"}>
                <ArrowLeft size={32} color="#FFFFFF" />
                </Link>
                <p className="flex text-white font-sans font-semibold flex-grow items-end justify-center">CREATE CONNECTIONS</p>
            </div>
            <div className="flex flex-col mx-12  justify-end gap-4 ">
                <div className="flex flex-col gap-1">
                    <p className="text-white font-sans">MATRIX NAME</p>
                    <Input placeholder="name" className="w-full"/>
                </div>
                <div className="flex flex-col gap-1">
                    <p className="text-white font-sans">MATRIX IP</p>
                    <Input placeholder="ip" className="w-full"/>
                </div>
                <div className="mt-4">
                    <p className="text-white font-sans">MATRIX PORT</p>
                    <Input placeholder="port" className=""/>
                </div>

            </div>
            <div className="flex justify-center mt-12">
              <Button className="text-black bg-white">
                Continue
                </Button> 
            </div>

        </div>
    )
}