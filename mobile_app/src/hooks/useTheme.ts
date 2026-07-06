import { useState, useCallback } from 'react';
import { generatePalette } from 'pro-color-harmonies';
import { generateTheme } from 'salt-theme-gen';
import { NativeEngine } from '../bridge/NativeEngine';

export const useTheme = (initialColor: string = '#FF6B6B') => {
  const [seedColor, setSeedColor] = useState(initialColor);
  const [uiTheme, setUiTheme] = useState(generateTheme({ primary: initialColor, mode: 'light' }));

  const updateTheme = useCallback((hex: string) => {
    setSeedColor(hex);

    // 1. Generate harmonious palette for drawing
    const drawingPalette = generatePalette({
      baseColor: hex,
      harmony: 'complementary',
      style: 'vibrant',
    });

    // 2. Apply to Rust engine
    NativeEngine.applyTheme({
      primary: drawingPalette.primary,
      secondary: drawingPalette.secondary,
      accent: drawingPalette.accent,
      shades: drawingPalette.swatches || [],
    });

    // 3. Generate UI theme for React Native components
    const newUiTheme = generateTheme({ primary: hex, mode: 'light' });
    setUiTheme(newUiTheme);
  }, []);

  return {
    seedColor,
    uiTheme,
    updateTheme,
  };
};
