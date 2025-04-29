import { ArrowLeft } from "@phosphor-icons/react";
import { Link,useNavigate } from "react-router-dom";
import { Input } from "@/components/ui/input_email";
import { Button } from "@/components/ui/button";
import { SubmitHandler, useForm } from "react-hook-form";
import { addSocket } from "@/lib/services";
import { useConnections } from "@/lib/socket/ComponentUuid";
import { toast, Toaster } from "sonner";

type FormFields = {
    name:string;
    ip: string;
    port: string;
  }


export const CreateConnections=()=>{
    const navigate=useNavigate()
    const { register,handleSubmit } =useForm<FormFields>();
    const {uuid}=useConnections()

    const onSubmit: SubmitHandler<FormFields> = async (data) => {
        try{
            const values={
                uuid:uuid,
                socket_name:data.name,
                socket:`${data.ip}:${data.port}`
              }
            await addSocket(values)
            navigate("/homeAudio")
        }catch(error){
            toast.error("Error creating new connections",{duration:1000})
        }
    }
    return (
        <div className="grid grid-rows-[1fr,2fr,1fr]  min-h-svh">
            <div className="flex mt-9 mx-7 justify-between items-start"  >
                <Link to={"/Login"} onClick={() => localStorage.removeItem("accessToken")}>
                <ArrowLeft size={32} color="#FFFFFF" />
                </Link>
                <p className="flex text-white font-sans font-semibold flex-grow items-end justify-center">CREATE CONNECTIONS</p>
            </div>
            <form onSubmit={handleSubmit(onSubmit)}>
            <div className="flex flex-col mx-12 gap-4 justify-start">
                <div className="flex flex-col gap-1 ">
                    <p className="text-white font-sans">MATRIX NAME</p>
                    <Input  {...register("name")} placeholder="name"  className="w-full"/>
                </div>
                <div className="flex flex-col gap-1 ">
                    <p className="text-white font-sans">MATRIX IP</p>
                    <Input {...register("ip")} placeholder="ip" className="w-full"/>
                </div>
                <div className="flex flex-col mt-5 gap-1">
                    <p className="text-white font-sans">MATRIX PORT</p>
                    <Input {...register("port")} placeholder="port" className="w-1/3"/>
                </div>
                <div className="flex justify-center mt-7">
                    <Button className="text-black bg-white" type="submit">
                        Continue
                    </Button> 
                </div>
            
            </div>
            </form>
            <div/>
        <Toaster/>
        </div>
    )
}