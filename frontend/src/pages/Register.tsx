import { ArrowLeft } from "@phosphor-icons/react";
import { Avatar, AvatarImage } from "@radix-ui/react-avatar";
import { Link } from "react-router-dom";
import { Button as Button_register } from "../components/ui/button_sign";
import { Input as Input_email } from "../components/ui/input_email";
import { Input as Input_pass } from "../components/ui/input_pass";

export default function Register() {
  return (
    <div className="grid grid-rows-5 min-h-svh">
      <div className="mt-9 ml-7">
        <Link to={"/Login"}>
          <ArrowLeft size={32} color="#FFFFFF" />
        </Link>
      </div>
      <div className="flex justify-center items-start">
        <Avatar className="flex justify-center">
          <AvatarImage className="w-3/5" src="/REM_avatar.svg" />
        </Avatar>
      </div>
      <div className="flex flex-col row-span-2 justify-start gap-[15%]">
        <div className="flex flex-col items-center ">
        <Input_email placeholder="Username"/>
        <Input_email placeholder="Email"/>
        </div>
        <div className="flex flex-col items-center ">
          <Input_pass className="visible" Eye_state={"visible"} Forgot={"hidden"} placeholder="Password" />
          <Input_pass className="visible" Eye_state={"hidden"} Forgot={"hidden"} placeholder="Confirm password" />
        </div>
      </div>
      <div className="flex flex-col row-span-2 items-center justify-start mt-8 gap-[10%]">
        <Button_register variant={"login"} size={"login"}>
          Register
        </Button_register>
        <Link to={"/Login"} className="text-login_colors-button_bg/text font-bold">Sign in</Link>
      </div>
    </div>
  );
}
