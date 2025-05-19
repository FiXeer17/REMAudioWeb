import { useEffect, useState, useCallback } from "react";

export type IntensityType = "slow" | "medium" | "fast";
export type MovementDirection = "up" | "down" | "left" | "right";

interface UseClickAndHoldOptions {
  onHold?: (intensity: IntensityType) => void;
  onSlowClick?: (action: string) => void;
  holdDelay?: number;
  clickResetDelay?: number;
  holdInterval?: number;
  rapidClickThreshold?: number;
}

interface UseClickAndHoldResult {
  clickCount: number;
  isHolding: boolean;
  currentIntensity: IntensityType | null;
  handleAction: (isStart: boolean) => void;
}

export const useClickAndHold = (options: UseClickAndHoldOptions = {}): UseClickAndHoldResult => {
  const {
    onHold = () => {},
    onSlowClick = () => {},
    holdDelay = 300,
    clickResetDelay = 500,
    holdInterval = 200,
    rapidClickThreshold = 300
  } = options;

  const [clickCount, setClickCount] = useState<number>(0);
  const [isHolding, setIsHolding] = useState<boolean>(false);
  const [currentIntensity, setCurrentIntensity] = useState<IntensityType | null>(null);
  const [clickTimeoutId, setClickTimeoutId] = useState<number | null>(null);
  const [holdTimeoutId, setHoldTimeoutId] = useState<number | null>(null);
  const [intervalId, setIntervalId] = useState<number | null>(null);
  const [lastClickTime, setLastClickTime] = useState<number>(0);

  const resetClickCount = useCallback(() => {
    setClickCount(0);
  }, []);

  const getIntensity = useCallback((count: number): IntensityType => {
    if (count === 1) return "slow";
    if (count === 2) return "medium";
    return "fast";
  }, []);

  const handleAction = useCallback((isStart: boolean) => {
    if (isStart) {

      const currentTime = Date.now();

      if (clickTimeoutId !== null) {
        window.clearTimeout(clickTimeoutId);
        setClickTimeoutId(null);
      }

      if (holdTimeoutId !== null) {
        window.clearTimeout(holdTimeoutId);
        setHoldTimeoutId(null);
      }

      const isFirstOrSlowClick = currentTime - lastClickTime > rapidClickThreshold;
      
      setClickCount(prevCount => {
        if (isFirstOrSlowClick) {
          onSlowClick("slow");
          return 1;
        } else {
          const newCount = prevCount + 1;
          return newCount > 3 ? 3 : newCount;
        }
      });

      setLastClickTime(currentTime);

      const holdTimer = window.setTimeout(() => {
        setClickCount(currentClickCount => {
          const intensity = getIntensity(currentClickCount);
          setCurrentIntensity(intensity);
          setIsHolding(true);
          onHold(intensity);

          const interval = window.setInterval(() => {
            onHold(intensity);
          }, holdInterval);

          setIntervalId(interval);

          return currentClickCount;
        });
      }, holdDelay);

      setHoldTimeoutId(holdTimer);

      const clickTimer = window.setTimeout(resetClickCount, clickResetDelay);
      setClickTimeoutId(clickTimer);
    } else {
      
      if (intervalId !== null) {
        window.clearInterval(intervalId);
        setIntervalId(null);
      }

      if (holdTimeoutId !== null) {
        window.clearTimeout(holdTimeoutId);
        setHoldTimeoutId(null);
      }

      if (isHolding) {
        setIsHolding(false);
        setCurrentIntensity(null);
        setClickCount(0);

        if (clickTimeoutId !== null) {
          window.clearTimeout(clickTimeoutId);
          setClickTimeoutId(null);
        }
      }
    }
  }, [
    clickTimeoutId, 
    holdTimeoutId, 
    intervalId, 
    isHolding, 
    lastClickTime, 
    rapidClickThreshold, 
    onSlowClick, 
    holdDelay, 
    getIntensity, 
    onHold, 
    holdInterval, 
    clickResetDelay, 
    resetClickCount
  ]);

  useEffect(() => {
    return () => {
      if (clickTimeoutId !== null) window.clearTimeout(clickTimeoutId);
      if (holdTimeoutId !== null) window.clearTimeout(holdTimeoutId);
      if (intervalId !== null) window.clearInterval(intervalId);
    };
  }, [clickTimeoutId, holdTimeoutId, intervalId]);

  return {
    handleAction,
    clickCount,
    isHolding,
    currentIntensity
  };
};