// coverage:ignore-file
// GENERATED CODE - DO NOT MODIFY BY HAND
// ignore_for_file: type=lint
// ignore_for_file: unused_element, deprecated_member_use, deprecated_member_use_from_same_package, use_function_type_syntax_for_parameters, unnecessary_const, avoid_init_to_null, invalid_override_different_default_values_named, prefer_expression_function_bodies, annotate_overrides, invalid_annotation_target, unnecessary_question_mark

part of 'bridge_generated.dart';

// **************************************************************************
// FreezedGenerator
// **************************************************************************

T _$identity<T>(T value) => value;

final _privateConstructorUsedError = UnsupportedError(
    'It seems like you constructed your class using `MyClass._()`. This constructor is only meant to be used by freezed and you are not supposed to need it nor use it.\nPlease check the documentation here for more information: https://github.com/rrousselGit/freezed#custom-getters-and-methods');

/// @nodoc
mixin _$Shape {
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function(double radius) ball,
    required TResult Function(
            List<Shape> children, List<ShapeTransform> transforms)
        compound,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function(double radius)? ball,
    TResult? Function(List<Shape> children, List<ShapeTransform> transforms)?
        compound,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function(double radius)? ball,
    TResult Function(List<Shape> children, List<ShapeTransform> transforms)?
        compound,
    required TResult orElse(),
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(Shape_Ball value) ball,
    required TResult Function(Shape_Compound value) compound,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(Shape_Ball value)? ball,
    TResult? Function(Shape_Compound value)? compound,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(Shape_Ball value)? ball,
    TResult Function(Shape_Compound value)? compound,
    required TResult orElse(),
  }) =>
      throw _privateConstructorUsedError;
}

/// @nodoc
abstract class $ShapeCopyWith<$Res> {
  factory $ShapeCopyWith(Shape value, $Res Function(Shape) then) =
      _$ShapeCopyWithImpl<$Res, Shape>;
}

/// @nodoc
class _$ShapeCopyWithImpl<$Res, $Val extends Shape>
    implements $ShapeCopyWith<$Res> {
  _$ShapeCopyWithImpl(this._value, this._then);

  // ignore: unused_field
  final $Val _value;
  // ignore: unused_field
  final $Res Function($Val) _then;
}

/// @nodoc
abstract class _$$Shape_BallCopyWith<$Res> {
  factory _$$Shape_BallCopyWith(
          _$Shape_Ball value, $Res Function(_$Shape_Ball) then) =
      __$$Shape_BallCopyWithImpl<$Res>;
  @useResult
  $Res call({double radius});
}

/// @nodoc
class __$$Shape_BallCopyWithImpl<$Res>
    extends _$ShapeCopyWithImpl<$Res, _$Shape_Ball>
    implements _$$Shape_BallCopyWith<$Res> {
  __$$Shape_BallCopyWithImpl(
      _$Shape_Ball _value, $Res Function(_$Shape_Ball) _then)
      : super(_value, _then);

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? radius = null,
  }) {
    return _then(_$Shape_Ball(
      radius: null == radius
          ? _value.radius
          : radius // ignore: cast_nullable_to_non_nullable
              as double,
    ));
  }
}

/// @nodoc

class _$Shape_Ball implements Shape_Ball {
  const _$Shape_Ball({required this.radius});

  @override
  final double radius;

  @override
  String toString() {
    return 'Shape.ball(radius: $radius)';
  }

  @override
  bool operator ==(dynamic other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$Shape_Ball &&
            (identical(other.radius, radius) || other.radius == radius));
  }

  @override
  int get hashCode => Object.hash(runtimeType, radius);

  @JsonKey(ignore: true)
  @override
  @pragma('vm:prefer-inline')
  _$$Shape_BallCopyWith<_$Shape_Ball> get copyWith =>
      __$$Shape_BallCopyWithImpl<_$Shape_Ball>(this, _$identity);

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function(double radius) ball,
    required TResult Function(
            List<Shape> children, List<ShapeTransform> transforms)
        compound,
  }) {
    return ball(radius);
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function(double radius)? ball,
    TResult? Function(List<Shape> children, List<ShapeTransform> transforms)?
        compound,
  }) {
    return ball?.call(radius);
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function(double radius)? ball,
    TResult Function(List<Shape> children, List<ShapeTransform> transforms)?
        compound,
    required TResult orElse(),
  }) {
    if (ball != null) {
      return ball(radius);
    }
    return orElse();
  }

  @override
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(Shape_Ball value) ball,
    required TResult Function(Shape_Compound value) compound,
  }) {
    return ball(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(Shape_Ball value)? ball,
    TResult? Function(Shape_Compound value)? compound,
  }) {
    return ball?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(Shape_Ball value)? ball,
    TResult Function(Shape_Compound value)? compound,
    required TResult orElse(),
  }) {
    if (ball != null) {
      return ball(this);
    }
    return orElse();
  }
}

