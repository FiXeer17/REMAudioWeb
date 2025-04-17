import { useState } from "react";

export default function InOutButton() {
  const [selected, setSelected] = useState<"IN" | "OUT">("IN");

  return (
    <div className="flex w-fit">
      <div
        onClick={() => setSelected("IN")}
        className={`flex border-[0.9px] text-sm font-bold justify-center w-10 cursor-pointer
          ${selected === "IN" 
            ? "text-home_colors-Selected_Borders/text border-home_colors-Selected_Borders/text"
            : "text-home_colors-Similar_White border-home_colors-Similar_White"}`}
      >
        IN
      </div>
      <div
        onClick={() => setSelected("OUT")}
        className={`flex border-[0.9px] text-sm font-bold justify-center w-10 cursor-pointer
          ${selected === "OUT" 
            ? "text-home_colors-Selected_Borders/text border-home_colors-Selected_Borders/text"
            : "text-home_colors-Similar_White border-home_colors-Similar_White"}`}
      >
        OUT
      </div>
    </div>
  );
}
