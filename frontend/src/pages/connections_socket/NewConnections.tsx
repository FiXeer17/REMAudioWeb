import { ArrowLeft, LinkSimple } from "@phosphor-icons/react";
import { useEffect, useState } from "react";
import { Link, useLocation } from "react-router-dom";
import { toast,Toaster } from "sonner";

export default function NewConnections() {
  const location=useLocation()
  const [show] = useState<boolean>(() => location.state?.show);
  
  useEffect(()=>{
    if(show)
        toast.error("Error with the socket, try again",{duration:1000})
},[show])

  return (
    <div className="grid grid-rows-[auto,1fr] min-h-svh ">
      <div className="mt-9 ml-7">
        <Link to={"/Login"}>
          <ArrowLeft size={32} color="#FFFFFF" />
        </Link>
      </div>
      <div className="flex justify-center items-center relative"> 
        <div className="flex justify-center items-center">
          {[1, 2, 3, 4, 5].map((i, index, array) => {
            const isLast = index === array.length - 1; 
            return (
              <div
                key={i}
                className="absolute rounded-full border border-home_colors-Selected_Borders/text"
                style={{
                  width: isLast
                    ? `${(100 - i * 10) * 0.8}vw`
                    : `${100 - i * 10}vw`, 
                  height: isLast
                    ? `${(100 - i * 10) * 0.8}vw`
                    : `${100 - i * 10}vw`, 
                  backgroundColor: isLast
                    ? "rgba(0, 46, 153, 0.2)"
                    : "transparent",
                  opacity:0.9,
                  filter: isLast
                  ?"blur(0px)"
                  :`blur(${4-i}px)`,
                  left: "50%", 
                  transform: "translateX(-50%)"
                }}
              >
              {isLast && (
                <div className="flex items-center justify-center h-full">
                  <Link to="/createconnections" className="flex flex-col justify-center items-center h-full w-full absolute z-10 text-home_colors-newconnections">
                      new connections
                    <LinkSimple size={32} color="#007AFF" />
                  </Link>
                </div>
              )}
              </div>
            );
          })}
        </div>
      </div>
      <Toaster/>
    </div>
  );
}
