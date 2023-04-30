import 'dart:typed_data';

import 'mod.dart';

class Uint8Buffer extends TypedBuffer<int> {
  Uint8Buffer() : super();
  Uint8Buffer.fromBuffer(Uint8Buffer buffer) : super.fromBuffer(buffer);
  Uint8Buffer.fromList(List<int> list) : super.fromList(list);

  @override
  int byteElementSize() {
    return 1;
  }

  @override
  ByteBuffer createByteBuffer(int length) {
    return Uint8List(length).buffer;
  }

  @override
  List<int> asList() {
    return Uint8List.view(currentBuffer);
  }
}

class Uint16Buffer extends TypedBuffer<int> {
  Uint16Buffer() : super();
  Uint16Buffer.fromBuffer(Uint16Buffer buffer) : super.fromBuffer(buffer);
  Uint16Buffer.fromList(List<int> list) : super.fromList(list);

  @override
  int byteElementSize() {
    return 2;
  }

  @override
  ByteBuffer createByteBuffer(int length) {
    return Uint16List(length).buffer;
  }

  @override
  List<int> asList() {
    return Uint16List.view(currentBuffer);
  }
}

class Uint32Buffer extends TypedBuffer<int> {
  Uint32Buffer() : super();
  Uint32Buffer.fromBuffer(Uint32Buffer buffer) : super.fromBuffer(buffer);
  Uint32Buffer.fromList(List<int> list) : super.fromList(list);

  @override
  int byteElementSize() {
    return 4;
  }

  @override
  ByteBuffer createByteBuffer(int length) {
    return Uint32List(length).buffer;
  }

  @override
  List<int> asList() {
    return Uint32List.view(currentBuffer);
  }
}

class Uint64Buffer extends TypedBuffer<int> {
  Uint64Buffer() : super();
  Uint64Buffer.fromBuffer(Uint64Buffer buffer) : super.fromBuffer(buffer);
  Uint64Buffer.fromList(List<int> list) : super.fromList(list);

  @override
  int byteElementSize() {
    return 8;
  }

  @override
  ByteBuffer createByteBuffer(int length) {
    return Uint64List(length).buffer;
  }

  @override
  List<int> asList() {
    return Uint64List.view(currentBuffer);
  }
}

class Int8Buffer extends TypedBuffer<int> {
  Int8Buffer() : super();
  Int8Buffer.fromBuffer(Int8Buffer buffer) : super.fromBuffer(buffer);
  Int8Buffer.fromList(List<int> list) : super.fromList(list);

  @override
  int byteElementSize() {
    return 1;
  }

  @override
  ByteBuffer createByteBuffer(int length) {
    return Int8List(length).buffer;
  }

  @override
  List<int> asList() {
    return Int8List.view(currentBuffer);
  }
}

class Int16Buffer extends TypedBuffer<int> {
  Int16Buffer() : super();
  Int16Buffer.fromBuffer(Int16Buffer buffer) : super.fromBuffer(buffer);
  Int16Buffer.fromList(List<int> list) : super.fromList(list);

  @override
  int byteElementSize() {
    return 2;
  }

  @override
  ByteBuffer createByteBuffer(int length) {
    return Int16List(length).buffer;
  }

  @override
  List<int> asList() {
    return Int16List.view(currentBuffer);
  }
}

class Int32Buffer extends TypedBuffer<int> {
  Int32Buffer() : super();
  Int32Buffer.fromBuffer(Int32Buffer buffer) : super.fromBuffer(buffer);
  Int32Buffer.fromList(List<int> list) : super.fromList(list);

  @override
  int byteElementSize() {
    return 4;
  }

  @override
  ByteBuffer createByteBuffer(int length) {
    return Int32List(length).buffer;
  }

  @override
  List<int> asList() {
    return Int32List.view(currentBuffer);
  }
}

class Int64Buffer extends TypedBuffer<int> {
  Int64Buffer() : super();
  Int64Buffer.fromBuffer(Int64Buffer buffer) : super.fromBuffer(buffer);
  Int64Buffer.fromList(List<int> list) : super.fromList(list);

  @override
  int byteElementSize() {
    return 8;
  }

  @override
  ByteBuffer createByteBuffer(int length) {
    return Int64List(length).buffer;
  }

  @override
  List<int> asList() {
    return Int64List.view(currentBuffer);
  }
}
