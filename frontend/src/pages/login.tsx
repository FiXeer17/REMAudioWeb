import { Button } from "../components/ui/button_sign";
import { Input as Input_email } from "../components/ui/input_email";
import { Input as Input_pass } from "../components/ui/input_pass";
import { Badge } from "../components/ui/badge";
import { Button as Button_channels } from "../components/ui/button_channels";
import { Button as Button_mute } from "../components/ui/button_mute";

export default function LoginPage() {
  return (
    <>
      <div className="h-36" />
      <div className="flex items-center justify-center">
        <Button size={"login"} variant={"login"}>
          Sign in
        </Button>
      </div>
      <div className="flex items-center justify-center m-10">
        <Input_pass placeholder="Password"></Input_pass>
      </div>
      <div className="flex items-center justify-center m-10">
        <Button_channels variant={"channels_activated"}>CH2</Button_channels>
        <Button_channels variant={"channels_disabled"}>CH2</Button_channels>
      </div>
      <div className="flex items-center justify-center m-10">
        <Button_mute variant="modified">MUTE ALL</Button_mute>
      </div>
    </>
  );
}
