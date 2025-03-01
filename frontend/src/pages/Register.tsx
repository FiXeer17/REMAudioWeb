import { ArrowLeft } from "@phosphor-icons/react";
import { Avatar, AvatarImage } from "@radix-ui/react-avatar";
import { Link,Form } from "react-router-dom";
import { Button as Button_register } from "../components/ui/button_sign";
import { Input as Input_email } from "../components/ui/input_email";
import { Input as Input_pass } from "../components/ui/input_pass";

export default function Register() {
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
      <Form method="post" action="/register" className="flex flex-col row-span-4 justify-start gap-[10%]">
      <div className="flex flex-col items-center gap-6">
        <Input_email placeholder="Username"/>
        <Input_email placeholder="Email"/>
      </div>
      <div className="flex flex-col items-center gap-6 ">
          <Input_pass className="visible" Eye_state={"visible"} Forgot={"hidden"} placeholder="Password" />
          <Input_pass className="visible" Eye_state={"hidden"} Forgot={"hidden"} placeholder="Confirm password" />
      </div>
      <div className="flex flex-col items-center justify-start mt-8">
        <Button_register variant={"login"} size={"login"}>
          Register
        </Button_register>
        <Link to={"/Login"} className="text-login_colors-button_bg/text font-bold mt-5">Sign in</Link>
      </div>
      </Form>
    </div>
  );
}

export const registerAction= async({ request }: {request:Request})=>{
  return
}