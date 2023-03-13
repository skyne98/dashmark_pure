import 'dart:typed_data';
import 'dart:ui';

import 'package:dashmark_pure/world.dart';
import 'package:vector_math/vector_math.dart';

class Batch {
  static const int batchSize = 16384;

  int _currentId = 0;

  // Matrices
  final List<Matrix4> _transforms = [];
  final List<Vector2> _position = [];
  final List<Vector2> _size = [];
  Float32List _vertexCoordsCache = Float32List(0);
  Float32List _textureCoordsCache = Float32List(0);
  Uint16List _indicesCache = Uint16List(0);

  // Cache flags
  bool cachesNeedExpanding = false;
  int? populateTextureAndIndexCacheFrom;
  int? populateTextureAndIndexCacheTo;

  int get length => _transforms.length;

  Vector2 getPosition(int id) => _position[id];
  void setPositionFrom(int id, Vector2 position) {
    _position[id] = position;
  }

  int add(double x, double y, double width, double height) {
    final id = _currentId++;
    final matrix = Matrix4.identity();
    matrix.translate(x, y);
    matrix.scale(World.desiredSize, World.desiredSize);
    _transforms.add(matrix);
    _position.add(Vector2(x, y));
    _size.add(Vector2(width, height));
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
      final size = _size[i];
      final sizeX = size.x;
      final sizeY = size.y;

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

  void transformVertsInCache(int index, Matrix4 matrix) {
    final matrixStorage = matrix.storage;
    final indexX = index;
    final indexY = index + 1;

    final x = _vertexCoordsCache[indexX];
    final y = _vertexCoordsCache[indexY];

    _vertexCoordsCache[indexX] =
        (x * matrixStorage[0]) + (y * matrixStorage[4]) + matrixStorage[12];
    _vertexCoordsCache[indexY] =
        (x * matrixStorage[1]) + (y * matrixStorage[5]) + matrixStorage[13];
  }

  void setVertInCache(int index, double x, double y) {
    _vertexCoordsCache[index] = x;
    _vertexCoordsCache[index + 1] = y;
  }

  void transformVertsInCacheFrom(
      int index, Matrix4 matrix, double x, double y) {
    final matrixStorage = matrix.storage;
    final indexX = index;
    final indexY = index + 1;

    _vertexCoordsCache[indexX] =
        (x * matrixStorage[0]) + (y * matrixStorage[4]) + matrixStorage[12];
    _vertexCoordsCache[indexY] =
        (x * matrixStorage[1]) + (y * matrixStorage[5]) + matrixStorage[13];
  }

  void draw(Canvas canvas, Paint paint) {
    // Draw the sprites
    final vertexTopLeft = Vector2(0.0, 0.0);
    final vertexBottomLeft = Vector2(0.0, 1.0);
    final vertexBottomRight = Vector2(1.0, 1.0);
    final vertexTopRight = Vector2(1.0, 0.0);

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
      transform.setTranslationRaw(_position[i].x, _position[i].y, 0.0);

      final index = i * 8;
      final index0 = index;
      final index1 = index + 2;
      final index2 = index + 4;
      final index3 = index + 6;

      // Calculating the transform
      transformVertsInCacheFrom(
          index0, transform, vertexTopLeft.x, vertexTopLeft.y);
      transformVertsInCacheFrom(
          index1, transform, vertexTopRight.x, vertexTopRight.y);
      transformVertsInCacheFrom(
          index2, transform, vertexBottomRight.x, vertexBottomRight.y);
      transformVertsInCacheFrom(
          index3, transform, vertexBottomLeft.x, vertexBottomLeft.y);
    }

    final vertices = Vertices.raw(VertexMode.triangles, _vertexCoordsCache,
        textureCoordinates: _textureCoordsCache, indices: _indicesCache);

    // Draw the sprite
    canvas.drawVertices(vertices, BlendMode.srcOver, paint);
  }
}
