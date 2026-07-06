import React, { useState } from 'react';
import { View, Text, TouchableOpacity, FlatList, StyleSheet, Modal, Slider } from 'react-native';
import DraggableFlatList, { RenderItemParams } from 'react-native-draggable-flatlist';
import { useTheme } from '../hooks/useTheme';
import { NativeEngine } from '../bridge/NativeEngine';

interface LayerData {
  id: number;
  name: string;
  opacity: number;
  visible: boolean;
  locked: boolean;
  isActive: boolean;
}

export const LayerPanel = ({ visible, onClose }: { visible: boolean; onClose: () => void }) => {
  const { uiTheme } = useTheme();
  const [layers, setLayers] = useState<LayerData[]>([
    { id: 0, name: 'Background', opacity: 1, visible: true, locked: false, isActive: true },
    { id: 1, name: 'Sketch', opacity: 0.8, visible: true, locked: false, isActive: false },
    { id: 2, name: 'Colors', opacity: 1, visible: true, locked: false, isActive: false },
  ]);

  const renderLayer = ({ item, drag, isActive }: RenderItemParams<LayerData>) => {
    const isSelected = item.isActive;

    return (
      <TouchableOpacity
        style={[
          styles.layerRow,
          { backgroundColor: isSelected ? (uiTheme?.primary + '30') || '#FF6B6B30' : 'transparent' },
        ]}
        onLongPress={drag}
        onPress={() => {
          NativeEngine.set_active_layer(item.id);
          setLayers(prev => prev.map(l => ({ ...l, isActive: l.id === item.id })));
        }}
      >
        {/* Visibility Toggle */}
        <TouchableOpacity
          style={styles.iconButton}
          onPress={() => {
            NativeEngine.toggle_layer_visibility(item.id);
            setLayers(prev => prev.map(l => l.id === item.id ? { ...l, visible: !l.visible } : l));
          }}
        >
          <Text style={styles.iconText}>{item.visible ? '👁️' : '🚫'}</Text>
        </TouchableOpacity>

        {/* Layer Name */}
        <Text style={[styles.layerName, { color: isSelected ? '#FFF' : '#AAA' }]}>{item.name}</Text>

        {/* Opacity Slider */}
        <Slider
          style={styles.opacitySlider}
          minimumValue={0}
          maximumValue={1}
          value={item.opacity}
          onValueChange={(val) => {
            NativeEngine.set_layer_opacity(item.id, val);
            setLayers(prev => prev.map(l => l.id === item.id ? { ...l, opacity: val } : l));
          }}
          thumbTintColor={uiTheme?.primary || '#FF6B6B'}
          minimumTrackTintColor={uiTheme?.primary || '#FF6B6B'}
        />

        {/* Lock Toggle */}
        <TouchableOpacity
          style={styles.iconButton}
          onPress={() => {
            NativeEngine.toggle_layer_lock(item.id);
            setLayers(prev => prev.map(l => l.id === item.id ? { ...l, locked: !l.locked } : l));
          }}
        >
          <Text style={styles.iconText}>{item.locked ? '🔒' : '🔓'}</Text>
        </TouchableOpacity>
      </TouchableOpacity>
    );
  };

  return (
    <Modal visible={visible} animationType="slide" transparent>
      <View style={styles.modalContainer}>
        <View style={[styles.panel, { backgroundColor: uiTheme?.background || '#1a1a1a' }]}>
          <View style={styles.header}>
            <Text style={styles.title}>Layers</Text>
            <View style={styles.headerButtons}>
              <TouchableOpacity onPress={() => {
                const newId = NativeEngine.add_layer();
                setLayers(prev => [...prev, {
                  id: newId,
                  name: `Layer ${newId}`,
                  opacity: 1,
                  visible: true,
                  locked: false,
                  isActive: true,
                }]);
              }}>
                <Text style={styles.headerButton}>➕</Text>
              </TouchableOpacity>
              <TouchableOpacity onPress={onClose}>
                <Text style={styles.headerButton}>✖️</Text>
              </TouchableOpacity>
            </View>
          </View>

          <DraggableFlatList
            data={layers}
            renderItem={renderLayer}
            keyExtractor={(item) => item.id.toString()}
            onDragEnd={({ data }) => {
              setLayers(data);
              // Reorder layers in Rust
              const order = data.map(l => l.id);
              NativeEngine.reorder_layers(order);
            }}
            contentContainerStyle={styles.list}
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
    backgroundColor: 'rgba(0,0,0,0.5)',
  },
  panel: {
    maxHeight: '70%',
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
  headerButtons: { flexDirection: 'row', gap: 16 },
  headerButton: { color: '#FFF', fontSize: 20 },
  list: { paddingBo
