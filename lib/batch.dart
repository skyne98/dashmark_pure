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

  int get length => _transforms.length;

  Vector2 getPosition(int id) => _position[id];
  void setPositionFrom(int id, Vector2 position) {
    _position[id] = position;
  }

  void add(double x, double y, double width, double height) {
    final id = _currentId++;
    final matrix = Matrix4.identity();
    matrix.translate(x, y);
    matrix.scale(World.desiredSize, World.desiredSize);
    _transforms.add(matrix);
    _position.add(Vector2(x, y));
    _size.add(Vector2(width, height));
  }

  void resizeBuffers() {
    final count = _transforms.length;
    final currentSize = _vertexCoordsCache.length ~/ 12;
    if (count > currentSize) {
      expandCaches(count);
      populateTextureAndIndexCache(currentSize, count);
    }
  }

  void expandCaches(int count) {
    final newVertexCoordsCache = Float32List(count * 12);
    final newTextureCoordsCache = Float32List(count * 12);
    final oldCount = _vertexCoordsCache.length ~/ 12;
    // Copy old data
    for (var i = 0; i < oldCount; i++) {
      final index = i * 12;
      newTextureCoordsCache[index] = _textureCoordsCache[index];
      newTextureCoordsCache[index + 1] = _textureCoordsCache[index + 1];
      newTextureCoordsCache[index + 2] = _textureCoordsCache[index + 2];
      newTextureCoordsCache[index + 3] = _textureCoordsCache[index + 3];
      newTextureCoordsCache[index + 4] = _textureCoordsCache[index + 4];
      newTextureCoordsCache[index + 5] = _textureCoordsCache[index + 5];
      newTextureCoordsCache[index + 6] = _textureCoordsCache[index + 6];
      newTextureCoordsCache[index + 7] = _textureCoordsCache[index + 7];
      newTextureCoordsCache[index + 8] = _textureCoordsCache[index + 8];
      newTextureCoordsCache[index + 9] = _textureCoordsCache[index + 9];
      newTextureCoordsCache[index + 10] = _textureCoordsCache[index + 10];
      newTextureCoordsCache[index + 11] = _textureCoordsCache[index + 11];
    }
    _vertexCoordsCache = newVertexCoordsCache;
    _textureCoordsCache = newTextureCoordsCache;
  }

  void populateTextureAndIndexCache(int from, int to) {
    for (var i = from; i < to; ++i) {
      final size = _size[i];
      final sizeX = size.x;
      final sizeY = size.y;

      final index = i * 12;

      final index0 = index;
      final index1 = index + 2;
      final index2 = index + 4;
      final index3 = index + 6;
      final index4 = index + 8;
      final index5 = index + 10;

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
      _textureCoordsCache[index3] = 0.0; // top left x
      _textureCoordsCache[index3 + 1] = 0.0; // top left y
      _textureCoordsCache[index4] = sizeX; // bottom right x
      _textureCoordsCache[index4 + 1] = sizeY; // bottom right y
      _textureCoordsCache[index5] = 0.0; // bottom left x
      _textureCoordsCache[index5 + 1] = sizeY; // bottom left y
    }
  }

  void transformVertsInCache(int index, Matrix4 matrix) {
    final indexX = index;
    final indexY = index + 1;

    final x = _vertexCoordsCache[indexX];
    final y = _vertexCoordsCache[indexY];

    _vertexCoordsCache[indexX] = (x * matrix[0]) + (y * matrix[4]) + matrix[12];
    _vertexCoordsCache[indexY] = (x * matrix[1]) + (y * matrix[5]) + matrix[13];
  }

  void setVertInCache(int index, double x, double y) {
    _vertexCoordsCache[index] = x;
    _vertexCoordsCache[index + 1] = y;
  }

  void draw(Canvas canvas, Paint paint) {
    // Draw the sprites
    final vertexTopLeft = Vector2(0.0, 0.0);
    final vertexBottomLeft = Vector2(0.0, 1.0);
    final vertexBottomRight = Vector2(1.0, 1.0);
    final vertexTopRight = Vector2(1.0, 0.0);

    final length = _transforms.length;
    for (var i = 0; i < length; ++i) {
      // Update the matrix
      final transform = _transforms[i];
      transform.setTranslationRaw(_position[i].x, _position[i].y, 0.0);

      final index = i * 12;

      final index0 = index;
      final index1 = index + 2;
      final index2 = index + 4;
      final index3 = index + 6;
      final index4 = index + 8;
      final index5 = index + 10;

      setVertInCache(index0, vertexTopLeft.x, vertexTopLeft.y);
      setVertInCache(index1, vertexTopRight.x, vertexTopRight.y);
      setVertInCache(index2, vertexBottomRight.x, vertexBottomRight.y);
      setVertInCache(index3, vertexTopLeft.x, vertexTopLeft.y);
      setVertInCache(index4, vertexBottomRight.x, vertexBottomRight.y);
      setVertInCache(index5, vertexBottomLeft.x, vertexBottomLeft.y);

      transformVertsInCache(index0, transform);
      transformVertsInCache(index1, transform);
      transformVertsInCache(index2, transform);
      transformVertsInCache(index3, transform);
      transformVertsInCache(index4, transform);
      transformVertsInCache(index5, transform);
    }

    final vertices = Vertices.raw(VertexMode.triangles, _vertexCoordsCache,
        textureCoordinates: _textureCoordsCache);

    // Draw the sprite
    canvas.drawVertices(vertices, BlendMode.srcOver, paint);
  }
}
