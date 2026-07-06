import { useEffect, useRef, useState } from 'react';
import { NativeEngine } from '../bridge/NativeEngine';

export const usePerformance = () => {
  const [fps, setFps] = useState(60);
  const frameCount = useRef(0);
  const lastTime = useRef(performance.now());

  useEffect(() => {
    let animationId: number;

    const measureFps = () => {
      const now = performance.now();
      const delta = now - lastTime.current;

      if (delta >= 1000) {
        // Update FPS every second
        const currentFps = Math.round((frameCount.current * 1000) / delta);
        setFps(currentFps);
        frameCount.current = 0;
        lastTime.current = now;

        // Report to console if dropping below 55
        if (currentFps < 55) {
          console.warn(`⚠️ Low FPS detected: ${currentFps}. Check viewport or cache.`);
        }
      }

      frameCount.current += 1;
      animationId = requestAnimationFrame(measureFps);
    };

    animationId = requestAnimationFrame(measureFps);
    return () => cancelAnimationFrame(animationId);
  }, []);

  // Get Rust engine's internal render time (if exposed via UDL)
  const getRenderTime = (): number => {
    // NativeEngine.get_last_render_time_ms()
    return 0;
  };

  return { fps, renderTime: getRenderTime() };
};
