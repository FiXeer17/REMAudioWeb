import { ArrowLeft } from "@phosphor-icons/react";
import { Link } from "react-router-dom";


export default function CallAdministrator(){
    return(
        <div className="grid grid-rows-[auto,1fr] h-screen">
            <div className=" mt-9 ml-7">
        <Link to={"/Login"}>
          <ArrowLeft size={32} color="#FFFFFF" />
        </Link>
      </div>

      <div className="flex justify-center items-center">
        {[1, 2, 3].map((i, index, array) => {
          const isLast = index === array.length - 1;
          return (
            <div
              key={i}
              className="absolute rounded-3xl border-2 border-home_colors-Selected_Borders/text"
              style={{
                width: isLast
                  ? `${(37 - i * 6) * 0.9}vw`
                  : `${(37 - i * 6) * 0.9}vw`,
                height: isLast
                  ? `${(90 - i * 10) * 0.9}vh`
                  : `${(90 - i * 10) * 0.9}vh`,

                borderColor: isLast
                  ? "rgba(0, 122, 255, 0.4)"
                  : "rgba(0, 122, 255, 0.4)",
                filter: isLast ? "blur(0px)" : `blur(${3 - i}px)`,
                left: "50%",
                transform: "translateX(-50%)",
              }}
            >
              {isLast && (
                <div className="flex flex-col items-center justify-center h-full text-white opacity-100 pb">
                  <div>
                    <p className="font-bold">
                      <span className="text-[80px]">X_</span>
                      <span className="text-[120px]">X</span>
                    </p>
                  </div>
                  <div className="flex items-center flex-col text-sm font-bold">
                    <p> No connection available,</p>
                    <p>
                      <span>call the </span>
                      <span className="text-home_colors-Selected_Borders/text">
                        administrator
                      </span>
                    </p>
                  </div>
                </div>
              )}
            </div>
          );
        })}
      </div>
        </div>
    )
}