import { Button as Button_sign } from "../components/ui/button_sign";
import { Input as Input_email } from "../components/ui/input_email";
import { Input as Input_pass } from "../components/ui/input_pass";
import { Avatar, AvatarFallback, AvatarImage } from "@radix-ui/react-avatar";



export default function SignInPage() {
  return (
    <>
    <div className="grid grid-rows-5 h-screen">
      <div className="">Riga 1</div>
      <div className="">
        <Avatar>
          <AvatarImage src="/User_avatar.svg"/>
        </Avatar>
      </div>
      <div className="">
      </div>
      <div className="">Riga 4</div>
      <div className="">Riga 5</div>
    </div>
    </>
  );
}
