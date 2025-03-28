import { useState, useRef } from "react";

interface SwipeChannelsReturn {
  displayedChannels: string[];
  offset: number;
  handleTouchStart: (e: React.TouchEvent) => void;
  handleTouchMove: (e: React.TouchEvent) => void;
  handleTouchEnd: () => void;
}

export const SwipeChannels = (
  channels1: string[] = ["1", "2", "3", "4", "5", "6", "7", "8"],
  channels2: string[] = ["9", "10", "11", "12", "13", "14", "15", "16"]
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