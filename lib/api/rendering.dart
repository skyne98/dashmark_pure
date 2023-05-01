import 'dart:typed_data';
import 'package:flutter/material.dart';

import '../ffi_export.dart';
import '../typed_buffer/mod.dart';

void setVertices(GenerationalIndex entity, Vector32Buffer vertices) {
  api.entitySetVerticesRaw(
      index: entity, vertices: vertices.buffer.toFloat32List());
}

void setTexCoords(GenerationalIndex entity, Vector32Buffer texCoords) {
  api.entitySetTexCoordsRaw(
      index: entity, texCoords: texCoords.buffer.toFloat32List());
}

void setIndices(GenerationalIndex entity, Uint16Buffer indices) {
  api.entitySetIndicesRaw(index: entity, indices: indices.toUint16List());
}

void setColor(GenerationalIndex entity, Color color) {
  api.entitySetColor(
    index: entity,
    color: color.value,
  );
}

/// Returns a number of generated batches for rendering.
int batchesCount() {
  return api.batchesCount();
}

/// Returns a [Float32List] view of the transformed vertices.
/// Entities are already sorted by their priority/z-index.
Float32List vertices(int batchIndex) {
  return api.vertices(batchIndex: batchIndex);
}

/// Returns a [Float32List] view of the normals.
/// Entities are already sorted by their priority/z-index.
Float32List texCoords(int batchIndex) {
  return api.texCoords(batchIndex: batchIndex);
}

/// Returns an [Int32List] view of the colors.
/// Entities are already sorted by their priority/z-index.
Int32List colors(int batchIndex) {
  return api.colors(batchIndex: batchIndex);
}

/// Returns a [Uint16List] view of the indices.
/// Entities are already sorted by their priority/z-index.
Uint16List indices(int batchIndex) {
  return api.indices(batchIndex: batchIndex);
}
