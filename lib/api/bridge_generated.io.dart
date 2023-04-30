// AUTO GENERATED FILE, DO NOT EDIT.
// Generated by `flutter_rust_bridge`@ 1.72.1.
// ignore_for_file: non_constant_identifier_names, unused_element, duplicate_ignore, directives_ordering, curly_braces_in_flow_control_structures, unnecessary_lambdas, slash_for_doc_comments, prefer_const_literals_to_create_immutables, implicit_dynamic_list_literal, duplicate_import, unused_import, unnecessary_import, prefer_single_quotes, prefer_const_constructors, use_super_parameters, always_use_package_imports, annotate_overrides, invalid_use_of_protected_member, constant_identifier_names, invalid_use_of_internal_member, prefer_is_empty, unnecessary_const

import 'dart:convert';
import 'dart:async';
import 'package:meta/meta.dart';
import 'package:flutter_rust_bridge/flutter_rust_bridge.dart';
import 'package:uuid/uuid.dart';
import 'bridge_generated.dart';
export 'bridge_generated.dart';
import 'dart:ffi' as ffi;

class NativePlatform extends FlutterRustBridgeBase<NativeWire> {
  NativePlatform(ffi.DynamicLibrary dylib) : super(NativeWire(dylib));

// Section: api2wire

  @protected
  ffi.Pointer<wire_uint_8_list> api2wire_ZeroCopyBuffer_Uint8List(
      Uint8List raw) {
    return api2wire_uint_8_list(raw);
  }

  @protected
  ffi.Pointer<wire_GenerationalIndex> api2wire_box_autoadd_generational_index(
      GenerationalIndex raw) {
    final ptr = inner.new_box_autoadd_generational_index_0();
    _api_fill_to_wire_generational_index(raw, ptr.ref);
    return ptr;
  }

  @protected
  ffi.Pointer<wire_Shape> api2wire_box_autoadd_shape(Shape raw) {
    final ptr = inner.new_box_autoadd_shape_0();
    _api_fill_to_wire_shape(raw, ptr.ref);
    return ptr;
  }

  @protected
  ffi.Pointer<wire_list_shape> api2wire_list_shape(List<Shape> raw) {
    final ans = inner.new_list_shape_0(raw.length);
    for (var i = 0; i < raw.length; ++i) {
      _api_fill_to_wire_shape(raw[i], ans.ref.ptr[i]);
    }
    return ans;
  }

  @protected
  ffi.Pointer<wire_list_shape_transform> api2wire_list_shape_transform(
      List<ShapeTransform> raw) {
    final ans = inner.new_list_shape_transform_0(raw.length);
    for (var i = 0; i < raw.length; ++i) {
      _api_fill_to_wire_shape_transform(raw[i], ans.ref.ptr[i]);
    }
    return ans;
  }

  @protected
  int api2wire_u64(int raw) {
    return raw;
  }

  @protected
  ffi.Pointer<wire_uint_8_list> api2wire_uint_8_list(Uint8List raw) {
    final ans = inner.new_uint_8_list_0(raw.length);
    ans.ref.ptr.asTypedList(raw.length).setAll(0, raw);
    return ans;
  }

// Section: finalizer

// Section: api_fill_to_wire

  void _api_fill_to_wire_box_autoadd_generational_index(
      GenerationalIndex apiObj, ffi.Pointer<wire_GenerationalIndex> wireObj) {
    _api_fill_to_wire_generational_index(apiObj, wireObj.ref);
  }

  void _api_fill_to_wire_box_autoadd_shape(
      Shape apiObj, ffi.Pointer<wire_Shape> wireObj) {
    _api_fill_to_wire_shape(apiObj, wireObj.ref);
  }

  void _api_fill_to_wire_generational_index(
      GenerationalIndex apiObj, wire_GenerationalIndex wireObj) {
    wireObj.field0 = api2wire_usize(apiObj.field0);
    wireObj.field1 = api2wire_u64(apiObj.field1);
  }

  void _api_fill_to_wire_shape(Shape apiObj, wire_Shape wireObj) {
    if (apiObj is Shape_Ball) {
      var pre_radius = api2wire_f64(apiObj.radius);
      wireObj.tag = 0;
      wireObj.kind = inner.inflate_Shape_Ball();
      wireObj.kind.ref.Ball.ref.radius = pre_radius;
      return;
    }
    if (apiObj is Shape_Compound) {
      var pre_children = api2wire_list_shape(apiObj.children);
      var pre_transforms = api2wire_list_shape_transform(apiObj.transforms);
      wireObj.tag = 1;
      wireObj.kind = inner.inflate_Shape_Compound();
      wireObj.kind.ref.Compound.ref.children = pre_children;
      wireObj.kind.ref.Compound.ref.transforms = pre_transforms;
      return;
    }
  }