abstract class Shape_Ball implements Shape {
  const factory Shape_Ball({required final double radius}) = _$Shape_Ball;

  double get radius;
  @JsonKey(ignore: true)
  _$$Shape_BallCopyWith<_$Shape_Ball> get copyWith =>
      throw _privateConstructorUsedError;
}

/// @nodoc
abstract class _$$Shape_CompoundCopyWith<$Res> {
  factory _$$Shape_CompoundCopyWith(
          _$Shape_Compound value, $Res Function(_$Shape_Compound) then) =
      __$$Shape_CompoundCopyWithImpl<$Res>;
  @useResult
  $Res call({List<Shape> children, List<ShapeTransform> transforms});
}

/// @nodoc
class __$$Shape_CompoundCopyWithImpl<$Res>
    extends _$ShapeCopyWithImpl<$Res, _$Shape_Compound>
    implements _$$Shape_CompoundCopyWith<$Res> {
  __$$Shape_CompoundCopyWithImpl(
      _$Shape_Compound _value, $Res Function(_$Shape_Compound) _then)
      : super(_value, _then);

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? children = null,
    Object? transforms = null,
  }) {
    return _then(_$Shape_Compound(
      children: null == children
          ? _value._children
          : children // ignore: cast_nullable_to_non_nullable
              as List<Shape>,
      transforms: null == transforms
          ? _value._transforms
          : transforms // ignore: cast_nullable_to_non_nullable
              as List<ShapeTransform>,
    ));
  }
}

/// @nodoc

class _$Shape_Compound implements Shape_Compound {
  const _$Shape_Compound(
      {required final List<Shape> children,
      required final List<ShapeTransform> transforms})
      : _children = children,
        _transforms = transforms;

  final List<Shape> _children;
  @override
  List<Shape> get children {
    if (_children is EqualUnmodifiableListView) return _children;
    // ignore: implicit_dynamic_type
    return EqualUnmodifiableListView(_children);
  }

  final List<ShapeTransform> _transforms;
  @override
  List<ShapeTransform> get transforms {
    if (_transforms is EqualUnmodifiableListView) return _transforms;
    // ignore: implicit_dynamic_type
    return EqualUnmodifiableListView(_transforms);
  }

  @override
  String toString() {
    return 'Shape.compound(children: $children, transforms: $transforms)';
  }

  @override
  bool operator ==(dynamic other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$Shape_Compound &&
            const DeepCollectionEquality().equals(other._children, _children) &&
            const DeepCollectionEquality()
                .equals(other._transforms, _transforms));
  }

  @override
  int get hashCode => Object.hash(
      runtimeType,
      const DeepCollectionEquality().hash(_children),
      const DeepCollectionEquality().hash(_transforms));

  @JsonKey(ignore: true)
  @override
  @pragma('vm:prefer-inline')
  _$$Shape_CompoundCopyWith<_$Shape_Compound> get copyWith =>
      __$$Shape_CompoundCopyWithImpl<_$Shape_Compound>(this, _$identity);

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function(double radius) ball,
    required TResult Function(
            List<Shape> children, List<ShapeTransform> transforms)
        compound,
  }) {
    return compound(children, transforms);
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function(double radius)? ball,
    TResult? Function(List<Shape> children, List<ShapeTransform> transforms)?
        compound,
  }) {
    return compound?.call(children, transforms);
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function(double radius)? ball,
    TResult Function(List<Shape> children, List<ShapeTransform> transforms)?
        compound,
    required TResult orElse(),
  }) {
    if (compound != null) {
      return compound(children, transforms);
    }
    return orElse();
  }

  @override
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(Shape_Ball value) ball,
    required TResult Function(Shape_Compound value) compound,
  }) {
    return compound(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(Shape_Ball value)? ball,
    TResult? Function(Shape_Compound value)? compound,
  }) {
    return compound?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(Shape_Ball value)? ball,
    TResult Function(Shape_Compound value)? compound,
    required TResult orElse(),
  }) {
    if (compound != null) {
      return compound(this);
    }
    return orElse();
  }
}

abstract class Shape_Compound implements Shape {
  const factory Shape_Compound(
      {required final List<Shape> children,
      required final List<ShapeTransform> transforms}) = _$Shape_Compound;

  List<Shape> get children;
  List<ShapeTransform> get transforms;
  @JsonKey(ignore: true)
  _$$Shape_CompoundCopyWith<_$Shape_Compound> get copyWith =>
      throw _privateConstructorUsedError;
}
