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
      <div className="flex flex-col items-center justify-center gap-[30%] row-span-2">
        <Input_email placeholder="Email" />
        <div>
          <Input_pass placeholder="Password"/>
          <h1 className="text-white self-start">Forgot password?</h1>
        </div>
      </div>
      <div className="flex justify-center">
        <Button_sign variant={"login"} size={"login"}>Sign In</Button_sign>
        <link rel="Register"></link>
      </div>
    </div>
  );
}
