import React from 'react';
import { View, TouchableOpacity, StyleSheet } from 'react-native';
import { useTheme } from '../hooks/useTheme';

const PRESET_COLORS = [
  '#FF6B6B', // Red
  '#FFA94D', // Orange
  '#FFD93D', // Yellow
  '#6BCB77', // Green
  '#4D96FF', // Blue
  '#9B59B6', // Purple
  '#FF6B9D', // Pink
  '#00C9A7', // Teal
];

export const ThemePicker = () => {
  const { updateTheme } = useTheme();

  return (
    <View style={styles.container}>
      {PRESET_COLORS.map((color) => (
        <TouchableOpacity
          key={color}
          style={[styles.swatch, { backgroundColor: color }]}
          onPress={() => updateTheme(color)}
        />
      ))}
    </View>
  );
};

const styles = StyleSheet.create({
  container: {
    flexDirection: 'row',
    padding: 16,
    backgroundColor: 'rgba(255,255,255,0.8)',
    borderRadius: 12,
    margin: 16,
    gap: 12,
  },
  swatch: {
    width: 40,
    height: 40,
    borderRadius: 20,
    borderWidth: 2,
    borderColor: '#fff',
    elevation: 4,
  },
});
