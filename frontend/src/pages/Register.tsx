import { ArrowLeft } from "@phosphor-icons/react";
import { Avatar, AvatarImage } from "@radix-ui/react-avatar";
import { Link,Form, redirect } from "react-router-dom";
import { Button as Button_register } from "../components/ui/button_sign";
import { Input as Input_email } from "../components/ui/input_email";
import { Input as Input_pass } from "../components/ui/input_pass";
import { useState } from "react";

export default function Register() {
  const [match,setMatch]=useState(false)

  const credentialValidation= async (data:FormData)=>{
    try{
    const username = data.get('username') 
    const email = data.get('email') 
    const password = data.get('password')
    const confirm_password= data.get('confirm_password') 
    const session_type = "web" 
    console.log(username)
    console.log(password)
    if (password===confirm_password)
      setMatch(true)
    else{
      setMatch(false)
    }

  }
  catch(error){
    return redirect("/register")
    }
  }
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
      <Form  className="flex flex-col row-span-4 justify-center gap-[10%]">
      <div className="flex flex-col items-center gap-2">
        <Input_email placeholder="Username" name="username"/>
        <Input_email placeholder="Email" name="email"/>
      </div>
      <div className="flex flex-col items-center gap-2 ">
          <Input_pass className="visible" Eye_state={"visible"} Forgot={"hidden"} placeholder="Password" name="password"/>
          <Input_pass className="visible" Eye_state={"hidden"} Forgot={"hidden"} placeholder="Confirm password" name="confirm_password"/>
          {match && <h1 className="text-white" >Password doesn't match</h1>}
      </div>
      <div className="flex flex-col items-center justify-start mt-7">
        <Button_register variant={"login"} size={"login"}>
          Register
        </Button_register>
        <Link to={"/Login"} className="text-login_colors-button_bg/text font-bold mt-5">Sign in</Link>
      </div>
      </Form>
    </div>
  );
}
