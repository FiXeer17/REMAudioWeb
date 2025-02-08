import * as React from "react";
import { Eye, EyeSlash } from "@phosphor-icons/react";

import { cn } from "@/lib/utils";

const Input = React.forwardRef<HTMLInputElement, React.ComponentProps<"input">>(
  ({ className, ...props }, ref) => {
    return (
      <div className="relative w-3/4">
        <input
          
          className={cn(
            "h-9 w-full rounded-md border border-login_colors-place_holder bg-transparent px-3 pr-10 py-1 text-login_colors-button_bg/text shadow-sm transition-colors file:border-0 file:bg-transparent file:text-sm file:font-medium file:text-foreground placeholder:text-muted-foreground focus-visible:outline-none focus-visible:ring-1 focus-visible:ring-ring disabled:cursor-not-allowed disabled:opacity-50 md:text-sm",
            className
          )}
          ref={ref}
          {...props}
        />
        <EyeSlash className="absolute right-3 top-1/2 transform -translate-y-1/2 text-white cursor-pointer" />
      </div>
    );
  }
);
Input.displayName = "Input";

export { Input };
