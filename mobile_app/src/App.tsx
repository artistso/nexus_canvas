
import React, { useState } from 'react';
import { SafeAreaView, StyleSheet, View, Text, TouchableOpacity } from 'react-native';
import { CanvasRenderer as Canvas } from './components/CanvasRenderer';
import { ThemePicker } from './components/ThemePicker';
import { RadialMenu } from './components/RadialMenu';
import { useDrawing } from './hooks/useDrawing';
import { BrushLibrary } from './components/BrushLibrary';
import { LayerPanel } from './components/LayerPanel';
import { usePerformance } from './hooks/usePerformance';
import { useCanvasGesture } from './hooks/useCanvasGesture';

export default function App() {
  const [menuVisible, setMenuVisible] = useState(false);
  const [menuPosition, setMenuPosition] = useState({ x: 0, y: 0 });
  const [brushLibVisible, setBrushLibVisible] = useState(false);
  const [layerPanelVisible, setLayerPanelVisible] = useState(false);
  const { isDrawing } = useDrawing();
  const { fps } = usePerformance();

  return (
    <SafeAreaView style={styles.container}>
      <Canvas />

      <View style={styles.fpsCounter}>
        <Text style={styles.fpsText}>{fps} FPS</Text>
      </View>

      <View style={styles.themePickerContainer}>
        <ThemePicker />
      </View>

      <View style={styles.topRightControls}>
        <TouchableOpacity style={styles.button} onPress={() => setBrushLibVisible(true)}>
          <Text style={styles.buttonText}>🖌️</Text>
        </TouchableOpacity>
        <TouchableOpacity style={styles.button} onPress={() => setLayerPanelVisible(true)}>
          <Text style={styles.buttonText}>📑</Text>
        </TouchableOpacity>
      </View>

      <BrushLibrary visible={brushLibVisible} onClose={() => setBrushLibVisible(false)} />
      <LayerPanel visible={layerPanelVisible} onClose={() => setLayerPanelVisible(false)} />

      {menuVisible && (
        <RadialMenu x={menuPosition.x} y={menuPosition.y} isVisible={menuVisible} />
      )}
    </SafeAreaView>
  );
}

const styles = StyleSheet.create({
  container: {
    flex: 1,
    backgroundColor: '#000',
  },
  themePickerContainer: {
    position: 'absolute',
    bottom: 40,
    left: 0,
    right: 0,
    alignItems: 'center',
  },
  topRightControls: {
    position: 'absolute',
    top: 40,
    right: 16,
    flexDirection: 'row',
    gap: 12,
  },
  button: {
    width: 40,
    height: 40,
    borderRadius: 20,
    backgroundColor: 'rgba(255,255,255,0.2)',
    justifyContent: 'center',
    alignItems: 'center',
  },
  buttonText: {
    fontSize: 20,
  },
  fpsCounter: {
    position: 'absolute',
    top: 40,
    left: 16,
    backgroundColor: 'rgba(0,0,0,0.5)',
    padding: 8,
    borderRadius: 8,
  },
  fpsText: {
    color: '#00FF00',
    fontWeight: 'bold',
  }
});
