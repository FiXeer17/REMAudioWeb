import { ArrowLeft } from "@phosphor-icons/react";
import { Avatar, AvatarImage } from "@radix-ui/react-avatar";
import { Link,redirect } from "react-router-dom";
import { Button as Button_register } from "../components/ui/button_sign";
import { Input as Input_email } from "../components/ui/input_email";
import { Input as Input_pass } from "../components/ui/input_pass";
import { SubmitHandler, useForm } from "react-hook-form"

type FormFields = {
  username:string;
  email:string;
  password:string;
  confirm_password:string;
}

export default function Register() {
  const { register, handleSubmit,setError,watch,formState: { errors } } = useForm<FormFields>();
  
  const onSubmit: SubmitHandler<FormFields> =  async (data)=>{
    
  }

  const password=watch("password")

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
      <form  className="flex flex-col row-span-4 justify-center gap-[10%]" onSubmit={handleSubmit(onSubmit)}>
      <div className="flex flex-col items-center gap-2">
        <Input_email {...register("username",{
          required: "Username required"
          })} placeholder="Username"/>
        <Input_email {...register("email",{
          required: "Email required",
          pattern: {
            value: /^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$/,
            message: "Email format not valid"
          }
        })} placeholder="Email"/>
      </div>
      <div className="flex flex-col items-center gap-2 ">
          <Input_pass {...register("password",{
            required: "Password required",
            pattern: {
              value: /^(?=.*[A-Z])(?=.*\d)(?=.*[!@#$%^&*(),.?":{}|<>]).{8,}$/,
              message: "Password must be at least 8 characters long, with an uppercase letter, a number, and a special character"
                }
 
          })} className="visible" Eye_state={"visible"} Forgot={"hidden"} placeholder="Password"/>
          {errors.password && (<h1 className="text-white">{errors.password.message}</h1>)}
          <Input_pass {...register("confirm_password",{
          required: "Must confirm the password",
          validate: (value) => {
            if(value != password){
              return  "Password doesn't match"
            }
            return true
          }
            })} className="visible" Eye_state={"hidden"} Forgot={"hidden"} placeholder="Confirm password"/>
          
          {errors.confirm_password && (<h1 className="text-white">{errors.confirm_password.message}</h1>)}
      </div>
      <div className="flex flex-col items-center justify-start mt-7">
        <Button_register variant={"login"} size={"login"}>
          Register
        </Button_register>
        <Link to={"/Login"} className="text-login_colors-button_bg/text font-bold mt-5">Sign in</Link>
      </div>
      </form>
    </div>
  );
}
