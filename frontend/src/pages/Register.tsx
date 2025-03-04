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

const schema = z.object({
  username: z.string().min(1,{message: "All fields must be filled"}),
  email: z.string().min(1,{message: "All fields must be filled"}).email({message:"Email not valid"}),
  password: z.string().min(1,{message: "All fields must be filled"}),
  confirm_password: z.string().min(1,{message: "All fields must be filled"})
})
.refine((data)=>data.password===data.confirm_password,{
  message:"Password doesn't match ",
  path: ["confirm_password"]
})
.refine((data)=>data.confirm_password===data.password,{
  message:"Password doesn't match ",
  path: ["password"]
})
.refine((data)=>{
  const regex= /^(?=.*[A-Z])(?=.*\d)(?=.*[!@#$%^&*(),.?":{}|<>]).{8,}$/;
  return regex.test(data.password)
},{
  message:"The password must contain at least 8 characters, one uppercase letter, one number, and one special character",
  path: ["confirm_password"]
})

type FormFields = z.infer<typeof schema>;


export default function Register() {
  const {register,handleSubmit,formState: { errors }} = useForm<FormFields>({ resolver: zodResolver(schema) });
  const navigate= useNavigate()

    const showErrorToast = () => {
      if (errors.username?.message === "All fields must be filled" ||
        errors.email?.message === "All fields must be filled" ||
        errors.password?.message === "All fields must be filled" ||
        errors.confirm_password?.message === "All fields must be filled") {
      toast.error("All fields must be filled", { duration: 2000 });
      return;
    }
    if (errors.email?.message === "Email not valid") {
      toast.error("Email not valid", { duration: 2000 });
      return;
    }  
    if (errors.confirm_password?.message === "Password doesn't match ") {
      toast.error("Password doesn't match", { duration: 2000 });
      return;
    }
    if (errors.password?.message === "Password doesn't match ") {
      toast.error("Password doesn't match", { duration: 2000 });
      return;
    }
    if (errors.confirm_password?.message === "The password must contain at least 8 characters, one uppercase letter, one number, and one special character") {
      toast.error("The password must contain at least 8 characters, one uppercase letter, one number, and one special character", { duration: 2000 });
      return;
    }
    };

  const onSubmit: SubmitHandler<FormFields> = async (data) => {
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
  }
  
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
      <form className="flex flex-col row-span-4 justify-center gap-[10%]" onSubmit={handleSubmit(onSubmit,showErrorToast)}>
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
