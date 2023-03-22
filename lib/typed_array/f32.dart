import 'dart:collection';

import 'package:flutter_rust_bridge/flutter_rust_bridge.dart';

/// Basically a Float32List that can grow internally and has a few helper methods.
class Float32Buffer with ListMixin<double> {
  late Float32List _buffer;
  int _count = 0;

  int get capacity => _buffer.length;
  int get byteLength => _count * 4;
  int get byteCapacity => _buffer.length * 4;

  // Constructors
  Float32Buffer() {
    _buffer = Float32List(0);
  }

  Float32Buffer.fromList(List<double> list) {
    _buffer = Float32List(list.length);
    _buffer.setAll(0, list);
    _count = list.length;
  }

  Float32Buffer.fromBuffer(Float32Buffer buffer) {
    _buffer = Float32List(buffer.length);
    _buffer.setAll(0, buffer);
    _count = buffer.length;
  }

  Float32Buffer.view(Float32Buffer buffer, int offset, int length) {
    _buffer = Float32List.view(buffer._buffer.buffer, offset * 4, length);
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
      final newBuffer = Float32List(newSize);
      newBuffer.setAll(0, _buffer);
      _buffer = newBuffer;
    }
  }

  void shrinkToFit() {
    if (_count < capacity) {
      final newBuffer = Float32List(_count);
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

  Float32List toFloat32List() {
    return Float32List.view(_buffer.buffer, 0, _count);
  }

  Uint8List toUint8List() {
    return Uint8List.view(_buffer.buffer, 0, _count * 4);
  }
}
