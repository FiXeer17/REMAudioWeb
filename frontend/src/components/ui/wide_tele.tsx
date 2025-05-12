import { useState } from "react";

interface InOutButtonProps {
  onChange: (value: "WIDE" | "TELE") => void;
}


export const WideTeleButton=({ onChange }: InOutButtonProps)=> {
  const [selected, setSelected] = useState<"WIDE" | "TELE">("WIDE");


  const handleSelected = (value: "WIDE" | "TELE") => {
    setSelected(value);
    onChange(value); 
  };

  return (
    <div className="flex w-fit">
      <div
        onClick={() => handleSelected("WIDE")}
        className={`flex border-[0.9px] text-sm font-bold justify-center w-20 cursor-pointer select-none
          ${selected === "WIDE" 
            ? "text-home_colors-Selected_Borders/text border-home_colors-Selected_Borders/text"
            : "text-home_colors-Similar_White border-home_colors-Similar_White"}`}
      >
        WIDE
      </div>
      <div
        onClick={() => handleSelected("TELE")}
        className={`flex border-[0.9px] text-sm font-bold justify-center w-20 cursor-pointer select-none
          ${selected === "TELE" 
            ? "text-home_colors-Selected_Borders/text border-home_colors-Selected_Borders/text"
            : "text-home_colors-Similar_White border-home_colors-Similar_White"}`}
      >
        TELE
      </div>
    </div>
  );
}
