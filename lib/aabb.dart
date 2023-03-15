import 'dart:typed_data';

import 'ffi.dart' if (dart.library.html) 'ffi_web.dart';

class AABB {
  static final Finalizer<AABB> _finalizer =
      Finalizer((aabb) => api.aabbDrop(aabbId: aabb.id));

  final int id;

  AABB(this.id) {
    _finalizer.attach(this, this);
  }

  // Factories
  factory AABB.minMax(double minX, double minY, double maxX, double maxY) {
    final id = api.aabbNew(minX: minX, minY: minY, maxX: maxX, maxY: maxY);
    return AABB(id);
  }

  factory AABB.fromList(List<double> minMax) {
    assert(minMax.length == 4);
    return AABB.minMax(minMax[0], minMax[1], minMax[2], minMax[3]);
  }

  factory AABB.fromXYWH(double x, double y, double width, double height) {
    return AABB.minMax(x, y, x + width, y + height);
  }

  factory AABB.fromXYWHList(List<double> xywh) {
    assert(xywh.length == 4);
    return AABB.fromXYWH(xywh[0], xywh[1], xywh[2], xywh[3]);
  }

  // Properties
  List<double> get min {
    final min = api.aabbMin(aabbId: id);
    return [min[0], min[1]];
  }

  List<double> get max {
    final max = api.aabbMax(aabbId: id);
    return [max[0], max[1]];
  }

  List<double> get center {
    final center = api.aabbCenter(aabbId: id);
    return [center[0], center[1]];
  }

  double get width {
    return max[0] - min[0];
  }

  double get height {
    return max[1] - min[1];
  }

  List<double> get size {
    return [width, height];
  }

  // Methods
  bool contains(double x, double y) {
    return api.aabbContainsPoint(
        aabbId: id, point: Float64List.fromList([x, y]));
  }

  bool containsAABB(AABB other) {
    return api.aabbContainsAabb(aabbLeftId: id, aabbRightId: other.id);
  }

  bool intersectsAABB(AABB other) {
    return api.aabbIntersectsAabb(aabbLeftId: id, aabbRightId: other.id);
  }

  AABB merge(AABB other) {
    final newId = api.aabbMerge(aabbLeftId: id, aabbRightId: other.id);
    return AABB(newId);
  }

  void mergeWith(AABB other) {
    api.aabbMergeWith(aabbId: id, otherId: other.id);
  }
}
