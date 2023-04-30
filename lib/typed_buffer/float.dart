import 'dart:typed_data';

import 'mod.dart';

class Float32Buffer extends TypedBuffer<double> {
  Float32Buffer() : super();
  Float32Buffer.fromBuffer(Float32Buffer buffer) : super.fromBuffer(buffer);
  Float32Buffer.fromList(List<double> list) : super.fromList(list);

  @override
  int byteElementSize() {
    return 4;
  }

  @override
  ByteBuffer createByteBuffer(int length) {
    return Float32List(length).buffer;
  }

  @override
  List<double> asList() {
    return Float32List.view(currentBuffer);
  }
}

class Float64Buffer extends TypedBuffer<double> {
  Float64Buffer() : super();
  Float64Buffer.fromBuffer(Float64Buffer buffer) : super.fromBuffer(buffer);
  Float64Buffer.fromList(List<double> list) : super.fromList(list);

  @override
  int byteElementSize() {
    return 8;
  }

  @override
  ByteBuffer createByteBuffer(int length) {
    return Float64List(length).buffer;
  }

  @override
  List<double> asList() {
    return Float64List.view(currentBuffer);
  }
}
