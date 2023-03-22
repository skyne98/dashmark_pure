import 'dart:collection';

import 'package:dashmark_pure/ffi_export.dart';
import 'package:flutter_rust_bridge/flutter_rust_bridge.dart';

/// Basically a Uint16List that can grow internally and has a few helper methods.
class Uint16Buffer with ListMixin<int> {
  late Uint16List _buffer;
  int _count = 0;

  int get capacity => _buffer.length;
  int get byteLength => _count * 2;
  int get byteCapacity => _buffer.length * 2;

  // Constructors
  Uint16Buffer() {
    _buffer = Uint16List(0);
  }

  Uint16Buffer.fromList(List<int> list) {
    _buffer = Uint16List(list.length);
    _buffer.setAll(0, list);
    _count = list.length;
  }

  Uint16Buffer.fromBuffer(Uint16Buffer buffer) {
    _buffer = Uint16List(buffer.length);
    _buffer.setAll(0, buffer);
    _count = buffer.length;
  }

  Uint16Buffer.view(Uint16Buffer buffer, int offset, int length) {
    _buffer = Uint16List.view(buffer._buffer.buffer, offset * 2, length);
    _count = length;
  }

  // List implementation
  @override
  int operator [](int index) {
    return _buffer[index].toInt();
  }

  @override
  void operator []=(int index, int value) {
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
  void add(int element) {
    growToFit(_count + 1);
    _buffer[_count] = element;
    _count++;
  }

  @override
  void addAll(Iterable<int> iterable) {
    final length = iterable.length;
    growToFit(_count + length);
    _buffer.setAll(_count, iterable);
    _count += length;
  }

  // Capacity management
  void growToFit(int count) {
    if (count > capacity) {
      final newSize = (count * 1.5).ceil();
      final newBuffer = Uint16List(newSize);
      newBuffer.setAll(0, _buffer);
      _buffer = newBuffer;
    }
  }

  void shrinkToFit() {
    if (_count < capacity) {
      final newBuffer = Uint16List(_count);
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

  Uint16List toUint16List() {
    return Uint16List.view(_buffer.buffer, 0, _count);
  }

  Uint8List toUint8List() {
    return Uint8List.view(_buffer.buffer, 0, _count * 2);
  }
}
