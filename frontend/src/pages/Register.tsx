import { ArrowLeft } from "@phosphor-icons/react";
import { Avatar, AvatarImage } from "@radix-ui/react-avatar";
import { Link, useNavigate } from "react-router-dom";
import { Button as Button_register } from "../components/ui/button_sign";
import { Input as Input_email } from "../components/ui/input_email";
import { Input as Input_pass } from "../components/ui/input_pass";
import { toast, Toaster } from "sonner"
import { SubmitHandler, useForm } from "react-hook-form";
import { zodResolver } from "@hookform/resolvers/zod";
import axios from "axios";
import { register as registerUser } from "@/lib/services";
import { z } from "zod";


type FormFields = {
  username: string;
  email:string;
  password: string;
  confirm_password: string;
}


export default function Register() {
  const { register,handleSubmit } =useForm<FormFields>();
  const navigate= useNavigate()

    const showErrorToast = (data : FormFields) => {
      const emailRegex=/^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$/
      const passRegex= /^(?=.*[A-Z])(?=.*\d)(?=.*[!@#$%^&*(),.?":{}|<>]).{8,}$/
      if (data.username===""||data.email===""||data.password===""||data.confirm_password===""){
        toast.error("All fields must be filled", { duration: 1000 });
        return false;
      }
      if (!emailRegex.test(data.email)){
        toast.error("Email not valid", { duration: 1000 });
        return false;
      } 
      if(data.password!==data.confirm_password){
        toast.error("Password doesn't match", { duration: 1000 });
        return false;
      }
      if(!passRegex.test(data.password)||!passRegex.test(data.confirm_password)){
        toast.error("The password must contain at least 8 characters, one uppercase letter, one number, and one special character", { duration: 1000 });
        return false;
      }
      return true
    };

  const onSubmit: SubmitHandler<FormFields> = async (data) => {
    if (showErrorToast(data)){
      try{
      const credential={
        username : data.username as string,
        email : data.email as string,
        password : data.password as string,
        session_type : "web" as string
      }
      const response= await registerUser(credential)
      const accessToken=response.data.jwt_token
      localStorage.setItem("accessToken",accessToken)

      return navigate("/volume")
    }catch(error){
        if (axios.isAxiosError(error) && error.response?.status === 422) {
          toast.error("Email already taken");
    }
    return navigate("/register")
    }}
  
};
  return (
    <div className="grid grid-rows-6 min-h-svh">
      <div className="mt-9 ml-7">
        <Link to={"/Login"}>
          <ArrowLeft size={32} color="#FFFFFF" />
        </Link>
      </div>
      <div className="flex justify-center items-start">
        <Avatar className="flex justify-center">
          <AvatarImage className="w-4/5" src="/REM_avatar.svg" />
        </Avatar>
      </div>
      <form className="flex flex-col row-span-4 justify-center gap-[10%]" onSubmit={handleSubmit(onSubmit)}>
        <div className="flex flex-col items-center gap-2">
          <Input_email
            {...register("username")}
            placeholder="Username"
          />
          <Input_email
            {...register("email")}
            placeholder="Email"
          />
        </div>
        <div className="flex flex-col items-center gap-2 ">
          <Input_pass
            {...register("password")}
            className="visible"
            Eye_state={"visible"}
            Forgot={"hidden"}
            placeholder="Password"
          />
          <Input_pass 
            {...register("confirm_password")}
            className="visible"
            Eye_state={"hidden"}
            Forgot={"hidden"}
            placeholder="Confirm password"
            
          />      
            <Toaster />            
        </div>
        <div className="flex flex-col items-center justify-start mt-7">

          <Button_register variant={"login"} size={"login" }>
            Register
          </Button_register>
          <Link
            to={"/Login"}
            className="text-login_colors-button_bg/text font-bold mt-5"
          >
            Sign in
          </Link>
        </div>
      </form>
    </div>
  );
}
