import { Link } from "react-router-dom";
import { Button as Button_sign } from "../components/ui/button_sign";
import { Input as Input_email } from "../components/ui/input_email";
import { Input as Input_pass } from "../components/ui/input_pass";
import { Avatar, AvatarImage } from "@radix-ui/react-avatar";

export default function SignInPage() {
  return (
    <div className="grid grid-rows-6 min-h-svh">
      <div></div>
      <div className="flex justify-center">
        <Avatar className="w-3/5">
          <AvatarImage src="/REM_avatar.svg" />
        </Avatar>
      </div>
      <div className="flex flex-col items-center justify-center row-span-2 gap-[17%]">
        <Input_email placeholder="Email" />
        <Input_pass className="visible" Forgot={"visible"} placeholder="Password" />
        
      </div>
      <div className="flex flex-col row-span-2 items-center justify-start mt-8 gap-[10%]">
        <Button_sign variant={"login"} size={"login"}>
          Sign In
        </Button_sign>
        <Link to={"/Register"} className="text-login_colors-button_bg/text font-bold">Register</Link>
      </div>
    </div>
  );
}
