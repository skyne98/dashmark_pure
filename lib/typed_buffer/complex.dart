import 'dart:collection';
import 'dart:ui';

import 'package:vector_math/vector_math_64.dart' as vector64;
import 'package:vector_math/vector_math.dart' as vector32;

import '../api/bridge_generated.dart';
import 'mod.dart';

abstract class ComplexTypedDataView<CT, T> extends ListBase<CT> {
  late TypedBuffer<T> buffer;

  ComplexTypedDataView(this.buffer);

  int elementLength();
  CT debyteify(int index);
  void byteify(int index, CT value);

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
    byteify(start, value);
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
    return vector32.Vector2(buffer[index], buffer[index + 1]);
  }

  @override
  void byteify(int index, vector32.Vector2 value) {
    buffer[index] = value.x;
    buffer[index + 1] = value.y;
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
    return vector64.Vector2(buffer[index], buffer[index + 1]);
  }

  @override
  void byteify(int index, vector64.Vector2 value) {
    buffer[index] = value.x;
    buffer[index + 1] = value.y;
  }
}

class GenerationalIndexBuffer
    extends ComplexTypedDataView<GenerationalIndex, int> {
  GenerationalIndexBuffer() : super(Uint64Buffer());
  GenerationalIndexBuffer.fromBuffer(GenerationalIndexBuffer buffer)
      : super(buffer.buffer);
  GenerationalIndexBuffer.fromList(List<GenerationalIndex> list)
      : super(Uint64Buffer.fromList(
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
  void byteify(int index, GenerationalIndex value) {
    buffer[index] = value.field0;
    buffer[index + 1] = value.field1;
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
  void byteify(int index, Color value) {
    buffer[index] = value.value;
  }
}
