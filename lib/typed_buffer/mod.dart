import 'dart:collection';
import 'dart:typed_data';

export 'integer.dart';
export 'float.dart';
export 'complex.dart';

void _copyByteBuffer(ByteBuffer source, ByteBuffer destination) {
  final sourceData = source.asByteData();
  final destinationData = destination.asByteData();

  for (var i = 0; i < source.lengthInBytes; i++) {
    destinationData.setUint8(i, sourceData.getUint8(i));
  }
}

abstract class DynamicByteBuffer<T> extends ListBase<T> {
  late ByteBuffer currentBuffer;
  late ByteData currentData;
  int _count = 0;
  Endian endian = Endian.host;

  int get capacity => currentBuffer.lengthInBytes ~/ byteElementSize();
  int get byteLength => _count * byteElementSize();
  int get byteCapacity => currentBuffer.lengthInBytes;

  int byteElementSize();
  ByteBuffer createByteBuffer(int length);
  T get(int index);
  void set(int index, T value);

  // Constructors
  DynamicByteBuffer() {
    currentBuffer = createByteBuffer(0);
    currentData = currentBuffer.asByteData();
  }

  DynamicByteBuffer.fromByteBuffer(ByteBuffer buffer, [int count = 0]) {
    // Copy the buffer
    currentBuffer = createByteBuffer(buffer.lengthInBytes);
    currentData = currentBuffer.asByteData();
    _copyByteBuffer(buffer, currentBuffer);
    // Set the count
    _count = count;
  }

  DynamicByteBuffer.fromBuffer(DynamicByteBuffer buffer) {
    currentBuffer = createByteBuffer(buffer.capacity);
    currentData = currentBuffer.asByteData();
    _copyByteBuffer(buffer.currentBuffer, currentBuffer);
  }

  @override
  int get length => _count;
  @override
  set length(int newLength) {
    growToFit(newLength);
    _count = newLength;
  }

  // Capacity management
  void growToFit(int count) {
    if (count > capacity) {
      final newSize = (count * 1.5).ceil();
      final newBuffer = Uint8List(newSize * byteElementSize());
      _copyByteBuffer(currentBuffer, newBuffer.buffer);
      currentBuffer = newBuffer.buffer;
      currentData = currentBuffer.asByteData();
    }
  }

  // List methods
  @override
  void clear() {
    _count = 0;
  }

  @override
  void add(T element) {
    var oldLength = length;
    length++;
    set(oldLength, element);
  }

  @override
  void addAll(Iterable<T> iterable) {
    var oldLength = length;
    length += iterable.length;
    int i = 0;
    for (var element in iterable) {
      set(oldLength + i, element);
      i++;
    }
  }

  T pop() {
    var value = get(length - 1);
    length--;
    return value;
  }

  @override
  void insert(int index, T element) {
    if (index >= length) {
      length = index + 1;
    }
    set(index, element);
  }

  // Operators
  @override
  T operator [](int index) {
    return get(index);
  }

  @override
  void operator []=(int index, T value) {
    set(index, value);
  }

  // Utility methods
  void cloneFrom(DynamicByteBuffer buffer) {
    if (buffer.byteLength > byteCapacity) {
      currentBuffer = createByteBuffer(buffer.capacity);
      currentData = currentBuffer.asByteData();
    }
    _copyByteBuffer(buffer.currentBuffer, currentBuffer);
  }

  void cloneFromIterable(Iterable<T> iterable) {
    clear();
    length += iterable.length;
    int i = 0;
    for (var element in iterable) {
      set(i, element);
      i++;
    }
  }

  void cloneInto(DynamicByteBuffer buffer) {
    buffer.cloneFrom(this);
  }

  ByteBuffer byteBuffer() {
    return currentBuffer;
  }

  ByteData byteData() {
    return currentData;
  }

  Float32List toFloat32List() {
    return Float32List.view(currentBuffer, 0, byteLength ~/ 4);
  }

  Float64List toFloat64List() {
    return Float64List.view(currentBuffer, 0, byteLength ~/ 8);
  }

  Uint8List toUint8List() {
    return Uint8List.view(currentBuffer, 0, byteLength);
  }

  Int8List toInt8List() {
    return Int8List.view(currentBuffer, 0, byteLength);
  }

  Uint16List toUint16List() {
    return Uint16List.view(currentBuffer, 0, byteLength ~/ 2);
  }

  Int16List toInt16List() {
    return Int16List.view(currentBuffer, 0, byteLength ~/ 2);
  }

  Uint32List toUint32List() {
    return Uint32List.view(currentBuffer, 0, byteLength ~/ 4);
  }

  Int32List toInt32List() {
    return Int32List.view(currentBuffer, 0, byteLength ~/ 4);
  }

  Uint64List toUint64List() {
    return Uint64List.view(currentBuffer, 0, byteLength ~/ 8);
  }

  Int64List toInt64List() {
    return Int64List.view(currentBuffer, 0, byteLength ~/ 8);
  }

  // Getters and setters
  int getUint8(int index) {
    return currentData.getUint8(index);
  }

  void setUint8(int index, int value) {
    currentData.setUint8(index, value);
  }

  int getInt8(int index) {
    return currentData.getInt8(index);
  }

  void setInt8(int index, int value) {
    currentData.setInt8(index, value);
  }

  int getUint16(int index) {
    return currentData.getUint16(index, endian);
  }

  void setUint16(int index, int value) {
    currentData.setUint16(index, value, endian);
  }

  int getInt16(int index) {
    return currentData.getInt16(index, endian);
  }

  void setInt16(int index, int value) {
    currentData.setInt16(index, value, endian);
  }

  int getUint32(int index) {
    return currentData.getUint32(index, endian);
  }

  void setUint32(int index, int value) {
    currentData.setUint32(index, value, endian);
  }

  int getInt32(int index) {
    return currentData.getInt32(index, endian);
  }

  void setInt32(int index, int value) {
    currentData.setInt32(index, value, endian);
  }

  int getUint64(int index) {
    return currentData.getUint64(index, endian);
  }

  void setUint64(int index, int value) {
    currentData.setUint64(index, value, endian);
  }

  int getInt64(int index) {
    return currentData.getInt64(index, endian);
  }

  void setInt64(int index, int value) {
    currentData.setInt64(index, value, endian);
  }

  double getFloat32(int index) {
    return currentData.getFloat32(index, endian);
  }

  void setFloat32(int index, double value) {
    currentData.setFloat32(index, value, endian);
  }

  double getFloat64(int index) {
    return currentData.getFloat64(index, endian);
  }

  void setFloat64(int index, double value) {
    currentData.setFloat64(index, value, endian);
  }
}
