import 'dart:collection';

import 'package:dashmark_pure/ffi_export.dart';
import 'package:flutter_rust_bridge/flutter_rust_bridge.dart';

/// Basically a Uint64List that can grow internally and has a few helper methods.
class Uint64Buffer with ListMixin<int> {
  late Uint64List _buffer;
  int _count = 0;

  int get capacity => _buffer.length;
  int get byteLength => _count * 8;
  int get byteCapacity => _buffer.length * 8;

  // Constructors
  Uint64Buffer() {
    _buffer = Uint64List(0);
  }

  Uint64Buffer.fromList(List<int> list) {
    _buffer = Uint64List(list.length);
    _buffer.setAll(0, list.map((e) => BigInt.from(e)));
    _count = list.length;
  }

  Uint64Buffer.fromBuffer(Uint64Buffer buffer) {
    _buffer = Uint64List(buffer.length);
    _buffer.setAll(0, buffer.map((e) => BigInt.from(e)));
    _count = buffer.length;
  }

  Uint64Buffer.view(Uint64Buffer buffer, int offset, int length) {
    _buffer = Uint64List.view(buffer._buffer.inner.buffer, offset * 8, length);
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
    _buffer.setAll(_count, iterable.map((e) => BigInt.from(e)));
    _count += length;
  }

  // Capacity management
  void growToFit(int count) {
    if (count > capacity) {
      final newSize = (count * 1.5).ceil();
      final newBuffer = Uint64List(newSize);
      newBuffer.setAll(0, _buffer);
      _buffer = newBuffer;
    }
  }

  void shrinkToFit() {
    if (_count < capacity) {
      final newBuffer = Uint64List(_count);
      newBuffer.setAll(0, _buffer);
      _buffer = newBuffer;
    }
  }

  // Utility methods
  ByteBuffer toByteBuffer() {
    return _buffer.inner.buffer;
  }

  ByteData toByteData() {
    return ByteData.view(_buffer.inner.buffer);
  }

  Uint64List toUint64List() {
    return Uint64List.view(_buffer.inner.buffer, 0, _count);
  }

  Uint8List toUint8List() {
    return Uint8List.view(_buffer.inner.buffer, 0, _count * 8);
  }
}
