// AUTO GENERATED FILE, DO NOT EDIT.
// Generated by `flutter_rust_bridge`@ 1.69.0.
// ignore_for_file: non_constant_identifier_names, unused_element, duplicate_ignore, directives_ordering, curly_braces_in_flow_control_structures, unnecessary_lambdas, slash_for_doc_comments, prefer_const_literals_to_create_immutables, implicit_dynamic_list_literal, duplicate_import, unused_import, unnecessary_import, prefer_single_quotes, prefer_const_constructors, use_super_parameters, always_use_package_imports, annotate_overrides, invalid_use_of_protected_member, constant_identifier_names, invalid_use_of_internal_member, prefer_is_empty, unnecessary_const

import 'dart:convert';
import 'dart:async';
import 'package:meta/meta.dart';
import 'package:flutter_rust_bridge/flutter_rust_bridge.dart';
import 'bridge_generated.dart';
export 'bridge_generated.dart';
import 'dart:ffi' as ffi;

class NativePlatform extends FlutterRustBridgeBase<NativeWire> {
  NativePlatform(ffi.DynamicLibrary dylib) : super(NativeWire(dylib));

// Section: api2wire

  @protected
  wire_RwLockAabb api2wire_RwLockAabb(RwLockAabb raw) {
    final ptr = inner.new_RwLockAabb();
    _api_fill_to_wire_RwLockAabb(raw, ptr);
    return ptr;
  }

  @protected
  ffi.Pointer<wire_float_64_list> api2wire_float_64_list(Float64List raw) {
    final ans = inner.new_float_64_list_0(raw.length);
    ans.ref.ptr.asTypedList(raw.length).setAll(0, raw);
    return ans;
  }

  @protected
  ffi.Pointer<wire_list_RwLockAabb> api2wire_list_RwLockAabb(
      List<RwLockAabb> raw) {
    final ans = inner.new_list_RwLockAabb_0(raw.length);
    for (var i = 0; i < raw.length; ++i) {
      _api_fill_to_wire_RwLockAabb(raw[i], ans.ref.ptr[i]);
    }
    return ans;
  }
// Section: finalizer

  late final OpaqueTypeFinalizer _RwLockAabbFinalizer =
      OpaqueTypeFinalizer(inner._drop_opaque_RwLockAabbPtr);
  OpaqueTypeFinalizer get RwLockAabbFinalizer => _RwLockAabbFinalizer;
  late final OpaqueTypeFinalizer _RwLockBvhFinalizer =
      OpaqueTypeFinalizer(inner._drop_opaque_RwLockBvhPtr);
  OpaqueTypeFinalizer get RwLockBvhFinalizer => _RwLockBvhFinalizer;
// Section: api_fill_to_wire

  void _api_fill_to_wire_RwLockAabb(
      RwLockAabb apiObj, wire_RwLockAabb wireObj) {
    wireObj.ptr = apiObj.shareOrMove();
  }
}

// ignore_for_file: camel_case_types, non_constant_identifier_names, avoid_positional_boolean_parameters, annotate_overrides, constant_identifier_names

// AUTO GENERATED FILE, DO NOT EDIT.
//
// Generated by `package:ffigen`.

/// generated by flutter_rust_bridge
class NativeWire implements FlutterRustBridgeWireBase {
  @internal
  late final dartApi = DartApiDl(init_frb_dart_api_dl);

  /// Holds the symbol lookup function.
  final ffi.Pointer<T> Function<T extends ffi.NativeType>(String symbolName)
      _lookup;

  /// The symbols are looked up in [dynamicLibrary].
  NativeWire(ffi.DynamicLibrary dynamicLibrary)
      : _lookup = dynamicLibrary.lookup;

  /// The symbols are looked up with [lookup].
  NativeWire.fromLookup(
      ffi.Pointer<T> Function<T extends ffi.NativeType>(String symbolName)
          lookup)
      : _lookup = lookup;

  void store_dart_post_cobject(
    DartPostCObjectFnType ptr,
  ) {
    return _store_dart_post_cobject(
      ptr,
    );
  }

  late final _store_dart_post_cobjectPtr =
      _lookup<ffi.NativeFunction<ffi.Void Function(DartPostCObjectFnType)>>(
          'store_dart_post_cobject');
  late final _store_dart_post_cobject = _store_dart_post_cobjectPtr
      .asFunction<void Function(DartPostCObjectFnType)>();

  Object get_dart_object(
    int ptr,
  ) {
    return _get_dart_object(
      ptr,
    );
  }

