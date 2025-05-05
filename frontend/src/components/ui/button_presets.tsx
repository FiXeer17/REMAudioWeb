import { useEffect, useLayoutEffect, useRef, useState } from "react";
import { Button } from "@/components/ui/button";

interface TextPreset {
  text: string;
  onClick?: () => void;
}

export const ButtonPresets = ({ text,onClick }: TextPreset) => {
  const containerRef = useRef<HTMLDivElement>(null);
  const textRef = useRef<HTMLDivElement>(null);
  const [shouldScroll, setShouldScroll] = useState(false);

  const measureElements = () => {
    if (containerRef.current && textRef.current) {
      const containerWidth = containerRef.current.offsetWidth;
      const textWidth = textRef.current.scrollWidth;
      setShouldScroll(textWidth > containerWidth);
    }
  };

  useLayoutEffect(() => {
    measureElements(); 
  }, [text]);

  useEffect(() => {
    window.addEventListener("resize", measureElements);
    return () => {
      window.removeEventListener("resize", measureElements);
    };
  }, []);

  return (
    <Button onClick={onClick} className="border-[1px] border-home_colors-Selected_Borders/text text-home_colors-Selected_Borders/text font-sans font-bold h-11 w-32 px-2 py-4">
      <div ref={containerRef} className="relative w-full overflow-hidden">
        <div
          ref={textRef}
          className={`whitespace-nowrap ${shouldScroll ? "animate-marquee" : ""}`}
        >
          {text}
        </div>
      </div>
    </Button>
  );
};
