import { useState } from "react";

interface InOutButtonProps {
  onChange: (value: "IN" | "OUT") => void;
}


export default function InOutButton({ onChange }: InOutButtonProps) {
  const [selected, setSelected] = useState<"IN" | "OUT">("IN");


  const handleSelected = (value: "IN" | "OUT") => {
    setSelected(value);
    onChange(value); 
  };

  return (
    <div className="flex w-fit">
      <div
        onClick={() => handleSelected("IN")}
        className={`flex border-[0.9px] text-sm font-bold justify-center w-10 cursor-pointer select-none
          ${selected === "IN" 
            ? "text-home_colors-Selected_Borders/text border-home_colors-Selected_Borders/text"
            : "text-home_colors-Similar_White border-home_colors-Similar_White"}`}
      >
        IN
      </div>
      <div
        onClick={() => handleSelected("OUT")}
        className={`flex border-[0.9px] text-sm font-bold justify-center w-10 cursor-pointer select-none
          ${selected === "OUT" 
            ? "text-home_colors-Selected_Borders/text border-home_colors-Selected_Borders/text"
            : "text-home_colors-Similar_White border-home_colors-Similar_White"}`}
      >
        OUT
      </div>
    </div>
  );
}
