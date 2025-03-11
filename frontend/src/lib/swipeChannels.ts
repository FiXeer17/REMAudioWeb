import { useState, useRef } from "react";

interface SwipeChannelsReturn {
  displayedChannels: string[];
  offset: number;
  handleTouchStart: (e: React.TouchEvent) => void;
  handleTouchMove: (e: React.TouchEvent) => void;
  handleTouchEnd: () => void;
}

export const SwipeChannels = (
  channels1: string[] = ["CH1", "CH2", "CH3", "CH4", "CH5", "CH6", "CH7", "CH8"],
  channels2: string[] = ["CH9", "CH10", "CH11", "CH12", "CH13", "CH14", "CH15", "CH16"]
): SwipeChannelsReturn => {
  const [currentSet, setCurrentSet] = useState(0);
  const [offset, setOffset] = useState(0);
  const touchStartX = useRef(0);
  const isDragging = useRef(false);

  const displayedChannels = currentSet === 0 ? channels1 : channels2;

  const handleTouchStart = (e: React.TouchEvent) => {
    touchStartX.current = e.touches[0].clientX;
    isDragging.current = true;
  };

  const handleTouchMove = (e: React.TouchEvent) => {
    if (!isDragging.current) return;
    const deltaX = e.touches[0].clientX - touchStartX.current;
    
    if ((currentSet === 0 && deltaX > 0) || (currentSet === 1 && deltaX < 0)) {
      return;
    }
    
    setOffset(deltaX); 
  };

  const handleTouchEnd = () => {
    if (offset > 70 && currentSet > 0) {
      setCurrentSet(0);
    } else if (offset < -70 && currentSet < 1) {
      setCurrentSet(1);
    }
    
    setOffset(0); 
    isDragging.current = false;
  };

  return {
    displayedChannels,
    offset,
    handleTouchStart,
    handleTouchMove,
    handleTouchEnd
  };
};