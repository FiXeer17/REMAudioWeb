import { useEffect, useLayoutEffect, useRef, useState } from "react";
import { Button } from "@/components/ui/button";

export const ButtonEdit=()=> {
  const [text, setText] = useState("");
  const [editing, setEditing] = useState(false);
  const [shouldScroll, setShouldScroll] = useState(false);
  const containerRef = useRef<HTMLDivElement>(null);
  const textRef = useRef<HTMLDivElement>(null);
  
  const measureElements = () => {
    if (containerRef.current && textRef.current) {
      const containerWidth = containerRef.current?.offsetWidth;
      const textWidth = textRef.current?.scrollWidth;
      
      setShouldScroll(textWidth > containerWidth);
    }
  };
  
  useLayoutEffect(() => {
    measureElements();
  }, [text, editing]);
  
  useEffect(() => {
    if (!containerRef.current || !textRef.current) return;
    
    const observer = new MutationObserver(measureElements);
    
    observer.observe(containerRef.current, { attributes: true, childList: true, subtree: true });
    
    observer.observe(textRef.current, { attributes: true, childList: true, subtree: true });
    
    window.addEventListener('resize', measureElements);
    
    return () => {
      observer.disconnect();
      window.removeEventListener('resize', measureElements);
    };
  }, []);
  
  const handleEditComplete = () => {
    setEditing(false);
    setTimeout(measureElements, 0);
  };

  return (
    <div className="flex flex-col gap-2">
      {editing ? (
        <input
          className="border-home_colors-Similar_White bg-home_colors-Navbar/Selection_Bg rounded-sm text-home_colors-Similar_White font-sans font-bold text-center h-10 w-full py-4"
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
          onClick={() => setEditing(true)}
          className="border border-gray-300 text-white font-sans font-bold text-center h-10 w-full"
        >
          <div ref={containerRef} className="relative w-full overflow-hidden">
            <div
              ref={textRef}
              className={`whitespace-nowrap ${shouldScroll ? "animate-marquee" : ""}`}
              style={{
                
              }}
            >
              {text}
            </div>
          </div>
        </Button>
      )}
      
    </div>
  );
}