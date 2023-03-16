import 'dart:ui';

import 'package:flutter_rust_bridge/flutter_rust_bridge.dart';

import 'aabb.dart';
import 'ffi.dart' if (dart.library.html) 'ffi_web.dart';

class BVH {
  static final Finalizer<Index> _finalizer =
      Finalizer((bvh) => api.bvhDrop(bvhId: bvh));

  final Index id;
  List<AABB>? aabbs = [];

  BVH(this.id, {this.aabbs}) {
    _finalizer.attach(this, id);
  }

  // Factories
  factory BVH.fromAABBs(List<AABB> aabbs) {
    final aabbIds = aabbs.map((aabb) => aabb.id).toList();
    final id = api.bvhNew(aabbs: aabbIds);
    return BVH(id, aabbs: aabbs);
  }

  static Future<BVH> fromAABBsAsync(List<AABB> aabbs) async {
    final aabbIds = aabbs.map((aabb) => aabb.id).toList();
    final id = await api.bvhNewAsync(aabbs: aabbIds);
    return BVH(id, aabbs: aabbs);
  }

  // Properties
  int get depth {
    return api.bvhDepth(bvhId: id);
  }

  double get overlapRatio {
    return api.bvhOverlapRatio(bvhId: id);
  }

  // Methods
  bool drop() {
    return api.bvhDrop(bvhId: id);
  }

  FlatBVH flatten() {
    return api.bvhFlatten(bvhId: id);
  }

  String print() {
    return api.bvhPrint(bvhId: id);
  }

  List<AABB> queryAABB(AABB aabb) {
    final ids = api.bvhQueryAabbCollisions(bvhId: id, aabbId: aabb.id);
    return ids.toList().map((id) => AABB(id)).toList();
  }

  List<AABB> queryPoint(double x, double y) {
    final ids = api.bvhQueryPointCollisions(bvhId: id, x: x, y: y);
    return ids.toList().map((id) => AABB(id)).toList();
  }
}

void drawFlatBVH(BVH bvh, FlatBVH flat, Canvas canvas) {
  final overallDepth = bvh.depth;
  final paint = Paint()
    ..color = Color.fromARGB(255, 255, 0, 0)
    ..style = PaintingStyle.stroke
    ..strokeWidth = 1.0;

  final length = flat.minX.length;
  for (var i = 0; i < length; i++) {
    final minX = flat.minX[i];
    final minY = flat.minY[i];
    final maxX = flat.maxX[i];
    final maxY = flat.maxY[i];
    final depth = flat.depth[i].toInt();
    final color =
        Color.fromARGB(255, 255, 255 - depth * 255 ~/ overallDepth, 0);
    paint.color = color;
    canvas.drawRect(Rect.fromLTRB(minX, minY, maxX, maxY), paint);
  }
}
