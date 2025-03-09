import React, { useState, useRef } from "react";

export default function SwipeComponent() {
  const colors = ["bg-red-500", "bg-green-500", "bg-blue-500", "bg-yellow-500"];
  const [currentIndex, setCurrentIndex] = useState(0);
  const [offset, setOffset] = useState(0);
  const touchStartX = useRef(0);
  const isDragging = useRef(false);

  // Inizio del tocco
  const handleTouchStart = (e: React.TouchEvent) => {
    touchStartX.current = e.touches[0].clientX;
    isDragging.current = true;
  };

  // Durante lo swipe
  const handleTouchMove = (e: React.TouchEvent) => {
    if (!isDragging.current) return;

    const deltaX = e.touches[0].clientX - touchStartX.current;

    // Blocca lo swipe se sei al primo o all'ultimo div
    if ((currentIndex === 0 && deltaX > 0) || (currentIndex === colors.length - 1 && deltaX < 0)) {
      return;
    }

    setOffset(deltaX); // Muove subito il div con il dito
  };

  // Rilascio del tocco
  const handleTouchEnd = () => {
    if (offset > 100 && currentIndex > 0) {
      setCurrentIndex((prevIndex) => prevIndex - 1);
    } else if (offset < -100 && currentIndex < colors.length - 1) {
      setCurrentIndex((prevIndex) => prevIndex + 1);
    }

    setOffset(0); // Reset offset dopo il rilascio
    isDragging.current = false;
  };

  return (
    <div className="w-full h-64 flex justify-center items-center bg-gray-300 rounded-lg overflow-hidden">
      <div
        className="flex w-full h-full"
        style={{
          transform: `translateX(calc(-${currentIndex * 100}% + ${offset}px))`,
          transition: isDragging.current ? "none" : "transform 0.3s ease",
        }}
        onTouchStart={handleTouchStart}
        onTouchMove={handleTouchMove}
        onTouchEnd={handleTouchEnd}
      >
        {colors.map((color, index) => (
          <div key={index} className={`w-full h-full flex-shrink-0 ${color}`} />
        ))}
      </div>
    </div>
  );
}
