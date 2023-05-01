import 'dart:collection';
import 'dart:typed_data';
import 'dart:ui';

import 'package:vector_math/vector_math_64.dart' as vector64;
import 'package:vector_math/vector_math.dart' as vector32;

import '../api/bridge_generated.dart';
import 'mod.dart';

/// A buffer for storing [Vector2] values.
class Vector32Buffer extends ListBase<vector32.Vector2> {
  final Float32Buffer buffer;

  Vector32Buffer() : buffer = Float32Buffer();

  Vector32Buffer.fromBuffer(Vector32Buffer buffer)
      : buffer = Float32Buffer.fromBuffer(buffer.buffer);

  @override
  int get length => buffer.length ~/ 2;
  @override
  set length(int newLength) {
    buffer.length = newLength * 2;
  }

  @override
  vector32.Vector2 operator [](int index) {
    var indexByTwo = index * 2;
    return vector32.Vector2(
      buffer[indexByTwo],
      buffer[indexByTwo + 1],
    );
  }

  @override
  void operator []=(int index, vector32.Vector2 value) {
    var indexByTwo = index * 2;
    buffer[indexByTwo] = value.x;
    buffer[indexByTwo + 1] = value.y;
  }
}

/// A buffer for storing [Vector2 x 64] values.
class Vector64Buffer extends ListBase<vector64.Vector2> {
  final Float64Buffer buffer;

  Vector64Buffer() : buffer = Float64Buffer();

  Vector64Buffer.fromBuffer(Vector64Buffer buffer)
      : buffer = Float64Buffer.fromBuffer(buffer.buffer);

  @override
  int get length => buffer.length ~/ 2;
  @override
  set length(int newLength) {
    buffer.length = newLength * 2;
  }

  @override
  vector64.Vector2 operator [](int index) {
    var indexByTwo = index * 2;
    return vector64.Vector2(
      buffer[indexByTwo],
      buffer[indexByTwo + 1],
    );
  }

  @override
  void operator []=(int index, vector64.Vector2 value) {
    var indexByTwo = index * 2;
    buffer[indexByTwo] = value.x;
    buffer[indexByTwo + 1] = value.y;
  }
}

/// A buffer for storing [Color] values.
class ColorBuffer extends ListBase<int> {
  final Int32Buffer buffer;

  ColorBuffer() : buffer = Int32Buffer();

  ColorBuffer.fromBuffer(ColorBuffer buffer)
      : buffer = Int32Buffer.fromBuffer(buffer.buffer);

  @override
  int get length => buffer.length;
  @override
  set length(int newLength) {
    buffer.length = newLength;
  }

  @override
  int operator [](int index) {
    return buffer[index];
  }

  @override
  void operator []=(int index, int value) {
    buffer[index] = value;
  }
}

/// A buffer for storing [GenerationalIndex] values.
class GenerationalIndexBuffer extends ListBase<GenerationalIndex> {
  final Uint32Buffer buffer;

  GenerationalIndexBuffer() : buffer = Uint32Buffer();

  GenerationalIndexBuffer.fromBuffer(GenerationalIndexBuffer buffer)
      : buffer = Uint32Buffer.fromBuffer(buffer.buffer);

  @override
  int get length => buffer.length ~/ 2;
  @override
  set length(int newLength) {
    buffer.length = newLength * 2;
  }

  @override
  GenerationalIndex operator [](int index) {
    var indexByTwo = index * 2;
    return GenerationalIndex(
      field0: buffer[indexByTwo],
      field1: buffer[indexByTwo + 1],
    );
  }

  @override
  void operator []=(int index, GenerationalIndex value) {
    var indexByTwo = index * 2;
    buffer[indexByTwo] = value.field0;
    buffer[indexByTwo + 1] = value.field1;
  }
}
