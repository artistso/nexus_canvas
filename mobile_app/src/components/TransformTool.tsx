import React, { useState } from 'react';
import { Canvas, Rect, Circle, Line, Group, useTouchHandler } from '@shopify/react-native-skia';
import { NativeEngine } from '../bridge/NativeEngine';

interface TransformToolProps {
  layerId: number;
  isVisible: boolean;
}

export const TransformTool = ({ layerId, isVisible }: TransformToolProps) => {
  if (!isVisible) return null;

  // Get the layer's bounding box from the engine
  const bounds = NativeEngine.get_layer_bounds(layerId);
  if (!bounds) return null;

  const [transform, setTransform] = useState({
    x: bounds.x,
    y: bounds.y,
    width: bounds.width,
    height: bounds.height,
    rotation: 0,
  });

  // Touch handler for dragging corners
  const touchHandler = useTouchHandler({
    onStart: (pt) => {
      // Check if touch is near a corner handle
      // Start drag operation
    },
    onMove: (pt) => {
      // Update transform state
      // Send to Rust engine
      // NativeEngine.transform_layer(layerId, transform);
    },
    onEnd: () => {
      // Finalize transform
    },
  });

  return (
    <Canvas style={{ position: 'absolute', top: 0, left: 0, width: '100%', height: '100%' }}>
      <Group transform={[{ translateX: transform.x }, { translateY: transform.y }]}>
        {/* Bounding Box */}
        <Rect
          x={0}
          y={0}
          width={transform.width}
          height={transform.height}
          color="rgba(255,255,255,0.1)"
          style="stroke"
          strokeWidth={1.5}
          strokeColor="#FFF"
        />
        {/* Corner Handles */}
        {[
          [0, 0],
          [transform.width, 0],
          [0, transform.height],
          [transform.width, transform.height],
        ].map(([cx, cy], idx) => (
          <Circle
            key={idx}
            cx={cx}
            cy={cy}
            r={10}
            color="#4D96FF"
            style="fill"
          />
        ))}
        {/* Rotation handle (top center) */}
        <Line
          p1={{ x: transform.width / 2, y: -20 }}
          p2={{ x: transform.width / 2, y: 0 }}
          color="#FFF"
          strokeWidth={1}
        />
        <Circle
          cx={transform.width / 2}
          cy={-20}
          r={8}
          color="#FFD93D"
        />
      </Group>
    </Canvas>
  );
};
