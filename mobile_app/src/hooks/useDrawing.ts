import { useRef, useState, useCallback } from 'react';
import { Gesture, GestureDetector } from 'react-native-gesture-handler';
import { runOnJS } from 'react-native-reanimated';
import { NativeEngine } from '../bridge/NativeEngine';

export const useDrawing = () => {
  const strokeIdRef = useRef(0);
  const [isDrawing, setIsDrawing] = useState(false);
  const [recentStrokes, setRecentStrokes] = useState<any[]>([]);

  const startStroke = useCallback((x: number, y: number, pressure: number) => {
    const id = strokeIdRef.current++;
    NativeEngine.startStroke(id);
    NativeEngine.addPoint(id, { x, y, pressure });
    setIsDrawing(true);
    return id;
  }, []);

  const addPoint = useCallback((id: number, x: number, y: number, pressure: number) => {
    NativeEngine.addPoint(id, { x, y, pressure });
  }, []);

  const finishStroke = useCallback((id: number) => {
    NativeEngine.finishStroke(id);
    setIsDrawing(false);
    // Update UI with latest strokes
    const strokes = NativeEngine.getRecentStrokes(10);
    setRecentStrokes(strokes);
  }, []);

  const panGesture = Gesture.Pan()
    .onStart((event) => {
      const id = startStroke(event.x, event.y, event.pressure || 0.5);
      (event as any).strokeId = id;
    })
    .onUpdate((event) => {
      const id = (event as any).strokeId;
      if (id !== undefined) {
        addPoint(id, event.x, event.y, event.pressure || 0.5);
      }
    })
    .onEnd((event) => {
      const id = (event as any).strokeId;
      if (id !== undefined) {
        finishStroke(id);
      }
    });

  return {
    panGesture,
    isDrawing,
    recentStrokes,
    undo: NativeEngine.undo,
    redo: NativeEngine.redo,
  };
};
