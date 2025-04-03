import { ArrowLeft } from "@phosphor-icons/react";
import { Link } from "react-router-dom";


export default function NewConnetions(){

    return (
        <div className="grid grid-rows-[auto,1fr] min-h-svh">
            <div className=" mt-9 ml-7">
                <Link to={"/Login"}>
                <ArrowLeft size={32} color="#FFFFFF" />
                </Link>
            </div>
            <div className="flex justify-center items-center ">
            <div className="relative w-screen h-screen flex justify-center items-center">
        
        {/* Cerchi concentrici */}
        {[1, 2, 3, 4].map((i) => (
          <div
            key={i}
            className={`absolute rounded-full border border-home_colors-Selected_Borders/text opacity-${100 - i * 10} `}
            style={{
              width: `${100 - i * 10}vw`, // Riduzione progressiva della larghezza
              height: `${100 - i * 10}vw`,
            }}
          />
        ))}

       
      </div>
        </div>
            
        </div>
    )
}