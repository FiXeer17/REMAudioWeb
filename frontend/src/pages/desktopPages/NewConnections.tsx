import { ArrowLeft, LinkSimple } from "@phosphor-icons/react";
import { Link } from "react-router-dom";

export default function NewConnections() {
  return (
    <div className="grid grid-rows-[auto,1fr] h-screen">
      <div className="mt-9 ml-7">
        <Link to={"/Login"}>
          <ArrowLeft size={32} color="#FFFFFF" />
        </Link>
      </div>
      <div className="flex justify-center items-center relative pb-10">
        <div className="relative w-[550px] h-[550px]">
          {[1, 2, 3, 4, 5].map((i, index, array) => {
            const isLast = index === array.length - 1;
            const size = isLast ? (100 - i * 10) * 0.8 : 100 - i * 10;

            return (
              <div
                key={i}
                className="absolute rounded-full border border-home_colors-Selected_Borders/text"
                style={{
                  width: `${size}%`,
                  height: `${size}%`,
                  backgroundColor: isLast
                    ? "rgba(0, 46, 153, 0.2)"
                    : "transparent",
                  opacity: 0.9,
                  filter: isLast ? "blur(0px)" : `blur(${4 - i}px)`,
                  left: "50%",
                  top: "50%",
                  transform: `translate(-50%, -50%)`,
                }}
              >
                {isLast && (
                  <div className="flex items-center justify-center h-full w-full">
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
    </div>
  );
}
