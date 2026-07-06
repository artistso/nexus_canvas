import React from 'react';
import { Canvas, Circle, Group, Text, useFont } from '@shopify/react-native-skia';
import { useTheme } from '../hooks/useTheme';

const TOOLS = ['✏️', '🖌️', '🧽', '⭕', '📐'];

export const RadialMenu = ({ x, y, isVisible }: { x: number; y: number; isVisible: boolean }) => {
  const { uiTheme } = useTheme();
  const font = useFont(require('../assets/Inter.ttf'), 24);

  if (!isVisible || !font) return null;

  const radius = 80;
  const items = TOOLS.map((tool, index) => {
    const angle = (index / TOOLS.length) * 2 * Math.PI - Math.PI / 2;
    const cx = x + radius * Math.cos(angle);
    const cy = y + radius * Math.sin(angle);
    return { tool, cx, cy };
  });

  return (
    <Canvas style={{ position: 'absolute', top: 0, left: 0, width: '100%', height: '100%' }}>
      <Group>
        {/* Background circle */}
        <Circle cx={x} cy={y} r={radius + 10} color="rgba(0,0,0,0.6)" />
        <Circle cx={x} cy={y} r={radius} color={uiTheme.primary || '#333'} />
        {/* Tool buttons */}
        {items.map((item) => (
          <Group key={item.tool}>
            <Circle cx={item.cx} cy={item.cy} r={25} color="#fff" />
            <Text
              x={item.cx - 12}
              y={item.cy + 8}
              text={item.tool}
              font={font}
              color="#000"
            />
          </Group>
        ))}
      </Group>
    </Canvas>
  );
};
