import React, { useState } from 'react';
import { View, Text, FlatList, TouchableOpacity, StyleSheet, Modal, Dimensions } from 'react-native';
import { useTheme } from '../hooks/useTheme';
import { NativeEngine } from '../bridge/NativeEngine';

const { width } = Dimensions.get('window');

interface BrushPreset {
  id: string;
  name: string;
  icon: string; // Emoji or path to thumbnail
  size: number;
  opacity: number;
  spacing: number;
  wetness: number;
}

// Procreate-inspired defaults
const PRESETS: BrushPreset[] = [
  { id: 'round', name: 'Round', icon: '✏️', size: 10, opacity: 1, spacing: 0.05, wetness: 0 },
  { id: 'pencil', name: 'Pencil', icon: '✏️', size: 3, opacity: 0.8, spacing: 0.02, wetness: 0 },
  { id: 'marker', name: 'Marker', icon: '🖊️', size: 15, opacity: 0.6, spacing: 0.1, wetness: 0 },
  { id: 'watercolor', name: 'Watercolor', icon: '🎨', size: 25, opacity: 0.4, spacing: 0.15, wetness: 0.8 },
  { id: 'airbrush', name: 'Airbrush', icon: '💨', size: 30, opacity: 0.3, spacing: 0.2, wetness: 0 },
  { id: 'charcoal', name: 'Charcoal', icon: '🪨', size: 12, opacity: 0.7, spacing: 0.08, wetness: 0 },
  { id: 'ink', name: 'Ink Pen', icon: '🖋️', size: 4, opacity: 1, spacing: 0.01, wetness: 0 },
  { id: 'flat', name: 'Flat Brush', icon: '🧹', size: 20, opacity: 0.9, spacing: 0.05, wetness: 0 },
  { id: 'spray', name: 'Spray Paint', icon: '💥', size: 40, opacity: 0.2, spacing: 0.3, wetness: 0.1 },
  { id: 'blend', name: 'Blender', icon: '🌀', size: 15, opacity: 0.5, spacing: 0.05, wetness: 1 },
];

export const BrushLibrary = ({ visible, onClose }: { visible: boolean; onClose: () => void }) => {
  const { uiTheme } = useTheme();
  const [selectedId, setSelectedId] = useState('round');

  const selectBrush = (brush: BrushPreset) => {
    setSelectedId(brush.id);
    NativeEngine.set_brush_size(brush.size);
    NativeEngine.set_brush_opacity(brush.opacity);
    NativeEngine.set_brush_spacing(brush.spacing);
    NativeEngine.set_brush_wetness(brush.wetness);
    onClose();
  };

  const renderItem = ({ item }: { item: BrushPreset }) => (
    <TouchableOpacity
      style={[
        styles.brushItem,
        {
          backgroundColor: selectedId === item.id ? (uiTheme?.primary || '#FF6B6B') : '#333',
          borderColor: selectedId === item.id ? '#FFF' : 'transparent',
        },
      ]}
      onPress={() => selectBrush(item)}
    >
      <Text style={styles.brushIcon}>{item.icon}</Text>
      <Text style={styles.brushName}>{item.name}</Text>
      <Text style={styles.brushSize}>{item.size}px</Text>
    </TouchableOpacity>
  );

  return (
    <Modal visible={visible} animationType="slide" transparent>
      <View style={styles.modalContainer}>
        <View style={[styles.panel, { backgroundColor: uiTheme?.background || '#1a1a1a' }]}>
          <View style={styles.header}>
            <Text style={styles.title}>Brush Library</Text>
            <TouchableOpacity onPress={onClose}>
              <Text style={styles.closeButton}>✖️</Text>
            </TouchableOpacity>
          </View>
          <FlatList
            data={PRESETS}
            renderItem={renderItem}
            keyExtractor={(item) => item.id}
            numColumns={3}
            contentContainerStyle={styles.grid}
          />
        </View>
      </View>
    </Modal>
  );
};

const styles = StyleSheet.create({
  modalContainer: {
    flex: 1,
    justifyContent: 'flex-end',
    backgroundColor: 'rgba(0,0,0,0.6)',
  },
  panel: {
    maxHeight: '60%',
    borderTopLeftRadius: 24,
    borderTopRightRadius: 24,
    padding: 16,
  },
  header: {
    flexDirection: 'row',
    justifyContent: 'space-between',
    alignItems: 'center',
    marginBottom: 16,
  },
  title: { color: '#FFF', fontSize: 20, fontWeight: 'bold' },
  closeButton: { color: '#FFF', fontSize: 20 },
  grid: {
    paddingBottom: 20,
  },
  brushItem: {
    flex: 1,
    margin: 6,
    padding: 12,
    borderRadius: 12,
    alignItems: 'center',
    borderWidth: 2,
    minHeight: 80,
    justifyContent: 'center',
  },
  brushIcon: { fontSize: 28, marginBottom: 4 },
  brushName: { color: '#FFF', fontSize: 12, fontWeight: '500' },
  brushSize: { color: '#AAA', fontSize: 10 },
});
