import { ArrowLeft, LinkSimple } from "@phosphor-icons/react";
import { useEffect, useState } from "react";
import { Link, useLocation } from "react-router-dom";
import { toast,Toaster } from "sonner";

export const NewConnections=()=> {
  const location=useLocation()
  const [show] = useState<boolean>(() => location.state?.show);
  
  useEffect(()=>{
    if(show)
        toast.error("Error with the socket, try again",{duration:1000})
},[show])

  return (
    <div className="grid grid-rows-[auto,1fr] min-h-svh ">
      <div className="mt-9 ml-7">
        <Link to={"/Login"} onClick={() => localStorage.removeItem("accessToken")}>
          <ArrowLeft size={32} color="#FFFFFF" />
        </Link>
      </div>
      <div className="flex justify-center items-center relative w-full">
        <div className="relative w-full max-w-[600px] aspect-square">
          {[1, 2, 3, 4, 5].map((i, index, array) => {
            const isLast = index === array.length - 1;
            const base = 100 - i * 10;

            return (
              <div
                key={i}
                className="absolute rounded-full border border-home_colors-Selected_Borders/text"
                style={{
                  width: isLast ? `${base * 0.8}%` : `${base}%`,
                  height: isLast ? `${base * 0.8}%` : `${base}%`,
                  backgroundColor: isLast ? "rgba(0, 46, 153, 0.2)" : "transparent",
                  filter: isLast ? "blur(0px)" : `blur(${4 - i}px)`,
                  left: "50%",
                  top: "50%",
                  transform: "translate(-50%, -50%)",
                  opacity: 0.9,
                }}
              >
                {isLast && (
                  <div className="flex items-center justify-center h-full">
                    <Link
                      to="/createconnections"
                      className="flex flex-col justify-center items-center h-full w-full absolute z-10 text-home_colors-newconnections"
                    >
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