  void _api_fill_to_wire_shape_transform(
      ShapeTransform apiObj, wire_ShapeTransform wireObj) {
    wireObj.position_x = api2wire_f64(apiObj.positionX);
    wireObj.position_y = api2wire_f64(apiObj.positionY);
    wireObj.rotation = api2wire_f64(apiObj.rotation);
    wireObj.absolute_origin_x = api2wire_f64(apiObj.absoluteOriginX);
    wireObj.absolute_origin_y = api2wire_f64(apiObj.absoluteOriginY);
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

  void wire_say_hello(
    int port_,
  ) {
    return _wire_say_hello(
      port_,
    );
  }

  late final _wire_say_helloPtr =
      _lookup<ffi.NativeFunction<ffi.Void Function(ffi.Int64)>>(
          'wire_say_hello');
  late final _wire_say_hello =
      _wire_say_helloPtr.asFunction<void Function(int)>();

  WireSyncReturn wire_update(
    double dt,
  ) {
    return _wire_update(
      dt,
    );
  }

  late final _wire_updatePtr =
      _lookup<ffi.NativeFunction<WireSyncReturn Function(ffi.Double)>>(
          'wire_update');
  late final _wire_update =
      _wire_updatePtr.asFunction<WireSyncReturn Function(double)>();

  WireSyncReturn wire_create_entity() {
    return _wire_create_entity();
  }

  late final _wire_create_entityPtr =
      _lookup<ffi.NativeFunction<WireSyncReturn Function()>>(
          'wire_create_entity');
  late final _wire_create_entity =
      _wire_create_entityPtr.asFunction<WireSyncReturn Function()>();

  WireSyncReturn wire_drop_entity(
    ffi.Pointer<wire_GenerationalIndex> index,
  ) {
    return _wire_drop_entity(
      index,
    );
  }

  late final _wire_drop_entityPtr = _lookup<
      ffi.NativeFunction<
          WireSyncReturn Function(
              ffi.Pointer<wire_GenerationalIndex>)>>('wire_drop_entity');
  late final _wire_drop_entity = _wire_drop_entityPtr.asFunction<
      WireSyncReturn Function(ffi.Pointer<wire_GenerationalIndex>)>();

  WireSyncReturn wire_entities_set_position_raw(
    ffi.Pointer<wire_uint_8_list> indices,
    ffi.Pointer<wire_uint_8_list> positions,
  ) {
    return _wire_entities_set_position_raw(
      indices,
      positions,
    );
  }

  late final _wire_entities_set_position_rawPtr = _lookup<
          ffi.NativeFunction<
              WireSyncReturn Function(ffi.Pointer<wire_uint_8_list>,
                  ffi.Pointer<wire_uint_8_list>)>>(
      'wire_entities_set_position_raw');
  late final _wire_entities_set_position_raw =
      _wire_entities_set_position_rawPtr.asFunction<
          WireSyncReturn Function(
              ffi.Pointer<wire_uint_8_list>, ffi.Pointer<wire_uint_8_list>)>();

  WireSyncReturn wire_entities_set_origin_raw(
    ffi.Pointer<wire_uint_8_list> indices,
    ffi.Pointer<wire_uint_8_list> origins,
  ) {
    return _wire_entities_set_origin_raw(
      indices,
      origins,
    );
  }

  late final _wire_entities_set_origin_rawPtr = _lookup<
      ffi.NativeFunction<
          WireSyncReturn Function(ffi.Pointer<wire_uint_8_list>,
              ffi.Pointer<wire_uint_8_list>)>>('wire_entities_set_origin_raw');
  late final _wire_entities_set_origin_raw =
      _wire_entities_set_origin_rawPtr.asFunction<
          WireSyncReturn Function(
              ffi.Pointer<wire_uint_8_list>, ffi.Pointer<wire_uint_8_list>)>();

  WireSyncReturn wire_entities_set_rotation_raw(
    ffi.Pointer<wire_uint_8_list> indices,
    ffi.Pointer<wire_uint_8_list> rotations,
  ) {
    return _wire_entities_set_rotation_raw(
      indices,
      rotations,
    );
  }

  late final _wire_entities_set_rotation_rawPtr = _lookup<
          ffi.NativeFunction<
              WireSyncReturn Function(ffi.Pointer<wire_uint_8_list>,
                  ffi.Pointer<wire_uint_8_list>)>>(
      'wire_entities_set_rotation_raw');
  late final _wire_entities_set_rotation_raw =
      _wire_entities_set_rotation_rawPtr.asFunction<
          WireSyncReturn Function(
              ffi.Pointer<wire_uint_8_list>, ffi.Pointer<wire_uint_8_list>)>();

  WireSyncReturn wire_entities_set_scale_raw(
    ffi.Pointer<wire_uint_8_list> indices,
    ffi.Pointer<wire_uint_8_list> scales,
  ) {
    return _wire_entities_set_scale_raw(
      indices,
      scales,
    );
  }

  late final _wire_entities_set_scale_rawPtr = _lookup<
      ffi.NativeFunction<
          WireSyncReturn Function(ffi.Pointer<wire_uint_8_list>,
              ffi.Pointer<wire_uint_8_list>)>>('wire_entities_set_scale_raw');
  late final _wire_entities_set_scale_raw =
      _wire_entities_set_scale_rawPtr.asFunction<
          WireSyncReturn Function(
              ffi.Pointer<wire_uint_8_list>, ffi.Pointer<wire_uint_8_list>)>();

  WireSyncReturn wire_query_aabb(
    double x,
    double y,
    double width,
    double height,
  ) {
    return _wire_query_aabb(
      x,
      y,
      width,
      height,
    );
  }

  late final _wire_query_aabbPtr = _lookup<
      ffi.NativeFunction<
          WireSyncReturn Function(ffi.Double, ffi.Double, ffi.Double,
              ffi.Double)>>('wire_query_aabb');
  late final _wire_query_aabb = _wire_query_aabbPtr
      .asFunction<WireSyncReturn Function(double, double, double, double)>();

  WireSyncReturn wire_query_aabb_raw(
    double x,
    double y,
    double width,
    double height,
  ) {
    return _wire_query_aabb_raw(
      x,
      y,
      width,
      height,
    );
  }

  late final _wire_query_aabb_rawPtr = _lookup<
      ffi.NativeFunction<
          WireSyncReturn Function(ffi.Double, ffi.Double, ffi.Double,
              ffi.Double)>>('wire_query_aabb_raw');
  late final _wire_query_aabb_raw = _wire_query_aabb_rawPtr
      .asFunction<WireSyncReturn Function(double, double, double, double)>();

  WireSyncReturn wire_entity_set_vertices_raw(
    ffi.Pointer<wire_GenerationalIndex> index,
    ffi.Pointer<wire_uint_8_list> vertices,
  ) {
    return _wire_entity_set_vertices_raw(
      index,
      vertices,
    );
  }

  late final _wire_entity_set_vertices_rawPtr = _lookup<
      ffi.NativeFunction<
          WireSyncReturn Function(ffi.Pointer<wire_GenerationalIndex>,
              ffi.Pointer<wire_uint_8_list>)>>('wire_entity_set_vertices_raw');
  late final _wire_entity_set_vertices_raw =
      _wire_entity_set_vertices_rawPtr.asFunction<
          WireSyncReturn Function(ffi.Pointer<wire_GenerationalIndex>,
              ffi.Pointer<wire_uint_8_list>)>();

  WireSyncReturn wire_entities_set_priority_raw(
    ffi.Pointer<wire_uint_8_list> indices,
    ffi.Pointer<wire_uint_8_list> priorities,
  ) {
    return _wire_entities_set_priority_raw(
      indices,
      priorities,
    );
  }

  late final _wire_entities_set_priority_rawPtr = _lookup<
          ffi.NativeFunction<
              WireSyncReturn Function(ffi.Pointer<wire_uint_8_list>,
                  ffi.Pointer<wire_uint_8_list>)>>(
      'wire_entities_set_priority_raw');
  late final _wire_entities_set_priority_raw =
      _wire_entities_set_priority_rawPtr.asFunction<
          WireSyncReturn Function(
              ffi.Pointer<wire_uint_8_list>, ffi.Pointer<wire_uint_8_list>)>();

  WireSyncReturn wire_entity_set_shape(
    ffi.Pointer<wire_GenerationalIndex> index,
    ffi.Pointer<wire_Shape> shape,
  ) {
    return _wire_entity_set_shape(
      index,
      shape,
    );
  }

  late final _wire_entity_set_shapePtr = _lookup<
      ffi.NativeFunction<
          WireSyncReturn Function(ffi.Pointer<wire_GenerationalIndex>,
              ffi.Pointer<wire_Shape>)>>('wire_entity_set_shape');
  late final _wire_entity_set_shape = _wire_entity_set_shapePtr.asFunction<
      WireSyncReturn Function(
          ffi.Pointer<wire_GenerationalIndex>, ffi.Pointer<wire_Shape>)>();

  WireSyncReturn wire_entity_set_color(
    ffi.Pointer<wire_GenerationalIndex> index,
    int color,
  ) {
    return _wire_entity_set_color(
      index,
      color,
    );
  }

  late final _wire_entity_set_colorPtr = _lookup<
      ffi.NativeFunction<
          WireSyncReturn Function(ffi.Pointer<wire_GenerationalIndex>,
              ffi.Int32)>>('wire_entity_set_color');
  late final _wire_entity_set_color = _wire_entity_set_colorPtr.asFunction<
      WireSyncReturn Function(ffi.Pointer<wire_GenerationalIndex>, int)>();

  WireSyncReturn wire_batches_count() {
    return _wire_batches_count();
  }

  late final _wire_batches_countPtr =
      _lookup<ffi.NativeFunction<WireSyncReturn Function()>>(
          'wire_batches_count');
  late final _wire_batches_count =
      _wire_batches_countPtr.asFunction<WireSyncReturn Function()>();

  WireSyncReturn wire_transformed_vertices(
    int batchIndex,
  ) {
    return _wire_transformed_vertices(
      batchIndex,
    );
  }

  late final _wire_transformed_verticesPtr =
      _lookup<ffi.NativeFunction<WireSyncReturn Function(ffi.Uint16)>>(
          'wire_transformed_vertices');
  late final _wire_transformed_vertices =
      _wire_transformed_verticesPtr.asFunction<WireSyncReturn Function(int)>();

  WireSyncReturn wire_tex_coords(
    int batchIndex,
  ) {
    return _wire_tex_coords(
      batchIndex,
    );
  }

  late final _wire_tex_coordsPtr =
      _lookup<ffi.NativeFunction<WireSyncReturn Function(ffi.Uint16)>>(
          'wire_tex_coords');
  late final _wire_tex_coords =
      _wire_tex_coordsPtr.asFunction<WireSyncReturn Function(int)>();

  WireSyncReturn wire_indices(
    int batchIndex,
  ) {
    return _wire_indices(
      batchIndex,
    );
  }

  late final _wire_indicesPtr =
      _lookup<ffi.NativeFunction<WireSyncReturn Function(ffi.Uint16)>>(
          'wire_indices');
  late final _wire_indices =
      _wire_indicesPtr.asFunction<WireSyncReturn Function(int)>();

  WireSyncReturn wire_colors(
    int batchIndex,
  ) {
    return _wire_colors(
      batchIndex,
    );
  }

  late final _wire_colorsPtr =
      _lookup<ffi.NativeFunction<WireSyncReturn Function(ffi.Uint16)>>(
          'wire_colors');
  late final _wire_colors =
      _wire_colorsPtr.asFunction<WireSyncReturn Function(int)>();

  ffi.Pointer<wire_GenerationalIndex> new_box_autoadd_generational_index_0() {
    return _new_box_autoadd_generational_index_0();
  }

  late final _new_box_autoadd_generational_index_0Ptr = _lookup<
          ffi.NativeFunction<ffi.Pointer<wire_GenerationalIndex> Function()>>(
      'new_box_autoadd_generational_index_0');
  late final _new_box_autoadd_generational_index_0 =
      _new_box_autoadd_generational_index_0Ptr
          .asFunction<ffi.Pointer<wire_GenerationalIndex> Function()>();

  ffi.Pointer<wire_Shape> new_box_autoadd_shape_0() {
    return _new_box_autoadd_shape_0();
  }

  late final _new_box_autoadd_shape_0Ptr =
      _lookup<ffi.NativeFunction<ffi.Pointer<wire_Shape> Function()>>(
          'new_box_autoadd_shape_0');
  late final _new_box_autoadd_shape_0 = _new_box_autoadd_shape_0Ptr
      .asFunction<ffi.Pointer<wire_Shape> Function()>();

  ffi.Pointer<wire_list_shape> new_list_shape_0(
    int len,
  ) {
    return _new_list_shape_0(
      len,
    );
  }

  late final _new_list_shape_0Ptr = _lookup<
          ffi.NativeFunction<ffi.Pointer<wire_list_shape> Function(ffi.Int32)>>(
      'new_list_shape_0');
  late final _new_list_shape_0 = _new_list_shape_0Ptr
      .asFunction<ffi.Pointer<wire_list_shape> Function(int)>();

  ffi.Pointer<wire_list_shape_transform> new_list_shape_transform_0(
    int len,
  ) {
    return _new_list_shape_transform_0(
      len,
    );
  }

  late final _new_list_shape_transform_0Ptr = _lookup<
      ffi.NativeFunction<
          ffi.Pointer<wire_list_shape_transform> Function(
              ffi.Int32)>>('new_list_shape_transform_0');
  late final _new_list_shape_transform_0 = _new_list_shape_transform_0Ptr
      .asFunction<ffi.Pointer<wire_list_shape_transform> Function(int)>();

  ffi.Pointer<wire_uint_8_list> new_uint_8_list_0(
    int len,
  ) {
    return _new_uint_8_list_0(
      len,
    );
  }

  late final _new_uint_8_list_0Ptr = _lookup<
      ffi.NativeFunction<
          ffi.Pointer<wire_uint_8_list> Function(
              ffi.Int32)>>('new_uint_8_list_0');
  late final _new_uint_8_list_0 = _new_uint_8_list_0Ptr
      .asFunction<ffi.Pointer<wire_uint_8_list> Function(int)>();

  ffi.Pointer<ShapeKind> inflate_Shape_Ball() {
    return _inflate_Shape_Ball();
  }

  late final _inflate_Shape_BallPtr =
      _lookup<ffi.NativeFunction<ffi.Pointer<ShapeKind> Function()>>(
          'inflate_Shape_Ball');
  late final _inflate_Shape_Ball =
      _inflate_Shape_BallPtr.asFunction<ffi.Pointer<ShapeKind> Function()>();

  ffi.Pointer<ShapeKind> inflate_Shape_Compound() {
    return _inflate_Shape_Compound();
  }

  late final _inflate_Shape_CompoundPtr =
      _lookup<ffi.NativeFunction<ffi.Pointer<ShapeKind> Function()>>(
          'inflate_Shape_Compound');
  late final _inflate_Shape_Compound = _inflate_Shape_CompoundPtr
      .asFunction<ffi.Pointer<ShapeKind> Function()>();

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

class wire_GenerationalIndex extends ffi.Struct {
  @ffi.UintPtr()
  external int field0;

  @ffi.Uint64()
  external int field1;
}

class wire_uint_8_list extends ffi.Struct {
  external ffi.Pointer<ffi.Uint8> ptr;

  @ffi.Int32()
  external int len;
}

class wire_Shape_Ball extends ffi.Struct {
  @ffi.Double()
  external double radius;
}

class wire_list_shape extends ffi.Struct {
  external ffi.Pointer<wire_Shape> ptr;

  @ffi.Int32()
  external int len;
}

class wire_Shape extends ffi.Struct {
  @ffi.Int32()
  external int tag;

  external ffi.Pointer<ShapeKind> kind;
}

class ShapeKind extends ffi.Union {
  external ffi.Pointer<wire_Shape_Ball> Ball;

  external ffi.Pointer<wire_Shape_Compound> Compound;
}

class wire_Shape_Compound extends ffi.Struct {
  external ffi.Pointer<wire_list_shape> children;

  external ffi.Pointer<wire_list_shape_transform> transforms;
}

class wire_list_shape_transform extends ffi.Struct {
  external ffi.Pointer<wire_ShapeTransform> ptr;

  @ffi.Int32()
  external int len;
}

class wire_ShapeTransform extends ffi.Struct {
  @ffi.Double()
  external double position_x;

  @ffi.Double()
  external double position_y;

  @ffi.Double()
  external double rotation;

  @ffi.Double()
  external double absolute_origin_x;

  @ffi.Double()
  external double absolute_origin_y;
}

typedef DartPostCObjectFnType = ffi.Pointer<
    ffi.NativeFunction<ffi.Bool Function(DartPort, ffi.Pointer<ffi.Void>)>>;
typedef DartPort = ffi.Int64;
