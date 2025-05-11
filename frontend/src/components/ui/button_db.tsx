import { useState } from "react";
import { Button } from "@/components/ui/button";

interface textPreset {
  onChange: (value: string) => void;
  Text: string;
}

export const ButtonDb = ({ onChange, Text }: textPreset) => {
  const [text, setText] = useState("");
  const [editing, setEditing] = useState(false);

  const handleEditComplete = () => {
    onChange(text);
    setEditing(false);
  };

  return (
    <div className="flex ">
      {editing ? (
        <input
          className="text-home_colors-Similar_White h-[20px] w-[25px] text-sm bg-transparent font-sans font-bold text-center  focus:ring-0 focus:outline-none border-0 border-home_colors-Navbar/Selection_Bg "
          autoFocus
          value={text}
          onChange={(e) => setText(e.target.value)}
          onBlur={handleEditComplete}
          onKeyDown={(e) => {
            if (e.key === "Enter") handleEditComplete();
          }}
        />
      ) : (
        <Button
          onClick={() => {
            setText(Text); 
            setEditing(true);
          }}
          className="text-white font-sans font-bold text-center text-sm px-0 py-0 h-fit bg-transparent  "
        >
          {Text}
        </Button>
      )}
    </div>
  );
};
