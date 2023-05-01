import 'dart:typed_data';

import 'mod.dart';

class Float32Buffer extends DynamicByteBuffer<double> {
  Float32Buffer() : super();
  Float32Buffer.fromBuffer(Float32Buffer buffer) : super.fromBuffer(buffer);

  @override
  int byteElementSize() {
    return 4;
  }

  @override
  ByteBuffer createByteBuffer(int length) {
    return Float32List(length).buffer;
  }

  @override
  double get(int index) {
    return currentData.getFloat32(index * byteElementSize(), endian);
  }

  @override
  void set(int index, double value) {
    currentData.setFloat32(index * byteElementSize(), value, endian);
  }
}

class Float64Buffer extends DynamicByteBuffer<double> {
  Float64Buffer() : super();
  Float64Buffer.fromBuffer(Float64Buffer buffer) : super.fromBuffer(buffer);

  @override
  int byteElementSize() {
    return 8;
  }

  @override
  ByteBuffer createByteBuffer(int length) {
    return Float64List(length).buffer;
  }

  @override
  double get(int index) {
    return currentData.getFloat64(index * byteElementSize(), endian);
  }

  @override
  void set(int index, double value) {
    currentData.setFloat64(index * byteElementSize(), value, endian);
  }
}
