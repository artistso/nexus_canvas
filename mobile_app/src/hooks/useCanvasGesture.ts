import { useRef, useCallback } from 'react';
import { Gesture } from 'react-native-gesture-handler';
import { useSharedValue, withSpring } from 'react-native-reanimated';
import { NativeEngine } from '../bridge/NativeEngine';

export const useCanvasGesture = (screenWidth: number, screenHeight: number) => {
  const zoom = useSharedValue(1);
  const offsetX = useSharedValue(0);
  const offsetY = useSharedValue(0);
  const rotation = useSharedValue(0);

  // Track previous touch positions for delta calculations
  const prevPan = useRef({ x: 0, y: 0 });
  const prevPinch = useRef({ scale: 1, x: 0, y: 0, rotation: 0 });

  // Pan Gesture (One finger)
  const panGesture = Gesture.Pan()
    .onStart(() => {
      // Store current screen position of the viewport
      const viewport = NativeEngine.get_viewport();
      prevPan.current = { x: viewport.center_x, y: viewport.center_y };
    })
    .onUpdate((event) => {
      // Convert screen delta to world delta
      NativeEngine.pan_viewport(-event.translationX, -event.translationY);
    })
    .onEnd(() => {
      // Optional: Spring back or snap
    });

  // Pinch + Rotate Gesture (Two fingers)
  const pinchGesture = Gesture.Pinch()
    .onStart((event) => {
      prevPinch.current = {
        scale: event.scale,
        x: event.focalX,
        y: event.focalY,
        rotation: event.rotation,
      };
    })
    .onUpdate((event) => {
      const deltaScale = event.scale / prevPinch.current.scale;
      const deltaRot = event.rotation - prevPinch.current.rotation;

      // Apply zoom to viewport
      NativeEngine.zoom_viewport(deltaScale, event.focalX, event.focalY);
      // Apply rotation
      NativeEngine.rotate_viewport(deltaRot);

      // Update state
      prevPinch.current = {
        scale: event.scale,
        x: event.focalX,
        y: event.focalY,
        rotation: event.rotation,
      };
    })
    .onEnd(() => {
      // Reset pinch state for next gesture
      prevPinch.current.scale = 1;
      prevPinch.current.rotation = 0;
    });

  // Combine gestures (Pan competes with Pinch, but we allow simultaneous)
  const composedGesture = Gesture.Simultaneous(panGesture, pinchGesture);

  // Reset viewport to center
  const resetView = useCallback(() => {
    NativeEngine.reset_viewport();
  }, []);

  return {
    gestureHandler: composedGesture,
    zoom,
    rotation,
    resetView,
  };
};
