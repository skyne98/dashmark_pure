import 'dart:collection';
import 'dart:typed_data';
import 'dart:ui';

import 'package:dashmark_pure/api/encoding.dart';
import 'package:vector_math/vector_math_64.dart' as vector64;
import 'package:vector_math/vector_math.dart' as vector32;

import '../api/bridge_generated.dart';
import 'mod.dart';

abstract class ComplexTypedDataView<CT, T> extends ListBase<CT> {
  late TypedBuffer<T> buffer;

  ComplexTypedDataView(this.buffer);

  int elementLength();
  CT debyteify(int index);
  List<T> byteify(CT value);

  @override
  int get length => buffer.length ~/ elementLength();

  @override
  set length(int newLength) {
    buffer.length = newLength * elementLength();
  }

  @override
  CT operator [](int index) {
    var start = index * elementLength();
    return debyteify(start);
  }

  @override
  void operator []=(int index, CT value) {
    var start = index * elementLength();
    var bytes = byteify(value);
    for (var i = 0; i < elementLength(); i++) {
      buffer[start + i] = bytes[i];
    }
  }

  @override
  void add(CT element) {
    buffer.addAll(byteify(element));
  }

  @override
  void addAll(Iterable<CT> iterable) {
    buffer.addAll(iterable.expand((v) => byteify(v)));
  }
}

class VectorBuffer extends ComplexTypedDataView<vector32.Vector2, double> {
  VectorBuffer() : super(Float32Buffer());
  VectorBuffer.fromBuffer(VectorBuffer buffer) : super(buffer.buffer);
  VectorBuffer.fromList(List<vector32.Vector2> list)
      : super(Float32Buffer.fromList(list.expand((v) => [v.x, v.y]).toList()));

  @override
  int elementLength() {
    return 2;
  }

  @override
  vector32.Vector2 debyteify(int index) {
    return vector32.Vector2.fromBuffer(
        buffer.currentBuffer, index * Float32List.bytesPerElement);
  }

  @override
  Float32List byteify(vector32.Vector2 value) {
    return value.storage;
  }
}

class Vector64Buffer extends ComplexTypedDataView<vector64.Vector2, double> {
  Vector64Buffer() : super(Float64Buffer());
  Vector64Buffer.fromBuffer(Vector64Buffer buffer) : super(buffer.buffer);
  Vector64Buffer.fromList(List<vector64.Vector2> list)
      : super(Float64Buffer.fromList(list.expand((v) => [v.x, v.y]).toList()));

  @override
  int elementLength() {
    return 2;
  }

  @override
  vector64.Vector2 debyteify(int index) {
    return vector64.Vector2.fromBuffer(
        buffer.currentBuffer, index * Float64List.bytesPerElement);
  }

  @override
  Float64List byteify(vector64.Vector2 value) {
    return value.storage;
  }
}

class GenerationalIndexBuffer
    extends ComplexTypedDataView<GenerationalIndex, int> {
  GenerationalIndexBuffer() : super(Uint32Buffer());
  GenerationalIndexBuffer.fromBuffer(GenerationalIndexBuffer buffer)
      : super(buffer.buffer);
  GenerationalIndexBuffer.fromList(List<GenerationalIndex> list)
      : super(Uint32Buffer.fromList(
            list.expand((v) => [v.field0, v.field1]).toList()));

  @override
  int elementLength() {
    return 2;
  }

  @override
  GenerationalIndex debyteify(int index) {
    return GenerationalIndex(field0: buffer[index], field1: buffer[index + 1]);
  }

  @override
  Uint32List byteify(GenerationalIndex value) {
    return Uint32List.fromList([value.field0, value.field1]);
  }
}

class ColorBuffer extends ComplexTypedDataView<Color, int> {
  ColorBuffer() : super(Int32Buffer());
  ColorBuffer.fromBuffer(ColorBuffer buffer) : super(buffer.buffer);
  ColorBuffer.fromList(List<Color> list)
      : super(Int32Buffer.fromList(list.map((e) => e.value).toList()));

  @override
  int elementLength() {
    return 1;
  }

  @override
  Color debyteify(int index) {
    return Color(buffer[index]);
  }

  @override
  Int32List byteify(Color value) {
    return Int32List.fromList([value.value]);
  }
}
