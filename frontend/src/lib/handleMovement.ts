type Speed = 'slow' | 'medium' | 'fast';

interface SpeedValues {
  [key: string]: number;
}

interface CameraSpeedControl {
  getCurrentSpeed: () => { name: Speed; delay: number };
  speedValues: SpeedValues;
}

function setupCameraSpeedControl(controlElement: HTMLElement): CameraSpeedControl {
  let clickCount = 0;
  let clickTimer: number | null = null;
  let currentSpeed: Speed = 'slow';
  let moveInterval: number | null = null;

  const speedValues: Record<Speed, number> = {
    slow: 300,
    medium: 150,
    fast: 50
  };

  controlElement.addEventListener('click', () => {
    clickCount++;

    if (clickTimer) clearTimeout(clickTimer);
    clickTimer = window.setTimeout(() => {
      if (clickCount === 1) {
        currentSpeed = 'slow';
      } else if (clickCount === 2) {
        currentSpeed = 'medium';
      } else if (clickCount >= 3) {
        currentSpeed = 'fast';
      }

      console.log(`Velocità impostata a: ${currentSpeed}`);
      clickCount = 0;
    }, 400);
  });

  controlElement.addEventListener('mousedown', () => {
    if (moveInterval !== null) {
      clearInterval(moveInterval);
    }

    const delay = speedValues[currentSpeed];
    console.log(`Avvio movimento a velocità ${currentSpeed} (${delay}ms)`);

    moveInterval = window.setInterval(() => {
      console.log(`Movimento attivo (${currentSpeed})`);
      // Inserisci qui la logica per muovere la videocamera
    }, delay);
  });

  controlElement.addEventListener('mouseup', stopMovement);
  controlElement.addEventListener('mouseleave', stopMovement);

  function stopMovement(): void {
    if (moveInterval !== null) {
      clearInterval(moveInterval);
      moveInterval = null;
      console.log('Movimento fermato');
    }
  }

  function getCurrentSpeed(): { name: Speed; delay: number } {
    return {
      name: currentSpeed,
      delay: speedValues[currentSpeed]
    };
  }

  return {
    getCurrentSpeed,
    speedValues
  };
}

export default setupCameraSpeedControl;