  late final _get_dart_objectPtr =
      _lookup<ffi.NativeFunction<ffi.Handle Function(ffi.UintPtr)>>(
          'get_dart_object');
  late final _get_dart_object =
      _get_dart_objectPtr.asFunction<Object Function(int)>();

  void drop_dart_object(
    int ptr,
  ) {
    return _drop_dart_object(
      ptr,
    );
  }

  late final _drop_dart_objectPtr =
      _lookup<ffi.NativeFunction<ffi.Void Function(ffi.UintPtr)>>(
          'drop_dart_object');
  late final _drop_dart_object =
      _drop_dart_objectPtr.asFunction<void Function(int)>();

  int new_dart_opaque(
    Object handle,
  ) {
    return _new_dart_opaque(
      handle,
    );
  }

  late final _new_dart_opaquePtr =
      _lookup<ffi.NativeFunction<ffi.UintPtr Function(ffi.Handle)>>(
          'new_dart_opaque');
  late final _new_dart_opaque =
      _new_dart_opaquePtr.asFunction<int Function(Object)>();

  int init_frb_dart_api_dl(
    ffi.Pointer<ffi.Void> obj,
  ) {
    return _init_frb_dart_api_dl(
      obj,
    );
  }

  late final _init_frb_dart_api_dlPtr =
      _lookup<ffi.NativeFunction<ffi.IntPtr Function(ffi.Pointer<ffi.Void>)>>(
          'init_frb_dart_api_dl');
  late final _init_frb_dart_api_dl = _init_frb_dart_api_dlPtr
      .asFunction<int Function(ffi.Pointer<ffi.Void>)>();

  void wire_say_hello_async(
    int port_,
  ) {
    return _wire_say_hello_async(
      port_,
    );
  }

  late final _wire_say_hello_asyncPtr =
      _lookup<ffi.NativeFunction<ffi.Void Function(ffi.Int64)>>(
          'wire_say_hello_async');
  late final _wire_say_hello_async =
      _wire_say_hello_asyncPtr.asFunction<void Function(int)>();

  void wire_morton_codes_async(
    int port_,
    ffi.Pointer<wire_float_64_list> xs,
    ffi.Pointer<wire_float_64_list> ys,
  ) {
    return _wire_morton_codes_async(
      port_,
      xs,
      ys,
    );
  }

  late final _wire_morton_codes_asyncPtr = _lookup<
      ffi.NativeFunction<
          ffi.Void Function(ffi.Int64, ffi.Pointer<wire_float_64_list>,
              ffi.Pointer<wire_float_64_list>)>>('wire_morton_codes_async');
  late final _wire_morton_codes_async = _wire_morton_codes_asyncPtr.asFunction<
      void Function(int, ffi.Pointer<wire_float_64_list>,
          ffi.Pointer<wire_float_64_list>)>();

  WireSyncReturn wire_morton_codes(
    ffi.Pointer<wire_float_64_list> xs,
    ffi.Pointer<wire_float_64_list> ys,
  ) {
    return _wire_morton_codes(
      xs,
      ys,
    );
  }

  late final _wire_morton_codesPtr = _lookup<
      ffi.NativeFunction<
          WireSyncReturn Function(ffi.Pointer<wire_float_64_list>,
              ffi.Pointer<wire_float_64_list>)>>('wire_morton_codes');
  late final _wire_morton_codes = _wire_morton_codesPtr.asFunction<
      WireSyncReturn Function(
          ffi.Pointer<wire_float_64_list>, ffi.Pointer<wire_float_64_list>)>();

  WireSyncReturn wire_aabb_new(
    double min_x,
    double min_y,
    double max_x,
    double max_y,
  ) {
    return _wire_aabb_new(
      min_x,
      min_y,
      max_x,
      max_y,
    );
  }

  late final _wire_aabb_newPtr = _lookup<
      ffi.NativeFunction<
          WireSyncReturn Function(ffi.Double, ffi.Double, ffi.Double,
              ffi.Double)>>('wire_aabb_new');
  late final _wire_aabb_new = _wire_aabb_newPtr
      .asFunction<WireSyncReturn Function(double, double, double, double)>();

  WireSyncReturn wire_aabb_new_bulk(
    ffi.Pointer<wire_float_64_list> min_xs,
    ffi.Pointer<wire_float_64_list> min_ys,
    ffi.Pointer<wire_float_64_list> max_xs,
    ffi.Pointer<wire_float_64_list> max_ys,
  ) {
    return _wire_aabb_new_bulk(
      min_xs,
      min_ys,
      max_xs,
      max_ys,
    );
  }

