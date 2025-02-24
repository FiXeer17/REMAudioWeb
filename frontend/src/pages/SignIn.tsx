import { Form, Link } from "react-router-dom";
import { Button as Button_sign } from "../components/ui/button_sign";
import { Input as Input_email } from "../components/ui/input_email";
import { Input as Input_pass } from "../components/ui/input_pass";
import { Avatar, AvatarImage } from "@radix-ui/react-avatar";

export default function SignInPage() {
  return (
    <div className="grid grid-rows-6 min-h-svh">
      <div/>
      <div className="flex justify-center">
        <Avatar className="flex justify-center items-start">
          <AvatarImage className="w-4/5" src="/REM_avatar.svg" />
        </Avatar>
      </div>
      <Form className="flex flex-col row-span-4 justify-center gap-[10%]">
        <div className="flex flex-col items-center justify-center gap-8">
          <Input_email placeholder="Email" />
          <Input_pass className="visible" Eye_state={"visible"} Forgot={"visible"} placeholder="Password" />
        </div>
        <div className="flex flex-col items-center justify-start mt-8 ">
          <Button_sign variant={"login"} size={"login"} type="submit">
            Sign In
          </Button_sign>
          <Link to={"/Register"} className="text-login_colors-button_bg/text font-bold mt-5">
            Register
          </Link>
        </div>
        </Form>
        
    </div>
  );
}
