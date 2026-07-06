import React, { useEffect, useRef } from 'react';
import { Canvas as SkiaCanvas, useCanvas, useValue, useTouchHandler } from '@shopify/react-native-skia';
import { useDrawing } from '../hooks/useDrawing';
import { useTheme } from '../hooks/useTheme';
import { GestureDetector } from 'react-native-gesture-handler';
import { StyleSheet, View } from 'react-native';

export const Canvas = () => {
  const { panGesture, recentStrokes } = useDrawing();
  const { uiTheme } = useTheme();

  // Skia ref for rendering the trail
  const canvasRef = useRef<any>(null);

  // Render the comet trail and strokes
  const renderStrokes = () => {
    // Simplified: render each stroke as a path with glow
    // In production, you'd use Skia's Picture API for performance
    return recentStrokes.map((stroke, idx) => (
      <SkiaPath key={idx}>
        {/* Map points to a path */}
      </SkiaPath>
    ));
  };

  return (
    <View style={styles.container}>
      <GestureDetector gesture={panGesture}>
        <View style={styles.canvasWrapper}>
          <SkiaCanvas style={styles.canvas}>
            {/* Render the comet trail and strokes here */}
            {renderStrokes()}
          </SkiaCanvas>
        </View>
      </GestureDetector>
    </View>
  );
};

const styles = StyleSheet.create({
  container: { flex: 1, backgroundColor: '#fff' },
  canvasWrapper: { flex: 1 },
  canvas: { flex: 1 },
});