  late final _wire_aabb_new_bulkPtr = _lookup<
      ffi.NativeFunction<
          WireSyncReturn Function(
              ffi.Pointer<wire_float_64_list>,
              ffi.Pointer<wire_float_64_list>,
              ffi.Pointer<wire_float_64_list>,
              ffi.Pointer<wire_float_64_list>)>>('wire_aabb_new_bulk');
  late final _wire_aabb_new_bulk = _wire_aabb_new_bulkPtr.asFunction<
      WireSyncReturn Function(
          ffi.Pointer<wire_float_64_list>,
          ffi.Pointer<wire_float_64_list>,
          ffi.Pointer<wire_float_64_list>,
          ffi.Pointer<wire_float_64_list>)>();

  WireSyncReturn wire_aabb_new_bulk_benchmark(
    ffi.Pointer<wire_float_64_list> min_xs,
    ffi.Pointer<wire_float_64_list> min_ys,
    ffi.Pointer<wire_float_64_list> max_xs,
    ffi.Pointer<wire_float_64_list> max_ys,
  ) {
    return _wire_aabb_new_bulk_benchmark(
      min_xs,
      min_ys,
      max_xs,
      max_ys,
    );
  }

  late final _wire_aabb_new_bulk_benchmarkPtr = _lookup<
          ffi.NativeFunction<
              WireSyncReturn Function(
                  ffi.Pointer<wire_float_64_list>,
                  ffi.Pointer<wire_float_64_list>,
                  ffi.Pointer<wire_float_64_list>,
                  ffi.Pointer<wire_float_64_list>)>>(
      'wire_aabb_new_bulk_benchmark');
  late final _wire_aabb_new_bulk_benchmark =
      _wire_aabb_new_bulk_benchmarkPtr.asFunction<
          WireSyncReturn Function(
              ffi.Pointer<wire_float_64_list>,
              ffi.Pointer<wire_float_64_list>,
              ffi.Pointer<wire_float_64_list>,
              ffi.Pointer<wire_float_64_list>)>();

  WireSyncReturn wire_aabb_min(
    wire_RwLockAabb aabb,
  ) {
    return _wire_aabb_min(
      aabb,
    );
  }

  late final _wire_aabb_minPtr =
      _lookup<ffi.NativeFunction<WireSyncReturn Function(wire_RwLockAabb)>>(
          'wire_aabb_min');
  late final _wire_aabb_min =
      _wire_aabb_minPtr.asFunction<WireSyncReturn Function(wire_RwLockAabb)>();

  WireSyncReturn wire_aabb_max(
    wire_RwLockAabb aabb,
  ) {
    return _wire_aabb_max(
      aabb,
    );
  }

  late final _wire_aabb_maxPtr =
      _lookup<ffi.NativeFunction<WireSyncReturn Function(wire_RwLockAabb)>>(
          'wire_aabb_max');
  late final _wire_aabb_max =
      _wire_aabb_maxPtr.asFunction<WireSyncReturn Function(wire_RwLockAabb)>();

  WireSyncReturn wire_aabb_size(
    wire_RwLockAabb aabb,
  ) {
    return _wire_aabb_size(
      aabb,
    );
  }

  late final _wire_aabb_sizePtr =
      _lookup<ffi.NativeFunction<WireSyncReturn Function(wire_RwLockAabb)>>(
          'wire_aabb_size');
  late final _wire_aabb_size =
      _wire_aabb_sizePtr.asFunction<WireSyncReturn Function(wire_RwLockAabb)>();

  WireSyncReturn wire_aabb_center(
    wire_RwLockAabb aabb,
  ) {
    return _wire_aabb_center(
      aabb,
    );
  }

  late final _wire_aabb_centerPtr =
      _lookup<ffi.NativeFunction<WireSyncReturn Function(wire_RwLockAabb)>>(
          'wire_aabb_center');
  late final _wire_aabb_center = _wire_aabb_centerPtr
      .asFunction<WireSyncReturn Function(wire_RwLockAabb)>();

  WireSyncReturn wire_aabb_intersects(
    wire_RwLockAabb aabb_left,
    wire_RwLockAabb aabb_right,
  ) {
    return _wire_aabb_intersects(
      aabb_left,
      aabb_right,
    );
  }

  late final _wire_aabb_intersectsPtr = _lookup<
      ffi.NativeFunction<
          WireSyncReturn Function(
              wire_RwLockAabb, wire_RwLockAabb)>>('wire_aabb_intersects');
  late final _wire_aabb_intersects = _wire_aabb_intersectsPtr
      .asFunction<WireSyncReturn Function(wire_RwLockAabb, wire_RwLockAabb)>();

