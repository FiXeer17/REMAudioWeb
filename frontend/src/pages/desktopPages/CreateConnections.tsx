import { ArrowLeft } from "@phosphor-icons/react";
import { Link,useLocation,useNavigate } from "react-router-dom";
import { Input } from "@/components/ui/input_email";
import { Button } from "@/components/ui/button";
import { SubmitHandler, useForm } from "react-hook-form";
import { addSocket } from "@/lib/services";
import { useConnections } from "@/lib/socket/ComponentUuid";
import { toast } from "sonner";
import { RadioGroup, RadioGroupItem } from "@/components/ui/radio-group"
import { Label } from "@/components/ui/label"

type FormFields = {
    name:string;
    ip: string;
    port: string;
    device_type:string;
  }


export const CreateConnections=()=>{
    const location=useLocation()
    const navigate=useNavigate()
    const { register,handleSubmit,setValue } =useForm<FormFields>();
    const {uuid}=useConnections()


    const onSubmit: SubmitHandler<FormFields> = async (data) => {
        try{
            const values={
                uuid:uuid,
                socket_name:data.name,
                socket:`${data.ip}:${data.port}`,
                device_type: data.device_type
                }
            await addSocket(values)
            if (data.device_type==="matrix")
                return navigate("/homeAudio")
            else
                return navigate("/video")
        }catch(error){
            toast.error("Error creating new connections",{duration:1000})
        }
    }

    return(
        <div className="flex flex-col pt-8 gap-14">
            <div className="relative w-full h-14 flex items-center justify-center ">
                <ArrowLeft size={32} color="#FFFFFF" className="absolute left-7" onClick={() => {
                    if(location.state){
                        navigate("/recentConnections")
                    }
                    else{
                        localStorage.removeItem("accessToken")
                        navigate("/login")
                    }
                    }}
                    />
                <p className="text-white font-sans font-semibold text-center">RECENT CONNECTIONS</p>
            </div>
        <div className="flex h-full justify-center items-start ">
            <div className="flex border-[1.5px] items-center justify-center border-home_colors-Selected_Borders/text border-opacity-40 bg-home_colors-Navbar/Selection_Bg rounded-[60px] h-[500px] w-[400px]">
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
                        <div className="flex mt-5 items-end ">
                    <div className="flex w-1/2 flex-col gap-1">
                        <p className="text-white font-sans ">MATRIX PORT</p>
                        <Input {...register("port")} placeholder="port" className="w-2/3"/>
                    </div>
                    <div >
                        <RadioGroup onValueChange={(value) => setValue("device_type", value)}>
                            <div className="flex items-center space-x-2">
                                <RadioGroupItem  value="matrix"  />
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
        </div>
        </div>
    )
}