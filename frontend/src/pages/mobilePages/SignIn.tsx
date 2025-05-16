import { useNavigate } from "react-router-dom";
import { Button as Button_sign } from "@/components/ui/button_sign";
import { Input as Input_username } from "@/components/ui/input_email";
import { Input as Input_pass } from "@/components/ui/input_pass";
import { Avatar, AvatarImage } from "@radix-ui/react-avatar";
import { useForm,SubmitHandler } from "react-hook-form";
import { toast,Toaster } from "sonner";
import axios from "axios";
import { login as loginUser } from "@/lib/services";
import { useEffect } from "react";


type FormFields = {
  username:string;
  password: string;
}



export const SignInPage=() => {
  const { register,handleSubmit } =useForm<FormFields>();
  const navigate= useNavigate()
  useEffect(()=>{
    localStorage.removeItem("access_token")
  })
  const showErrorToast = (data : FormFields) => {
      if (data.username===""||data.password===""){
        toast.error("All fields must be filled", { duration: 1000 });
        return false;
      }
      return true
   }
  const onSubmit: SubmitHandler<FormFields> = async (data) => {
    if (showErrorToast(data)){
      try{
      const credential={
        username : data.username as string,
        password : data.password as string,
        session_type : "web" as string
      }
      const response= await loginUser(credential)
      const isAdmin=response.data.admin
      const accessToken=response.data.access_token
      localStorage.setItem("accessToken",accessToken)
      localStorage.setItem("isAdmin",isAdmin)
      localStorage.setItem("user",data.username)

      return navigate("/uuidprovider",{state:{isAdmin}})
    }catch(error){
      
        if (axios.isAxiosError(error) && error.response?.status !== 200) {
          
          toast.error("Wrong credentials");
          }
    return navigate("/login")
    }}
  
};

  return (
    <div className="grid grid-rows-6 min-h-svh">
      <div/>
      <div className="flex justify-center">
        <Avatar className="flex justify-center items-start">
          <AvatarImage className="w-4/5" src="/REM_avatar.svg" />
        </Avatar>
      </div>
      <form className="flex flex-col row-span-4 justify-center gap-[10%]" onSubmit={handleSubmit(onSubmit)}>
        <div className="flex flex-col items-center justify-center gap-8">
          <Input_username  {...register("username")} placeholder="username" />
          <Input_pass className="visible" Eye_state={"visible"} Forgot={"visible"} placeholder="Password" {...register("password")} />
        </div>
        <div className="flex flex-col items-center justify-start mt-8 ">
         <Button_sign variant={"login"} size={"login"} >
            Sign In
          </Button_sign>
          
          <Toaster/>
        </div>
        </form>
        
    </div>
  );
}