  WireSyncReturn wire_aabb_contains(
    wire_RwLockAabb aabb,
    ffi.Pointer<wire_float_64_list> point,
  ) {
    return _wire_aabb_contains(
      aabb,
      point,
    );
  }

  late final _wire_aabb_containsPtr = _lookup<
      ffi.NativeFunction<
          WireSyncReturn Function(wire_RwLockAabb,
              ffi.Pointer<wire_float_64_list>)>>('wire_aabb_contains');
  late final _wire_aabb_contains = _wire_aabb_containsPtr.asFunction<
      WireSyncReturn Function(
          wire_RwLockAabb, ffi.Pointer<wire_float_64_list>)>();

  WireSyncReturn wire_aabb_contains_aabb(
    wire_RwLockAabb aabb_left,
    wire_RwLockAabb aabb_right,
  ) {
    return _wire_aabb_contains_aabb(
      aabb_left,
      aabb_right,
    );
  }

  late final _wire_aabb_contains_aabbPtr = _lookup<
      ffi.NativeFunction<
          WireSyncReturn Function(
              wire_RwLockAabb, wire_RwLockAabb)>>('wire_aabb_contains_aabb');
  late final _wire_aabb_contains_aabb = _wire_aabb_contains_aabbPtr
      .asFunction<WireSyncReturn Function(wire_RwLockAabb, wire_RwLockAabb)>();

  WireSyncReturn wire_aabb_merge(
    wire_RwLockAabb aabb_left,
    wire_RwLockAabb aabb_right,
  ) {
    return _wire_aabb_merge(
      aabb_left,
      aabb_right,
    );
  }

  late final _wire_aabb_mergePtr = _lookup<
      ffi.NativeFunction<
          WireSyncReturn Function(
              wire_RwLockAabb, wire_RwLockAabb)>>('wire_aabb_merge');
  late final _wire_aabb_merge = _wire_aabb_mergePtr
      .asFunction<WireSyncReturn Function(wire_RwLockAabb, wire_RwLockAabb)>();

  WireSyncReturn wire_aabb_merge_with(
    wire_RwLockAabb aabb,
    wire_RwLockAabb other,
  ) {
    return _wire_aabb_merge_with(
      aabb,
      other,
    );
  }

  late final _wire_aabb_merge_withPtr = _lookup<
      ffi.NativeFunction<
          WireSyncReturn Function(
              wire_RwLockAabb, wire_RwLockAabb)>>('wire_aabb_merge_with');
  late final _wire_aabb_merge_with = _wire_aabb_merge_withPtr
      .asFunction<WireSyncReturn Function(wire_RwLockAabb, wire_RwLockAabb)>();

  WireSyncReturn wire_bvh_new(
    ffi.Pointer<wire_list_RwLockAabb> aabbs,
  ) {
    return _wire_bvh_new(
      aabbs,
    );
  }

  late final _wire_bvh_newPtr = _lookup<
      ffi.NativeFunction<
          WireSyncReturn Function(
              ffi.Pointer<wire_list_RwLockAabb>)>>('wire_bvh_new');
  late final _wire_bvh_new = _wire_bvh_newPtr
      .asFunction<WireSyncReturn Function(ffi.Pointer<wire_list_RwLockAabb>)>();

  void wire_bvh_new_async(
    int port_,
    ffi.Pointer<wire_list_RwLockAabb> aabbs,
  ) {
    return _wire_bvh_new_async(
      port_,
      aabbs,
    );
  }

  late final _wire_bvh_new_asyncPtr = _lookup<
      ffi.NativeFunction<
          ffi.Void Function(ffi.Int64,
              ffi.Pointer<wire_list_RwLockAabb>)>>('wire_bvh_new_async');
  late final _wire_bvh_new_async = _wire_bvh_new_asyncPtr
      .asFunction<void Function(int, ffi.Pointer<wire_list_RwLockAabb>)>();

  wire_RwLockAabb new_RwLockAabb() {
    return _new_RwLockAabb();
  }

  late final _new_RwLockAabbPtr =
      _lookup<ffi.NativeFunction<wire_RwLockAabb Function()>>('new_RwLockAabb');
  late final _new_RwLockAabb =
      _new_RwLockAabbPtr.asFunction<wire_RwLockAabb Function()>();

  ffi.Pointer<wire_float_64_list> new_float_64_list_0(
    int len,
  ) {
    return _new_float_64_list_0(
      len,
    );
  }

