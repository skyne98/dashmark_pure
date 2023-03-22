import 'dart:typed_data';
import 'dart:ui';

import 'package:dashmark_pure/matrix.dart';
import 'package:dashmark_pure/typed_array/f64.dart';
import 'package:dashmark_pure/world.dart';
import 'package:vector_math/vector_math_64.dart';

class Batch {
  static const int batchSize = 8192;

  static const vertexTopLeft = Offset(0.0, 0.0);
  static const vertexBottomLeft = Offset(0.0, 1.0);
  static const vertexBottomRight = Offset(1.0, 1.0);
  static const vertexTopRight = Offset(1.0, 0.0);

  int _currentId = 0;

  // Matrices
  final List<Affine> _transforms = [];
  final List<Affine> _globalTransforms = [];
  final Vector2Buffer _position = Vector2Buffer();
  final Float64Buffer _rotation = Float64Buffer();
  final Vector2Buffer _size = Vector2Buffer();
  Float32List _vertexCoordsCache = Float32List(0);
  Float32List _textureCoordsCache = Float32List(0);
  Uint16List _indicesCache = Uint16List(0);

  // Cache flags
  bool cachesNeedExpanding = false;
  int? populateTextureAndIndexCacheFrom;
  int? populateTextureAndIndexCacheTo;

  int get length => _transforms.length;

  void setPositionFrom(int id, double x, double y) {
    _position.setXY(id, x, y);
  }

  void setRotationFrom(int id, double rotation) {
    _rotation[id] = rotation;
  }

  int add(double x, double y, double width, double height) {
    final id = _currentId++;
    final matrix = Affine.identity();
    matrix.setTransform(
        x, y, 0, World.desiredSize, World.desiredSize, 0.5, 0.5);
    _transforms.add(matrix);
    _globalTransforms.add(Affine.identity());
    _position.addXY(x, y);
    _rotation.add(0.3);
    _size.addXY(width, height);
    return id;
  }

  void expandCaches({int? count}) {
    count ??= _transforms.length;
    final newVertexCoordsCache = Float32List(count * 8);
    final newTextureCoordsCache = Float32List(count * 8);
    final newIndicesCache = Uint16List(count * 6);
    final oldCount = _vertexCoordsCache.length ~/ 8;
    // Copy old data
    for (var i = 0; i < oldCount; i++) {
      final index = i * 8;
      newTextureCoordsCache[index] = _textureCoordsCache[index];
      newTextureCoordsCache[index + 1] = _textureCoordsCache[index + 1];
      newTextureCoordsCache[index + 2] = _textureCoordsCache[index + 2];
      newTextureCoordsCache[index + 3] = _textureCoordsCache[index + 3];
      newTextureCoordsCache[index + 4] = _textureCoordsCache[index + 4];
      newTextureCoordsCache[index + 5] = _textureCoordsCache[index + 5];
      newTextureCoordsCache[index + 6] = _textureCoordsCache[index + 6];
      newTextureCoordsCache[index + 7] = _textureCoordsCache[index + 7];

      final indexOffset = i * 6;
      final vertexOffset = i * 4;
      newIndicesCache[indexOffset + 0] = vertexOffset + 0;
      newIndicesCache[indexOffset + 1] = vertexOffset + 1;
      newIndicesCache[indexOffset + 2] = vertexOffset + 2;
      newIndicesCache[indexOffset + 3] = vertexOffset + 0;
      newIndicesCache[indexOffset + 4] = vertexOffset + 2;
      newIndicesCache[indexOffset + 5] = vertexOffset + 3;
    }
    _vertexCoordsCache = newVertexCoordsCache;
    _textureCoordsCache = newTextureCoordsCache;
    _indicesCache = newIndicesCache;
    cachesNeedExpanding = false;
  }

