import 'package:flutter_rust_bridge/flutter_rust_bridge.dart';

import 'aabb.dart';
import 'ffi.dart' if (dart.library.html) 'ffi_web.dart';

class BVH {
  static final Finalizer<BVH> _finalizer =
      Finalizer((bvh) => api.bvhDrop(bvhId: bvh.id));

  final int id;

  BVH(this.id) {
    _finalizer.attach(this, this);
  }

  // Factories
  factory BVH.fromAABBs(List<AABB> aabbs) {
    final aabbIds = aabbs.map((aabb) => aabb.id).toList();
    final aabbIdsUint64 = Uint64List.fromList(aabbIds);
    final id = api.bvhNew(aabbs: aabbIdsUint64);
    return BVH(id);
  }

  static Future<BVH> fromAABBsAsync(List<AABB> aabbs) async {
    final aabbIds = aabbs.map((aabb) => aabb.id).toList();
    final aabbIdsUint64 = Uint64List.fromList(aabbIds);
    final id = await api.bvhNewAsync(aabbs: aabbIdsUint64);
    return BVH(id);
  }

  // Properties
  int get depth {
    return api.bvhDepth(bvhId: id);
  }

  // Methods
  FlatBVH flatten() {
    return api.bvhFlatten(bvhId: id);
  }

  String print() {
    return api.bvhPrint(bvhId: id);
  }

  List<AABB> queryAABB(AABB aabb) {
    final ids = api.bvhQueryAabbCollisions(bvhId: id, aabbId: aabb.id);
    return ids.toList().map((id) => AABB(id.toInt())).toList();
  }

  List<AABB> queryPoint(double x, double y) {
    final ids = api.bvhQueryPointCollisions(bvhId: id, x: x, y: y);
    return ids.toList().map((id) => AABB(id.toInt())).toList();
  }
}
