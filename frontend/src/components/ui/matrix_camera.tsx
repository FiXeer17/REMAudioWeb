import { useEffect, useState } from "react";

interface InOutButtonProps {
  onChange: (value: "MATRIX" | "CAMERA") => void;
  device_disconnected: string
}


export const MatrixCameraButton=({ onChange,device_disconnected }: InOutButtonProps)=> {
  const [selected, setSelected] = useState<"MATRIX" | "CAMERA">("MATRIX");

  useEffect(()=>{
    if (device_disconnected==="matrix"&&selected==="MATRIX"){
        setSelected("CAMERA")
        onChange("CAMERA")
    }else if(device_disconnected==="camera"&&selected==="CAMERA"){
        setSelected("MATRIX")
        onChange("MATRIX")
    }
  },[device_disconnected])

  const handleSelected = (value: "MATRIX" | "CAMERA") => {
    setSelected(value);
    onChange(value); 
  };

  return (
    <div className="flex w-fit">
      <div
        onClick={() => { if (device_disconnected !== "matrix") handleSelected("MATRIX") }}
        className={`flex border-[0.9px] text-sm font-bold justify-center w-20 cursor-pointer
          ${device_disconnected!=="matrix"? selected === "MATRIX" 
                    ? "text-home_colors-Selected_Borders/text border-home_colors-Selected_Borders/text"
                    : "text-home_colors-Similar_White border-home_colors-Similar_White"
                :"text-login_colors-place_holder border-login_colors-place_holder cursor-default"}`}
      >
        MATRIX
      </div>
      <div
        onClick={() => { if (device_disconnected !== "camera") handleSelected("CAMERA") }}
        className={`flex border-[0.9px] text-sm font-bold justify-center w-20 cursor-pointer
            ${device_disconnected!=="camera"? selected === "CAMERA" 
                ? "text-home_colors-Selected_Borders/text border-home_colors-Selected_Borders/text"
                : "text-home_colors-Similar_White border-home_colors-Similar_White"
            :"text-login_colors-place_holder border-login_colors-place_holder cursor-default"}`}
      >
        CAMERA
      </div>
    </div>
  );
}
