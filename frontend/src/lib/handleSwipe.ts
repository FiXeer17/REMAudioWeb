import { useRef, useCallback } from 'react';

interface SliderThrottleOptions {
  speedThreshold?: number;
  slowInterval?: number;
  fastInterval?: number;
  skipCount?: number;
}

interface ThrottleState {
  lastSentTime: { [key: string]: number };
  isMoving: { [key: string]: boolean };
  lastValue: { [key: string]: number };
  changeCount: { [key: string]: number };
  changeTime: { [key: string]: number };
  changeSpeed: { [key: string]: number };
}

export const useSliderThrottle = (
  sendValueCallback: (channel: string, source: string, value: number) => void,
  options: SliderThrottleOptions = {}
) => {
  const {
    speedThreshold = 0.3,
    slowInterval = 50,
    fastInterval = 100,
    skipCount = 3
  } = options;

  const state = useRef<ThrottleState>({
    lastSentTime: {},
    isMoving: {},
    lastValue: {},
    changeCount: {},
    changeTime: {},
    changeSpeed: {}
  });

  const handleSliderChange = useCallback((newValue: number[], channel: string, source: string) => {
    const currentValue = newValue[0];
    const currentTime = Date.now();

    if (!state.current.isMoving[channel]) {
      state.current.isMoving[channel] = true;
      state.current.lastValue[channel] = currentValue;
      state.current.changeTime[channel] = currentTime;
      state.current.changeCount[channel] = 0;
      state.current.changeSpeed[channel] = 0;
    }

    const timeDelta = currentTime - (state.current.changeTime[channel] || currentTime);
    if (timeDelta > 0) {
      const valueDelta = Math.abs(currentValue - (state.current.lastValue[channel] || currentValue));
      state.current.changeSpeed[channel] = valueDelta / timeDelta;
    }

    state.current.lastValue[channel] = currentValue;
    state.current.changeTime[channel] = currentTime;
    state.current.changeCount[channel] = (state.current.changeCount[channel] || 0) + 1;

    const lastSentTime = state.current.lastSentTime[channel] || 0;
    const timeSinceLastSent = currentTime - lastSentTime;
    const isFastMovement = state.current.changeSpeed[channel] > speedThreshold;

    let shouldSend = false;

    if (isFastMovement) {
      shouldSend = (state.current.changeCount[channel] % skipCount === 0) || (timeSinceLastSent > fastInterval);
    } else {
      shouldSend = timeSinceLastSent > slowInterval;
    }

    if (shouldSend) {
      sendValueCallback(channel, source, currentValue);
      state.current.lastSentTime[channel] = currentTime;
      state.current.changeCount[channel] = 0;
    }
  }, [sendValueCallback, skipCount, slowInterval, fastInterval, speedThreshold]);

  const handleSliderCommit = useCallback((newValue: number[], channel: string, source: string) => {
    const currentValue = newValue[0];
    sendValueCallback(channel, source, currentValue);
    state.current.isMoving[channel] = false;
    state.current.changeCount[channel] = 0;
  }, [sendValueCallback]);

  return {
    handleSliderChange,
    handleSliderCommit
  };
};

export default useSliderThrottle;
