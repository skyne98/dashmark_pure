import 'package:dashmark_pure/typed_buffer/mod.dart';
import 'package:vector_math/vector_math.dart';

import '../ffi_export.dart';

import 'package:vector_math/vector_math_64.dart' as vector64;

// Position
void setPosition(GenerationalIndex entity, double x, double y) {
  var indicesBuffer = GenerationalIndexBuffer();
  indicesBuffer.add(entity);
  var vectorBuffer = Vector64Buffer();
  vectorBuffer.add(vector64.Vector2(x, y));
  api.entitiesSetPositionRaw(
      indices: indicesBuffer.buffer.toUint8List(),
      positions: vectorBuffer.buffer.toUint8List());
}

void setPositionsBulk(
    GenerationalIndexBuffer entities, Vector64Buffer positions) {
  api.entitiesSetPositionRaw(
      indices: entities.buffer.toUint8List(),
      positions: positions.buffer.toUint8List());
}

// Rotation
void setRotation(GenerationalIndex entity, double angle) {
  var indicesBuffer = GenerationalIndexBuffer();
  indicesBuffer.add(entity);
  var vectorBuffer = Vector64Buffer();
  vectorBuffer.add(vector64.Vector2(angle, 0));
  api.entitiesSetRotationRaw(
      indices: indicesBuffer.buffer.toUint8List(),
      rotations: vectorBuffer.buffer.toUint8List());
}

void setRotationsBulk(
    GenerationalIndexBuffer entities, Float64Buffer rotations) {
  api.entitiesSetRotationRaw(
      indices: entities.buffer.toUint8List(),
      rotations: rotations.toUint8List());
}

// Scale
void setScale(GenerationalIndex entity, double x, double y) {
  var indicesBuffer = GenerationalIndexBuffer();
  indicesBuffer.add(entity);
  var vectorBuffer = Vector64Buffer();
  vectorBuffer.add(vector64.Vector2(x, y));
  api.entitiesSetScaleRaw(
      indices: indicesBuffer.buffer.toUint8List(),
      scales: vectorBuffer.buffer.toUint8List());
}

void setScalesBulk(GenerationalIndexBuffer entities, Vector64Buffer scales) {
  api.entitiesSetScaleRaw(
      indices: entities.buffer.toUint8List(),
      scales: scales.buffer.toUint8List());
}

// Origin
void setOrigin(GenerationalIndex entity, double x, double y) {
  var indicesBuffer = GenerationalIndexBuffer();
  indicesBuffer.add(entity);
  var vectorBuffer = Vector64Buffer();
  vectorBuffer.add(vector64.Vector2(x, y));
  api.entitiesSetOriginRaw(
      indices: indicesBuffer.buffer.toUint8List(),
      origins: vectorBuffer.buffer.toUint8List());
}

void setOriginsBulk(GenerationalIndexBuffer entities, Vector64Buffer origins) {
  api.entitiesSetOriginRaw(
      indices: entities.buffer.toUint8List(),
      origins: origins.buffer.toUint8List());
}

// Size
void setSize(GenerationalIndex entity, double x, double y) {
  var indicesBuffer = GenerationalIndexBuffer();
  indicesBuffer.add(entity);
  var vectorBuffer = Vector64Buffer();
  vectorBuffer.add(vector64.Vector2(x, y));
  api.entitiesSetSizeRaw(
      indices: indicesBuffer.buffer.toUint8List(),
      sizes: vectorBuffer.buffer.toUint8List());
}

void setSizesBulk(GenerationalIndexBuffer entities, Vector64Buffer sizes) {
  api.entitiesSetSizeRaw(
      indices: entities.buffer.toUint8List(),
      sizes: sizes.buffer.toUint8List());
}
