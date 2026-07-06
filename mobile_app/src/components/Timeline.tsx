import React, { useState } from 'react';
import { View, FlatList, TouchableOpacity, Text, StyleSheet, Dimensions } from 'react-native';
import { useTheme } from '../hooks/useTheme';

const { width } = Dimensions.get('window');

interface Frame {
  id: number;
  duration: number; // frames exposed
  label: string;
}

export const Timeline = () => {
  const { uiTheme } = useTheme();
  const [frames, setFrames] = useState<Frame[]>([
    { id: 1, duration: 2, label: 'A' },
    { id: 2, duration: 2, label: 'B' },
    { id: 3, duration: 1, label: 'C' },
  ]);
  const [selectedFrame, setSelectedFrame] = useState<number | null>(1);

  const addFrame = () => {
    const newId = frames.length + 1;
    setFrames([...frames, { id: newId, duration: 2, label: String.fromCharCode(64 + newId) }]);
  };

  const renderFrame = ({ item }: { item: Frame }) => (
    <TouchableOpacity
      style={[
        styles.frame,
        { backgroundColor: selectedFrame === item.id ? uiTheme?.primary || '#FF6B6B' : '#333' },
      ]}
      onPress={() => setSelectedFrame(item.id)}
    >
      <Text style={styles.frameText}>{item.label}</Text>
      <Text style={styles.durationText}>{item.duration}f</Text>
    </TouchableOpacity>
  );

  return (
    <View style={[styles.container, { backgroundColor: uiTheme?.background || '#222' }]}>
      <View style={styles.header}>
        <Text style={styles.headerTitle}>Exposure Sheet</Text>
        <TouchableOpacity style={styles.addButton} onPress={addFrame}>
          <Text style={styles.addButtonText}>+</Text>
        </TouchableOpacity>
      </View>
      <FlatList
        data={frames}
        renderItem={renderFrame}
        keyExtractor={(item) => item.id.toString()}
        horizontal
        showsHorizontalScrollIndicator={false}
        contentContainerStyle={styles.list}
      />
      {/* Onion skin toggle */}
      <View style={styles.controls}>
        <TouchableOpacity style={styles.controlButton}>
          <Text style={styles.controlText}>🧅 Onion Skin</Text>
        </TouchableOpacity>
        <TouchableOpacity style={styles.controlButton}>
          <Text style={styles.controlText}>▶️ Play</Text>
        </TouchableOpacity>
      </View>
    </View>
  );
};

const styles = StyleSheet.create({
  container: {
    position: 'absolute',
    bottom: 80,
    left: 16,
    right: 16,
    borderRadius: 16,
    padding: 12,
    elevation: 10,
    shadowColor: '#000',
    shadowOffset: { width: 0, height: 4 },
    shadowOpacity: 0.3,
    shadowRadius: 8,
    maxHeight: 180,
  },
  header: {
    flexDirection: 'row',
    justifyContent: 'space-between',
    alignItems: 'center',
    marginBottom: 8,
  },
  headerTitle: { color: '#FFF', fontWeight: 'bold', fontSize: 14 },
  addButton: {
    backgroundColor: '#FF6B6B',
    width: 28,
    height: 28,
    borderRadius: 14,
    justifyContent: 'center',
    alignItems: 'center',
  },
  addButtonText: { color: '#FFF', fontSize: 18, fontWeight: 'bold' },
  list: { gap: 8, paddingVertical: 4 },
  frame: {
    width: 60,
    height: 80,
    borderRadius: 8,
    justifyContent: 'center',
    alignItems: 'center',
    borderWidth: 1,
    borderColor: '#555',
  },
  frameText: { color: '#FFF', fontSize: 18, fontWeight: 'bold' },
  durationText: { color: '#CCC', fontSize: 10, marginTop: 4 },
  controls: {
    flexDirection: 'row',
    justifyContent: 'space-around',
    marginTop: 8,
    borderTopWidth: 1,
    borderTopColor: '#444',
    paddingTop: 8,
  },
  controlButton: {
    padding: 6,
    borderRadius: 4,
    backgroundColor: 'rgba(255,255,255,0.1)',
  },
  controlText: { color: '#FFF', fontSize: 12 },
});
