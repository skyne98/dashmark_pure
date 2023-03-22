import 'dart:collection';

import 'package:flutter_rust_bridge/flutter_rust_bridge.dart';
import 'package:vector_math/vector_math_64.dart';

/// Basically a Float64List that can grow internally and has a few helper methods.
class Float64Buffer with ListMixin<double> {
  late Float64List _buffer;
  int _count = 0;

  int get capacity => _buffer.length;
  int get byteLength => _count * 8;
  int get byteCapacity => _buffer.length * 8;

  // Constructors
  Float64Buffer() {
    _buffer = Float64List(0);
  }

  Float64Buffer.fromList(List<double> list) {
    _buffer = Float64List(list.length);
    _buffer.setAll(0, list);
    _count = list.length;
  }

  Float64Buffer.fromBuffer(Float64Buffer buffer) {
    _buffer = Float64List(buffer.length);
    _buffer.setAll(0, buffer);
    _count = buffer.length;
  }

  Float64Buffer.view(Float64Buffer buffer, int offset, int length) {
    _buffer = Float64List.view(buffer._buffer.buffer, offset * 8, length);
    _count = length;
  }

  // List implementation
  @override
  double operator [](int index) {
    return _buffer[index];
  }

  @override
  void operator []=(int index, double value) {
    _buffer[index] = value;
  }

  @override
  int get length => _count;
  @override
  set length(int newLength) {
    if (newLength > _buffer.length) {
      growToFit(newLength);
    } else {
      shrinkToFit();
    }
  }

  @override
  void add(double element) {
    growToFit(_count + 1);
    _buffer[_count] = element;
    _count++;
  }

  @override
  void addAll(Iterable<double> iterable) {
    final length = iterable.length;
    growToFit(_count + length);
    _buffer.setAll(_count, iterable);
    _count += length;
  }

  // Capacity management
  void growToFit(int count) {
    if (count > capacity) {
      final newSize = (count * 1.5).ceil();
      final newBuffer = Float64List(newSize);
      newBuffer.setAll(0, _buffer);
      _buffer = newBuffer;
    }
  }

  void shrinkToFit() {
    if (_count < capacity) {
      final newBuffer = Float64List(_count);
      newBuffer.setAll(0, _buffer);
      _buffer = newBuffer;
    }
  }

  // Utility methods
  ByteBuffer toByteBuffer() {
    return _buffer.buffer;
  }

  ByteData toByteData() {
    return ByteData.view(_buffer.buffer);
  }

  Float64List toFloat64List() {
    return Float64List.view(_buffer.buffer, 0, _count);
  }

  Uint8List toUint8List() {
    return Uint8List.view(_buffer.buffer, 0, _count * 8);
  }
}

class Vector2Buffer extends Float64Buffer {
  int get vectorsLength => _count ~/ 2;

  Vector2Buffer() : super();

  Vector2Buffer.fromList(List<Vector2> list) : super() {
    growToFit(list.length * 2);
    for (final vector in list) {
      add(vector.x);
      add(vector.y);
    }
    _count = list.length;
  }

  Vector2Buffer.fromBuffer(Vector2Buffer buffer) : super.fromBuffer(buffer);

  Vector2Buffer.view(Vector2Buffer buffer, int offset, int length)
      : super.view(buffer, offset * 2, length);

  // Utilities to get and set specific pairs of vectors
  double getX(int index) {
    return _buffer[index * 2];
  }

  double getY(int index) {
    return _buffer[index * 2 + 1];
  }

  void setX(int index, double value) {
    _buffer[index * 2] = value;
  }

  void setY(int index, double value) {
    _buffer[index * 2 + 1] = value;
  }

  void setXY(int index, double x, double y) {
    final offset = index * 2;
    _buffer[offset] = x;
    _buffer[offset + 1] = y;
  }

  void addXY(double x, double y) {
    growToFit(_count + 2);
    _buffer[_count] = x;
    _buffer[_count + 1] = y;
    _count += 2;
  }
}
