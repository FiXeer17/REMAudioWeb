import { ArrowLeft } from "@phosphor-icons/react";
import { Avatar, AvatarImage } from "@radix-ui/react-avatar";
import { Link } from "react-router-dom";
import { Button as Button_register } from "../components/ui/button_sign";

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
          <AvatarImage className="w-3/5" src="/REM_avatar.svg" />
        </Avatar>
      </div>
      <div>
        <ArrowLeft className="w-3/4" />
      </div>
      <div>
        <ArrowLeft className="w-3/4" />
      </div>
      <div>
        <ArrowLeft className="w-3/4" />
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
