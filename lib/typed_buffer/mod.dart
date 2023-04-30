import 'dart:collection';
import 'dart:typed_data';

export 'integer.dart';
export 'float.dart';
export 'complex.dart';

abstract class TypedBuffer<T> with ListMixin<T> {
  late ByteBuffer currentBuffer;
  late List<T> _list;
  int _count = 0;

  int get capacity => currentBuffer.lengthInBytes ~/ byteElementSize();
  int get byteLength => _count * byteElementSize();
  int get byteCapacity => currentBuffer.lengthInBytes;

  int byteElementSize();
  ByteBuffer createByteBuffer(int length);
  List<T> asList();

  // Constructors
  TypedBuffer() {
    currentBuffer = createByteBuffer(0);
    _list = asList();
  }

  TypedBuffer.fromBuffer(TypedBuffer<T> buffer) {
    currentBuffer = createByteBuffer(buffer.capacity);
    _list = asList();
    var otherList = buffer._list;
    _list.setAll(0, otherList);
  }

  TypedBuffer.fromList(List<T> list) {
    currentBuffer = createByteBuffer(list.length);
    _list = asList();
    _list.setAll(0, list);
    _count = list.length;
  }

  // List implementation
  @override
  T operator [](int index) {
    return _list[index];
  }

  @override
  void operator []=(int index, T value) {
    _list[index] = value;
  }

  @override
  int get length => _count;
  @override
  set length(int newLength) {
    if (newLength > capacity) {
      growToFit(newLength);
    } else {
      shrinkToFit();
    }
  }

  @override
  void add(T element) {
    growToFit(_count + 1);
    _list[_count] = element;
    _count++;
  }

  @override
  void addAll(Iterable<T> iterable) {
    final length = iterable.length;
    growToFit(_count + length);
    _list.setAll(_count, iterable);
    _count += length;
  }

  @override
  void clear() {
    _count = 0;
  }

  // Capacity management
  void growToFit(int count) {
    if (count > capacity) {
      final newSize = (count * 1.5).ceil();
      final newBuffer = Uint8List(newSize * byteElementSize());
      final originalBuffer = currentBuffer.asUint8List();
      newBuffer.setAll(0, originalBuffer);
      currentBuffer = newBuffer.buffer;
      _list = asList();
    }
  }

  void shrinkToFit() {
    if (_count < capacity) {
      final newBuffer = Uint8List(_count * byteElementSize());
      final originalBuffer = currentBuffer.asUint8List();
      newBuffer.setAll(0, originalBuffer);
      currentBuffer = newBuffer.buffer;
      _list = asList();
    }
  }

  // Utility methods
  void cloneFrom(TypedBuffer<T> buffer) {
    if (buffer.byteLength > byteCapacity) {
      currentBuffer = createByteBuffer(buffer.capacity);
      _list = asList();
    }
    final otherList = buffer._list;
    _list.setAll(0, otherList);
    _count = buffer._count;
  }

  void cloneInto(TypedBuffer<T> buffer) {
    buffer.cloneFrom(this);
  }

  ByteBuffer toByteBuffer() {
    return currentBuffer;
  }

  ByteData toByteData() {
    return ByteData.view(currentBuffer);
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
}