  void populateTextureAndIndexCache(int from, int to) {
    for (var i = from; i < to; ++i) {
      final sizeX = _size.getX(i);
      final sizeY = _size.getY(i);

      final index = i * 8;

      final index0 = index;
      final index1 = index + 2;
      final index2 = index + 4;
      final index3 = index + 6;

      // 0 - top left - 0, 0
      // 1 - top right - 1, 0
      // 2 - bottom right - 1, 1
      // 3 - bottom left - 0, 1

      _textureCoordsCache[index0] = 0.0; // top left x
      _textureCoordsCache[index0 + 1] = 0.0; // top left y
      _textureCoordsCache[index1] = sizeX; // top right x
      _textureCoordsCache[index1 + 1] = 0.0; // top right y
      _textureCoordsCache[index2] = sizeX; // bottom right x
      _textureCoordsCache[index2 + 1] = sizeY; // bottom right y
      _textureCoordsCache[index3] = 0.0; // bottom left x
      _textureCoordsCache[index3 + 1] = sizeY; // bottom left y

      final indexOffset = i * 6;
      final vertexOffset = i * 4;
      _indicesCache[indexOffset + 0] = vertexOffset + 0;
      _indicesCache[indexOffset + 1] = vertexOffset + 1;
      _indicesCache[indexOffset + 2] = vertexOffset + 2;
      _indicesCache[indexOffset + 3] = vertexOffset + 0;
      _indicesCache[indexOffset + 4] = vertexOffset + 2;
      _indicesCache[indexOffset + 5] = vertexOffset + 3;
    }
  }

  void draw(Canvas canvas, Paint paint) {
    // Draw the sprites
    final parentMatrix0 = Affine.identity();
    parentMatrix0.setRotation(45 * degrees2Radians);
    final parentMatrix1 = Affine.identity();
    parentMatrix1.setRotation(-45 * degrees2Radians);

    // Check if expansion is needed
    if (cachesNeedExpanding) {
      expandCaches();
    }
    if (populateTextureAndIndexCacheFrom != null ||
        populateTextureAndIndexCacheTo != null) {
      populateTextureAndIndexCacheFrom ??= 0;
      populateTextureAndIndexCacheTo ??= _transforms.length;
      populateTextureAndIndexCache(
          populateTextureAndIndexCacheFrom!, populateTextureAndIndexCacheTo!);
      populateTextureAndIndexCacheFrom = null;
      populateTextureAndIndexCacheTo = null;
    }

    final length = _transforms.length;
    for (var i = 0; i < length; ++i) {
      // Update the matrix
      final transform = _transforms[i];
      final rotation = _rotation[i];

      final multipliedTransform = _globalTransforms[i];
      final positionX = _position.getX(i);
      final positionY = _position.getY(i);
      transform.setTransform(positionX, positionY, rotation, World.desiredSize,
          World.desiredSize, 0.5, 0.5);

      final index = i * 8;
      final index0 = index;
      final index1 = index + 2;
      final index2 = index + 4;
      final index3 = index + 6;

      multipliedTransform.setIdentity();
      // multipliedTransform.multiplyBy(parentMatrix0);
      // multipliedTransform.multiplyBy(parentMatrix1);
      multipliedTransform.multiplyBy(transform);

      // Calculating the transform
      multipliedTransform.transformRawFrom(
          _vertexCoordsCache, index0, vertexTopLeft.dx, vertexTopLeft.dy);
      multipliedTransform.transformRawFrom(
          _vertexCoordsCache, index1, vertexTopRight.dx, vertexTopRight.dy);
      multipliedTransform.transformRawFrom(_vertexCoordsCache, index2,
          vertexBottomRight.dx, vertexBottomRight.dy);
      multipliedTransform.transformRawFrom(
          _vertexCoordsCache, index3, vertexBottomLeft.dx, vertexBottomLeft.dy);
    }

    final vertices = Vertices.raw(VertexMode.triangles, _vertexCoordsCache,
        textureCoordinates: _textureCoordsCache, indices: _indicesCache);

    // Draw the sprite
    canvas.drawVertices(vertices, BlendMode.srcOver, paint);
    vertices.dispose();
  }
}
