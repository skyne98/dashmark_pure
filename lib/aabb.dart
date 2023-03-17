import 'dart:typed_data';

import 'ffi.dart' if (dart.library.html) 'ffi_web.dart';

class AABB {
  static final Finalizer<Index> _finalizer =
      Finalizer((aabb) => api.aabbDropBulk(aabbIds: [aabb]));

  final BigInt index;
  final BigInt generation;

  Index getIndex() {
    return Index(index: index.toInt(), generation: generation.toInt());
  }

  AABB(this.index, this.generation) {
    _finalizer.attach(
        this, Index(index: index.toInt(), generation: generation.toInt()));
  }

  // Factories
  factory AABB.fromIndex(Index index) {
    return AABB(BigInt.from(index.index), BigInt.from(index.generation));
  }

  factory AABB.minMax(double minX, double minY, double maxX, double maxY) {
    final id = api.aabbNew(minX: minX, minY: minY, maxX: maxX, maxY: maxY);
    return AABB(BigInt.from(id.index), BigInt.from(id.generation));
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

  static List<AABB> minMaxRawBulk(Float64List minXs, Float64List minYs,
      Float64List maxXs, Float64List maxYs) {
    assert(minXs.length == minYs.length);
    assert(minXs.length == maxXs.length);
    assert(minXs.length == maxYs.length);
    final ids =
        api.aabbNewBulk(minXs: minXs, minYs: minYs, maxXs: maxXs, maxYs: maxYs);
    final aabbs = <AABB>[];
    for (var i = 0; i < ids.length; i += 2) {
      final idIndex = ids[i];
      final idGeneration = ids[i + 1];
      aabbs.add(AABB(idIndex, idGeneration));
    }
    return aabbs;
  }

  static List<AABB> minMaxBulk(List<double> minXs, List<double> minYs,
      List<double> maxXs, List<double> maxYs) {
    return minMaxRawBulk(
        Float64List.fromList(minXs),
        Float64List.fromList(minYs),
        Float64List.fromList(maxXs),
        Float64List.fromList(maxYs));
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

  static List<bool> dropBulk(List<AABB> aabbs) {
    final ids = aabbs.map((aabb) => aabb.getIndex()).toList();
    return api.aabbDropBulk(aabbIds: ids).map((i) => i == 1).toList();
  }

  // Properties
  List<double> get min {
    final min = api.aabbMin(aabbId: getIndex());
    return [min[0], min[1]];
  }

  List<double> get max {
    final max = api.aabbMax(aabbId: getIndex());
    return [max[0], max[1]];
  }

  List<double> get center {
    final center = api.aabbCenter(aabbId: getIndex());
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
  bool drop() {
    return api.aabbDropBulk(aabbIds: [getIndex()])[0] == 1;
  }

  bool contains(double x, double y) {
    return api.aabbContainsPoint(
        aabbId: getIndex(), point: F64Array2(Float64List.fromList([x, y])));
  }

  bool containsAABB(AABB other) {
    return api.aabbContainsAabb(
        aabbLeftId: getIndex(), aabbRightId: other.getIndex());
  }

  bool intersectsAABB(AABB other) {
    return api.aabbIntersectsAabb(
        aabbLeftId: getIndex(), aabbRightId: other.getIndex());
  }

  AABB merge(AABB other) {
    final newId =
        api.aabbMerge(aabbLeftId: getIndex(), aabbRightId: other.getIndex());
    return AABB.fromIndex(newId);
  }

  void mergeWith(AABB other) {
    api.aabbMergeWith(aabb: getIndex(), other: other.getIndex());
  }
}
