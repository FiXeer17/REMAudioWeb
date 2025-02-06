import { Button } from "../components/ui/button";
import { Input } from "../components/ui/input"


export default function LoginPage() {
  return (
    <>
      <div className="h-36"/>
      <div className="flex items-center justify-center">        
        <Button size={"login"} variant={"login"}>Sign in</Button>
      </div>
      <div className="flex items-center justify-center m-10">
      <Input placeholder="Email"></Input>
      </div>
    </>
  );
}
