import 'package:dashmark_pure/typed_buffer/mod.dart';

import '../ffi_export.dart';

import 'package:vector_math/vector_math_64.dart' as vector64;

void setTransformsBulk(
    GenerationalIndexBuffer entities,
    Vector32Buffer positions,
    Vector32Buffer origins,
    Float32Buffer rotations,
    Vector32Buffer scales) {
  api.entitiesSetTransformRaw(
      indices: entities.buffer.toUint32List(),
      positions: positions.buffer.toFloat32List(),
      origins: origins.buffer.toFloat32List(),
      rotations: rotations.toFloat32List(),
      scales: scales.buffer.toFloat32List());
}

// Position
void setPosition(GenerationalIndex entity, double x, double y) {
  var indicesBuffer = GenerationalIndexBuffer();
  indicesBuffer.add(entity);
  var vectorBuffer = Vector64Buffer();
  vectorBuffer.add(vector64.Vector2(x, y));
  api.entitiesSetPositionRaw(
      indices: indicesBuffer.buffer.toUint32List(),
      positions: vectorBuffer.buffer.toFloat32List());
}

void setPositionsBulk(
    GenerationalIndexBuffer entities, Vector64Buffer positions) {
  api.entitiesSetPositionRaw(
      indices: entities.buffer.toUint32List(),
      positions: positions.buffer.toFloat32List());
}

// Rotation
void setRotation(GenerationalIndex entity, double angle) {
  var indicesBuffer = GenerationalIndexBuffer();
  indicesBuffer.add(entity);
  var vectorBuffer = Vector64Buffer();
  vectorBuffer.add(vector64.Vector2(angle, 0));
  api.entitiesSetRotationRaw(
      indices: indicesBuffer.buffer.toUint32List(),
      rotations: vectorBuffer.buffer.toFloat32List());
}

void setRotationsBulk(
    GenerationalIndexBuffer entities, Float64Buffer rotations) {
  api.entitiesSetRotationRaw(
      indices: entities.buffer.toUint32List(),
      rotations: rotations.toFloat32List());
}

// Scale
void setScale(GenerationalIndex entity, double x, double y) {
  var indicesBuffer = GenerationalIndexBuffer();
  indicesBuffer.add(entity);
  var vectorBuffer = Vector64Buffer();
  vectorBuffer.add(vector64.Vector2(x, y));
  api.entitiesSetScaleRaw(
      indices: indicesBuffer.buffer.toUint32List(),
      scales: vectorBuffer.buffer.toFloat32List());
}

void setScalesBulk(GenerationalIndexBuffer entities, Vector64Buffer scales) {
  api.entitiesSetScaleRaw(
      indices: entities.buffer.toUint32List(),
      scales: scales.buffer.toFloat32List());
}

// Origin
void setOrigin(GenerationalIndex entity, double x, double y) {
  var indicesBuffer = GenerationalIndexBuffer();
  indicesBuffer.add(entity);
  var vectorBuffer = Vector64Buffer();
  vectorBuffer.add(vector64.Vector2(x, y));
  api.entitiesSetOriginRaw(
      indices: indicesBuffer.buffer.toUint32List(),
      origins: vectorBuffer.buffer.toFloat32List());
}

void setOriginsBulk(GenerationalIndexBuffer entities, Vector64Buffer origins) {
  api.entitiesSetOriginRaw(
      indices: entities.buffer.toUint32List(),
      origins: origins.buffer.toFloat32List());
}
