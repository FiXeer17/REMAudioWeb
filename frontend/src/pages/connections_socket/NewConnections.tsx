import { ArrowLeft, LinkSimple } from "@phosphor-icons/react";
import { useNavigate, Link } from "react-router-dom";

export default function NewConnetions() {
  const navigate = useNavigate();

  return (
    <div className="grid grid-rows-[auto,1fr] min-h-svh ">
      <div className="mt-9 ml-7">
        <Link to={"/Login"}>
          <ArrowLeft size={32} color="#FFFFFF" />
        </Link>
      </div>
      <div className="flex justify-center items-center relative"> {/* Aggiungi relative per il posizionamento */}
        <div className="flex justify-center items-center">
          {[1, 2, 3, 4, 5].map((i, index, array) => {
            const isLast = index === array.length - 1; // Verifica se è l'ultimo cerchio

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
                  left: "50%", 
                  transform: "translateX(-50%)"
                }}
              />
            );
          })}
        </div>

        <Link to="/createconnections" className="flex flex-col justify-center items-center absolute z-10 text-home_colors-newconnections">
            new connections
          <LinkSimple size={32} color="#007AFF" />
        </Link>
      </div>
    </div>
  );
}
