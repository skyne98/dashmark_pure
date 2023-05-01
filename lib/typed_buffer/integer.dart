import 'dart:typed_data';

import 'mod.dart';

class Uint8Buffer extends DynamicByteBuffer<int> {
  Uint8Buffer() : super();
  Uint8Buffer.fromBuffer(Uint8Buffer buffer) : super.fromBuffer(buffer);

  @override
  int byteElementSize() {
    return 1;
  }

  @override
  ByteBuffer createByteBuffer(int length) {
    return Uint8List(length).buffer;
  }

  @override
  int get(int index) {
    return currentData.getUint8(index * byteElementSize());
  }

  @override
  void set(int index, int value) {
    currentData.setUint8(index * byteElementSize(), value);
  }
}

class Uint16Buffer extends DynamicByteBuffer<int> {
  Uint16Buffer() : super();
  Uint16Buffer.fromBuffer(Uint16Buffer buffer) : super.fromBuffer(buffer);

  @override
  int byteElementSize() {
    return 2;
  }

  @override
  ByteBuffer createByteBuffer(int length) {
    return Uint16List(length).buffer;
  }

  @override
  int get(int index) {
    return currentData.getUint16(index * byteElementSize(), endian);
  }

  @override
  void set(int index, int value) {
    currentData.setUint16(index * byteElementSize(), value, endian);
  }
}

class Uint32Buffer extends DynamicByteBuffer<int> {
  Uint32Buffer() : super();
  Uint32Buffer.fromBuffer(Uint32Buffer buffer) : super.fromBuffer(buffer);

  @override
  int byteElementSize() {
    return 4;
  }

  @override
  ByteBuffer createByteBuffer(int length) {
    return Uint32List(length).buffer;
  }

  @override
  int get(int index) {
    return currentData.getUint32(index * byteElementSize(), endian);
  }

  @override
  void set(int index, int value) {
    currentData.setUint32(index * byteElementSize(), value, endian);
  }
}

class Uint64Buffer extends DynamicByteBuffer<int> {
  Uint64Buffer() : super();
  Uint64Buffer.fromBuffer(Uint64Buffer buffer) : super.fromBuffer(buffer);

  @override
  int byteElementSize() {
    return 8;
  }

  @override
  ByteBuffer createByteBuffer(int length) {
    return Uint64List(length).buffer;
  }

  @override
  int get(int index) {
    return currentData.getUint64(index * byteElementSize(), endian);
  }

  @override
  void set(int index, int value) {
    currentData.setUint64(index * byteElementSize(), value, endian);
  }
}

class Int8Buffer extends DynamicByteBuffer<int> {
  Int8Buffer() : super();
  Int8Buffer.fromBuffer(Int8Buffer buffer) : super.fromBuffer(buffer);

  @override
  int byteElementSize() {
    return 1;
  }

  @override
  ByteBuffer createByteBuffer(int length) {
    return Int8List(length).buffer;
  }

  @override
  int get(int index) {
    return currentData.getInt8(index);
  }

  @override
  void set(int index, int value) {
    currentData.setInt8(index, value);
  }
}

class Int16Buffer extends DynamicByteBuffer<int> {
  Int16Buffer() : super();
  Int16Buffer.fromBuffer(Int16Buffer buffer) : super.fromBuffer(buffer);

  @override
  int byteElementSize() {
    return 2;
  }

  @override
  ByteBuffer createByteBuffer(int length) {
    return Int16List(length).buffer;
  }

  @override
  int get(int index) {
    return currentData.getInt16(index * byteElementSize(), endian);
  }

  @override
  void set(int index, int value) {
    currentData.setInt16(index * byteElementSize(), value, endian);
  }
}

class Int32Buffer extends DynamicByteBuffer<int> {
  Int32Buffer() : super();
  Int32Buffer.fromBuffer(Int32Buffer buffer) : super.fromBuffer(buffer);

  @override
  int byteElementSize() {
    return 4;
  }

  @override
  ByteBuffer createByteBuffer(int length) {
    return Int32List(length).buffer;
  }

  @override
  int get(int index) {
    return currentData.getInt32(index * byteElementSize(), endian);
  }

  @override
  void set(int index, int value) {
    currentData.setInt32(index * byteElementSize(), value, endian);
  }
}

class Int64Buffer extends DynamicByteBuffer<int> {
  Int64Buffer() : super();
  Int64Buffer.fromBuffer(Int64Buffer buffer) : super.fromBuffer(buffer);

  @override
  int byteElementSize() {
    return 8;
  }

  @override
  ByteBuffer createByteBuffer(int length) {
    return Int64List(length).buffer;
  }

  @override
  int get(int index) {
    return currentData.getInt64(index * byteElementSize(), endian);
  }

  @override
  void set(int index, int value) {
    currentData.setInt64(index * byteElementSize(), value, endian);
  }
}
