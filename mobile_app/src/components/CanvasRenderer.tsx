import React from 'react';
import { Canvas, Path, Skia, useCanvas, useTouchHandler } from '@shopify/react-native-skia';
import { useDrawing } from '../hooks/useDrawing';
import { useTheme } from '../hooks/useTheme';
import { View, StyleSheet } from 'react-native';
import { GestureDetector } from 'react-native-gesture-handler';

export const CanvasRenderer = () => {
  const { panGesture, recentStrokes, isDrawing } = useDrawing();
  const { uiTheme } = useTheme();

  // Build the Comet Trail and actual strokes
  const renderStrokes = () => {
    const paths: React.ReactNode[] = [];

    // Iterate through strokes from the engine
    // In production, we'd use a Picture or Surface cache for performance.
    // For this scaffold, we render a dynamic path for each stroke.
    recentStrokes.forEach((stroke, idx) => {
      if (!stroke.points || stroke.points.length < 2) return;

      // 1. Main Stroke (Solid)
      const mainPath = Skia.Path.Make();
      mainPath.moveTo(stroke.points[0].x, stroke.points[0].y);
      for (let i = 1; i < stroke.points.length; i++) {
        // Smooth interpolation
        const p0 = stroke.points[i - 1];
        const p1 = stroke.points[i];
        const midX = (p0.x + p1.x) / 2;
        const midY = (p0.y + p1.y) / 2;
        mainPath.quadTo(p0.x, p0.y, midX, midY);
      }

      // 2. Comet Trail (Glow)
      // We use a separate path for the "glow" with varying opacity/width.
      // Simplified: Render a slightly offset, blurred path.
      const glowPath = Skia.Path.Make();
      if (stroke.points.length > 3) {
        glowPath.moveTo(stroke.points[0].x + 2, stroke.points[0].y + 2);
        for (let i = 1; i < stroke.points.length - 1; i++) {
          const p0 = stroke.points[i];
          const p1 = stroke.points[i + 1];
          const midX = (p0.x + p1.x) / 2;
          const midY = (p0.y + p1.y) / 2;
          glowPath.quadTo(p0.x + 2, p0.y + 2, midX + 2, midY + 2);
        }
      }

      // Push the main path
      paths.push(
        <Path
          key={`main-${idx}`}
          path={mainPath}
          color={stroke.color_hex || '#FF6B6B'}
          style="stroke"
          strokeWidth={stroke.width || 4}
          strokeJoin="round"
          strokeCap="round"
        />
      );

      // Push the comet trail (glow) behind it
      if (glowPath) {
        paths.push(
          <Path
            key={`glow-`${idx}`}
            path={glowPath}
            color={stroke.color_hex || '#FF6B6B'}
            style="stroke"
            strokeWidth={(stroke.width || 4) + 8}
            strokeJoin="round"
            strokeCap="round"
            opacity={0.15}
          />
        );
        // Second layer of glow for the "light streak" effect
        paths.push(
          <Path
            key={`glow2-${idx}`}
            path={glowPath}
            color="#FFFFFF"
            style="stroke"
            strokeWidth={(stroke.width || 4) + 2}
            strokeJoin="round"
            strokeCap="round"
            opacity={0.2}
          />
        );
      }
    });

    return paths;
  };

  return (
    <View style={styles.container}>
      <GestureDetector gesture={panGesture}>
        <View style={styles.canvasWrapper}>
          <Canvas style={styles.canvas}>
            {/* Render all stored strokes with comet trails */}
            {renderStrokes()}
          </Canvas>
        </View>
      </GestureDetector>
    </View>
  );
};

const styles = StyleSheet.create({
  container: { flex: 1, backgroundColor: '#1a1a1a' },
  canvasWrapper: { flex: 1 },
  canvas: { flex: 1 },
});
