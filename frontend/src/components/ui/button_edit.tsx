import { useState } from "react";
import { Button } from "@/components/ui/button"; 

export const ButtonEdit=()=> {
  const [text, setText] = useState("LABELS\nPRESETS");
  const [editing, setEditing] = useState(false);

  return (
    <>
      {editing ? (
        <input
          className="border rounded px-2 py-1"
          autoFocus
          value={text}
          onChange={(e) => setText(e.target.value)}
          onBlur={() => setEditing(false)} 
          onKeyDown={(e) => {
            if (e.key === "Enter") setEditing(false);
          }}
        />
      ) : (
        <Button
          onClick={() => setEditing(true)}
          className="border-[1px] border-home_colors-Similar_White text-home_colors-Similar_White font-sans font-bold whitespace-pre-line text-center h-10 w-22 py-4" 
        >
          {text}
        </Button>
      )}
    </>
  );
}
