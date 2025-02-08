import { Button } from "../components/ui/button";
import { Input as Input_email}   from "../components/ui/input_email"
import { Input as Input_pass}   from "../components/ui/input_pass"


export default function LoginPage() {
  return (
    <>
      <div className="h-36"/>
      <div className="flex items-center justify-center">        
        <Button size={"login"} variant={"login"}>Sign in</Button>
      </div>
      <div className="flex items-center justify-center m-10">
      <Input_pass placeholder="Password" ></Input_pass>
      
      </div>
    </>
  );
}
