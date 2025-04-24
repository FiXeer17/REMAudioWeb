import { useState, useRef } from "react";

interface SwipeVolumesReturn {
  currentSet: number;
  displayedVolumes: Record<string, number>;
  sets: Record<string, number>[];
  offset: number;
  handleTouchStart: (e: React.TouchEvent) => void;
  handleTouchMove: (e: React.TouchEvent) => void;
  handleTouchEnd: () => void;
}

export const SwipeVolumes = (
  i_volumes: Record<string, number>
): SwipeVolumesReturn => {
  const [currentSet, setCurrentSet] = useState(0);
  const [offset, setOffset] = useState(0);
  const touchStartX = useRef(0);
  const isDragging = useRef(false);

  const sets = divideIntoSets(i_volumes);
  const displayedVolumes = sets[currentSet] || {};

  const handleTouchStart = (e: React.TouchEvent) => {
    touchStartX.current = e.touches[0].clientX;
    isDragging.current = true;
  };

  const handleTouchMove = (e: React.TouchEvent) => {
    if (!isDragging.current) return;
    const deltaX = e.touches[0].clientX - touchStartX.current;

    if ((currentSet === 0 && deltaX > 0) || (currentSet === sets.length - 1 && deltaX < 0)) {
      return;
    }

    setOffset(deltaX);
  };

  const handleTouchEnd = () => {
    if (offset > 70 && currentSet > 0) {
      setCurrentSet(currentSet - 1);
    } else if (offset < -70 && currentSet < sets.length - 1) {
      setCurrentSet(currentSet + 1);
    }
    setOffset(0);
    isDragging.current = false;
  };

  return {
    currentSet,
    displayedVolumes,
    sets,
    offset,
    handleTouchStart,
    handleTouchMove,
    handleTouchEnd,
  };
};

const divideIntoSets = (data: Record<string, number>, chunkSize = 4): Record<string, number>[] => {
  const keys = Object.keys(data);
  const sets: Record<string, number>[] = [];

  for (let i = 0; i < keys.length; i += chunkSize) {
    const chunk = keys.slice(i, i + chunkSize);
    const group: Record<string, number> = {};
    chunk.forEach((key) => {
      group[key] = data[key];
    });
    sets.push(group);
  }

  return sets;
};
