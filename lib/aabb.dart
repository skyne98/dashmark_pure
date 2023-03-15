import 'dart:typed_data';

import 'ffi.dart' if (dart.library.html) 'ffi_web.dart';

class AABB {
  static final Finalizer<int> _finalizer =
      Finalizer((aabb) => api.aabbDrop(aabbId: aabb));

  final int id;

  AABB(this.id) {
    _finalizer.attach(this, id);
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

  static List<AABB> minMaxBulk(List<double> minXs, List<double> minYs,
      List<double> maxXs, List<double> maxYs) {
    assert(minXs.length == minYs.length);
    assert(minXs.length == maxXs.length);
    assert(minXs.length == maxYs.length);
    final ids = api.aabbNewBulk(
        minXs: Float64List.fromList(minXs),
        minYs: Float64List.fromList(minYs),
        maxXs: Float64List.fromList(maxXs),
        maxYs: Float64List.fromList(maxYs));
    return ids.toList().map((id) => AABB(id.toInt())).toList();
  }

  static List<AABB> fromListBulk(List<List<double>> minMaxs) {
    final minXs = <double>[];
    final minYs = <double>[];
    final maxXs = <double>[];
    final maxYs = <double>[];
    for (final minMax in minMaxs) {
      assert(minMax.length == 4);
      minXs.add(minMax[0]);
      minYs.add(minMax[1]);
      maxXs.add(minMax[2]);
      maxYs.add(minMax[3]);
    }
    return minMaxBulk(minXs, minYs, maxXs, maxYs);
  }

  static List<AABB> fromXYWHBulk(List<double> xs, List<double> ys,
      List<double> widths, List<double> heights) {
    assert(xs.length == ys.length);
    assert(xs.length == widths.length);
    assert(xs.length == heights.length);
    final minXs = <double>[];
    final minYs = <double>[];
    final maxXs = <double>[];
    final maxYs = <double>[];
    for (var i = 0; i < xs.length; ++i) {
      final x = xs[i];
      final y = ys[i];
      final width = widths[i];
      final height = heights[i];
      minXs.add(x);
      minYs.add(y);
      maxXs.add(x + width);
      maxYs.add(y + height);
    }
    return minMaxBulk(minXs, minYs, maxXs, maxYs);
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