  late final _new_float_64_list_0Ptr = _lookup<
      ffi.NativeFunction<
          ffi.Pointer<wire_float_64_list> Function(
              ffi.Int32)>>('new_float_64_list_0');
  late final _new_float_64_list_0 = _new_float_64_list_0Ptr
      .asFunction<ffi.Pointer<wire_float_64_list> Function(int)>();

  ffi.Pointer<wire_list_RwLockAabb> new_list_RwLockAabb_0(
    int len,
  ) {
    return _new_list_RwLockAabb_0(
      len,
    );
  }

  late final _new_list_RwLockAabb_0Ptr = _lookup<
      ffi.NativeFunction<
          ffi.Pointer<wire_list_RwLockAabb> Function(
              ffi.Int32)>>('new_list_RwLockAabb_0');
  late final _new_list_RwLockAabb_0 = _new_list_RwLockAabb_0Ptr
      .asFunction<ffi.Pointer<wire_list_RwLockAabb> Function(int)>();

  void drop_opaque_RwLockAabb(
    ffi.Pointer<ffi.Void> ptr,
  ) {
    return _drop_opaque_RwLockAabb(
      ptr,
    );
  }

  late final _drop_opaque_RwLockAabbPtr =
      _lookup<ffi.NativeFunction<ffi.Void Function(ffi.Pointer<ffi.Void>)>>(
          'drop_opaque_RwLockAabb');
  late final _drop_opaque_RwLockAabb = _drop_opaque_RwLockAabbPtr
      .asFunction<void Function(ffi.Pointer<ffi.Void>)>();

  ffi.Pointer<ffi.Void> share_opaque_RwLockAabb(
    ffi.Pointer<ffi.Void> ptr,
  ) {
    return _share_opaque_RwLockAabb(
      ptr,
    );
  }

  late final _share_opaque_RwLockAabbPtr = _lookup<
      ffi.NativeFunction<
          ffi.Pointer<ffi.Void> Function(
              ffi.Pointer<ffi.Void>)>>('share_opaque_RwLockAabb');
  late final _share_opaque_RwLockAabb = _share_opaque_RwLockAabbPtr
      .asFunction<ffi.Pointer<ffi.Void> Function(ffi.Pointer<ffi.Void>)>();

  void drop_opaque_RwLockBvh(
    ffi.Pointer<ffi.Void> ptr,
  ) {
    return _drop_opaque_RwLockBvh(
      ptr,
    );
  }

  late final _drop_opaque_RwLockBvhPtr =
      _lookup<ffi.NativeFunction<ffi.Void Function(ffi.Pointer<ffi.Void>)>>(
          'drop_opaque_RwLockBvh');
  late final _drop_opaque_RwLockBvh = _drop_opaque_RwLockBvhPtr
      .asFunction<void Function(ffi.Pointer<ffi.Void>)>();

  ffi.Pointer<ffi.Void> share_opaque_RwLockBvh(
    ffi.Pointer<ffi.Void> ptr,
  ) {
    return _share_opaque_RwLockBvh(
      ptr,
    );
  }

  late final _share_opaque_RwLockBvhPtr = _lookup<
      ffi.NativeFunction<
          ffi.Pointer<ffi.Void> Function(
              ffi.Pointer<ffi.Void>)>>('share_opaque_RwLockBvh');
  late final _share_opaque_RwLockBvh = _share_opaque_RwLockBvhPtr
      .asFunction<ffi.Pointer<ffi.Void> Function(ffi.Pointer<ffi.Void>)>();

  void free_WireSyncReturn(
    WireSyncReturn ptr,
  ) {
    return _free_WireSyncReturn(
      ptr,
    );
  }

  late final _free_WireSyncReturnPtr =
      _lookup<ffi.NativeFunction<ffi.Void Function(WireSyncReturn)>>(
          'free_WireSyncReturn');
  late final _free_WireSyncReturn =
      _free_WireSyncReturnPtr.asFunction<void Function(WireSyncReturn)>();
}

class _Dart_Handle extends ffi.Opaque {}

class wire_float_64_list extends ffi.Struct {
  external ffi.Pointer<ffi.Double> ptr;

  @ffi.Int32()
  external int len;
}

class wire_RwLockAabb extends ffi.Struct {
  external ffi.Pointer<ffi.Void> ptr;
}

class wire_list_RwLockAabb extends ffi.Struct {
  external ffi.Pointer<wire_RwLockAabb> ptr;

  @ffi.Int32()
  external int len;
}

typedef DartPostCObjectFnType = ffi.Pointer<
    ffi.NativeFunction<ffi.Bool Function(DartPort, ffi.Pointer<ffi.Void>)>>;
typedef DartPort = ffi.Int64;
