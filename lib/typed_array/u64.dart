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

  Uint64Buffer.fromBytes(Uint8List bytes) {
    _buffer = Uint64List.view(bytes.buffer);
    _count = bytes.length ~/ 8;
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

class RawIndexBuffer extends Uint64Buffer {
  int get indicesLength => _count;

  RawIndexBuffer() : super();

  RawIndexBuffer.fromBytes(Uint8List bytes) : super() {
    _buffer = Uint64List.view(bytes.buffer);
    _count = bytes.length ~/ 8;
  }

  RawIndexBuffer.fromList(List<int> list) : super() {
    _buffer = Uint64List(list.length);
    _buffer.setAll(0, list.map((e) => BigInt.from(e)));
    _count = list.length;
  }

  RawIndexBuffer.fromBuffer(RawIndexBuffer buffer) : super() {
    _buffer = Uint64List(buffer.length);
    _buffer.setAll(0, buffer.map((e) => BigInt.from(e)));
    _count = buffer.length;
  }

  RawIndexBuffer.view(RawIndexBuffer buffer, int offset, int length) : super() {
    _buffer = Uint64List.view(buffer._buffer.inner.buffer, offset * 8, length);
    _count = length;
  }

  // Utility methods
  int getIndex(int index) {
    return _buffer[index * 2] as int;
  }

  int getGeneration(int index) {
    return _buffer[index * 2 + 1] as int;
  }

  void setIndex(int index, int value) {
    _buffer[index * 2] = value;
  }

  void setGeneration(int index, int value) {
    _buffer[index * 2 + 1] = value;
  }

  void set(int index, int indexValue, int generationValue) {
    final offset = index * 2;
    _buffer[offset] = indexValue;
    _buffer[offset + 1] = generationValue;
  }

  void addRawIndex(RawIndex index) {
    growToFit(_count + 2);
    _buffer[_count] = index.field0;
    _buffer[_count + 1] = index.field1;
    _count += 2;
  }

  RawIndex toRawIndex(int index) {
    final offset = index * 2;
    return RawIndex(
        field0: _buffer[offset] as int, field1: _buffer[offset + 1] as int);
  }
}
