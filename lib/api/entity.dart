import 'package:dashmark_pure/typed_array/f64.dart';
import 'package:dashmark_pure/typed_array/u64.dart';

import '../ffi_export.dart';

void entitySetPosition(RawIndex entity, double x, double y) {
  var indicesBuffer = RawIndexBuffer();
  indicesBuffer.addRawIndex(entity);
  var vectorBuffer = Vector2Buffer();
  vectorBuffer.addXY(x, y);
  api.entitiesSetPositionRaw(
      indices: indicesBuffer.toUint8List(),
      positions: vectorBuffer.toUint8List());
}

void entitySetRotation(RawIndex entity, double angle) {
  var indicesBuffer = RawIndexBuffer();
  indicesBuffer.addRawIndex(entity);
  var anglesBuffer = Float64Buffer();
  anglesBuffer.add(angle);
  api.entitiesSetRotationRaw(
      indices: indicesBuffer.toUint8List(),
      rotations: anglesBuffer.toUint8List());
}
