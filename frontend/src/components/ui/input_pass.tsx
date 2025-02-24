import * as React from "react";
import { Eye, EyeSlash } from "@phosphor-icons/react";
import { useState } from "react";
import { cn } from "@/lib/utils";
import { Link } from "react-router-dom";

  



const Input = React.forwardRef<HTMLInputElement, React.ComponentProps<"input"> & {Forgot:String,Eye_state:String}>(
  ({ className,Forgot,Eye_state,...props }, ref) => {
    const [state,setState]=useState(false);

    return (      
      <div className="flex flex-col items-center justify-center relative w-3/4">
        <div className="flex flex-col relative w-full   ">
          <input
            type={ state ? "text" : "password"}
            className={cn(
              "h-9 w-full rounded-md border border-login_colors-place_holder bg-transparent px-3 pr-12 py-1 text-login_colors-button_bg/text shadow-sm transition-colors file:border-0 file:bg-transparent file:text-sm file:font-medium file:text-foreground placeholder:text-muted-foreground focus-visible:outline-none focus-visible:ring-1 focus-visible:ring-ring disabled:cursor-not-allowed disabled:opacity-50 md:text-sm",
              className
            )}
            ref={ref}
            {...props}
          />
          <div className={cn(Eye_state)}>
          {state ? <Eye id="Eye" className="absolute right-3 top-1/2 transform -translate-y-1/2 text-white cursor-pointer" size={22} onClick={()=>setState(!state)}/> 
          : <EyeSlash id="EyeSlash" className="absolute right-3 top-1/2 transform -translate-y-1/2 text-white cursor-pointer" size={22} onClick={()=>setState(!state)} />}
          </div>   
        </div>
        <Link to={"/Login"} className= {cn("flex self-start px-3 text-login_colors-forgot_pass text-sm mt-1 underline",Forgot)}>Forgot password?</Link>
      </div>
    );
  }
);
Input.displayName = "Input";

export { Input };
