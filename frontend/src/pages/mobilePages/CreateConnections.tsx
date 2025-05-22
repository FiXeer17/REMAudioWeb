import { ArrowLeft } from "@phosphor-icons/react";
import { useLocation,useNavigate } from "react-router-dom";
import { Input } from "@/components/ui/input_email";
import { Button } from "@/components/ui/button";
import { SubmitHandler, useForm } from "react-hook-form";
import { setSocket } from "@/lib/services";
import { useConnections } from "@/lib/socket/ComponentUuid";
import { toast, Toaster } from "sonner";
import { RadioGroup, RadioGroupItem } from "@/components/ui/radio-group"
import { Label } from "@/components/ui/label"
import { useEffect } from "react";

type FormFields = {
    name:string;
    ip: string;
    port: string;
    device_type:string;
  }


export const CreateConnections=()=>{
    const navigate=useNavigate()
    const location = useLocation()
    const { register,handleSubmit,setValue,watch } =useForm<FormFields>();
    const {uuid}=useConnections()

    useEffect(() => {
        setValue("device_type", "matrix");
    },[]);

    const onSubmit: SubmitHandler<FormFields> = async (data) => {
        try{
            const values={
                uuid:uuid,
                socket_name:data.name,
                socket:`${data.ip}:${data.port}`,
                device_type: data.device_type
              }
            await setSocket(values)
            localStorage.setItem("showImage","false")
            if (data.device_type==="matrix")
                return navigate("/homeAudio")
            else
                return navigate("/video")
        }catch(error){
            toast.error("Error creating new connections",{duration:1000})
        }
    }
    return (
        <div className="grid grid-rows-[0.45fr_1fr_0.5fr]  min-h-svh">
            <div className="flex mt-9 mx-7 justify-between items-start"  >
                 
                    <ArrowLeft size={32} color="#FFFFFF" onClick={() => {
                        if(location.state){
                            navigate("/recentConnections")
                        }
                        else{
                            localStorage.removeItem("accessToken")
                            navigate("/login")
                        }
                        }}
                        />

                <p className="flex text-white font-sans font-semibold flex-grow items-end justify-center">CREATE CONNECTIONS</p>
            </div>
            <div className="">
                <form onSubmit={handleSubmit(onSubmit)}>
                <div className="flex flex-col mx-12 gap-4 justify-start">
                    <div className="flex flex-col gap-1 ">
                        <p className="text-white font-sans">{watch("device_type") === "matrix" ? "MATRIX NAME" : "CAMERA NAME"}</p>
                        <Input  {...register("name")} placeholder="name"  className="w-full"/>
                    </div>
                    <div className="flex flex-col gap-1 ">
                        <p className="text-white font-sans">{watch("device_type") === "matrix" ? "MATRIX IP" : "CAMERA IP"}</p>
                        <Input {...register("ip")} placeholder="ip" className="w-full"/>
                    </div>
                    <div className="flex mt-5 items-start ">
                        <div className="flex w-1/2 flex-col gap-1">
                            {watch("device_type") && (
                                <>
                                    <p className="text-white font-sans">
                                        {watch("device_type") === "matrix" ? "MATRIX PORT" : "VISCA PORT"}
                                    </p>
                                    <Input {...register("port")} placeholder="port" className="w-2/3"/>
                                </>
                                )}
                        </div>
                        <div className="flex w-1/2 h-[64px] items-end">
                            <RadioGroup defaultValue="matrix" onValueChange={(value) => setValue("device_type", value)}>
                                <div className="flex items-center space-x-2">
                                    <RadioGroupItem   value="matrix"  />
                                    <Label htmlFor="matrix">Matrix</Label>
                                </div>
                                <div className="flex items-center space-x-2">
                                    <RadioGroupItem  value="camera" />
                                    <Label htmlFor="camera">Camera</Label>
                                </div>
                            </RadioGroup>
                        </div>
                    </div>
                    <div className="flex justify-center mt-7">
                        <Button className="text-black bg-white" type="submit">
                            Continue
                        </Button> 
                    </div>
                
                </div>
                </form>
            </div>
            <div/>
        <Toaster/>
        </div>
    )
}