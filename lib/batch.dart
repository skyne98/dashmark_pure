import 'dart:typed_data';
import 'dart:ui';

import 'package:dashmark_pure/world.dart';
import 'package:vector_math/vector_math.dart';

class Batch {
  static const int batchSize = 16384;

  static const vertexTopLeft = Offset(0.0, 0.0);
  static const vertexBottomLeft = Offset(0.0, 1.0);
  static const vertexBottomRight = Offset(1.0, 1.0);
  static const vertexTopRight = Offset(1.0, 0.0);

  int _currentId = 0;

  // Matrices
  final List<Matrix4> _transforms = [];
  final List<Matrix4> _globalTransforms = [];
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
    _globalTransforms.add(Matrix4.identity());
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

  // Transform operations
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

  void transformTopDownVertsInCacheFrom(
      int index, List<Matrix4> matrices, double x, double y) {
    final indexX = index;
    final indexY = index + 1;

    _vertexCoordsCache[indexX] = x;
    _vertexCoordsCache[indexY] = y;

    for (final matrix in matrices) {
      final matrixStorage = matrix.storage;
      final x = _vertexCoordsCache[indexX];
      final y = _vertexCoordsCache[indexY];

      _vertexCoordsCache[indexX] =
          (x * matrixStorage[0]) + (y * matrixStorage[4]) + matrixStorage[12];
      _vertexCoordsCache[indexY] =
          (x * matrixStorage[1]) + (y * matrixStorage[5]) + matrixStorage[13];
    }
  }

  // Matrix operations
  void setToIdentity(Matrix4 matrix) {
    final storage = matrix.storage;
    storage[0] = 1.0;
    storage[1] = 0.0;
    storage[2] = 0.0;
    storage[3] = 0.0;
    storage[4] = 0.0;
    storage[5] = 1.0;
    storage[6] = 0.0;
    storage[7] = 0.0;
    storage[8] = 0.0;
    storage[9] = 0.0;
    storage[10] = 1.0;
    storage[11] = 0.0;
    storage[12] = 0.0;
    storage[13] = 0.0;
    storage[14] = 0.0;
    storage[15] = 1.0;
  }

  void multiplyBy(Matrix4 matrix, Matrix4 other) {
    final storage = matrix.storage;
    final otherStorage = other.storage;

    final m00 = storage[0];
    final m01 = storage[4];
    final m02 = storage[8];
    final m03 = storage[12];
    final m10 = storage[1];
    final m11 = storage[5];
    final m12 = storage[9];
    final m13 = storage[13];
    final m20 = storage[2];
    final m21 = storage[6];
    final m22 = storage[10];
    final m23 = storage[14];
    final m30 = storage[3];
    final m31 = storage[7];
    final m32 = storage[11];
    final m33 = storage[15];
    final n00 = otherStorage[0];
    final n01 = otherStorage[4];
    final n02 = otherStorage[8];
    final n03 = otherStorage[12];
    final n10 = otherStorage[1];
    final n11 = otherStorage[5];
    final n12 = otherStorage[9];
    final n13 = otherStorage[13];
    final n20 = otherStorage[2];
    final n21 = otherStorage[6];
    final n22 = otherStorage[10];
    final n23 = otherStorage[14];
    final n30 = otherStorage[3];
    final n31 = otherStorage[7];
    final n32 = otherStorage[11];
    final n33 = otherStorage[15];
    storage[0] = (m00 * n00) + (m01 * n10) + (m02 * n20) + (m03 * n30);
    storage[4] = (m00 * n01) + (m01 * n11) + (m02 * n21) + (m03 * n31);
    storage[8] = (m00 * n02) + (m01 * n12) + (m02 * n22) + (m03 * n32);
    storage[12] = (m00 * n03) + (m01 * n13) + (m02 * n23) + (m03 * n33);
    storage[1] = (m10 * n00) + (m11 * n10) + (m12 * n20) + (m13 * n30);
    storage[5] = (m10 * n01) + (m11 * n11) + (m12 * n21) + (m13 * n31);
    storage[9] = (m10 * n02) + (m11 * n12) + (m12 * n22) + (m13 * n32);
    storage[13] = (m10 * n03) + (m11 * n13) + (m12 * n23) + (m13 * n33);
    storage[2] = (m20 * n00) + (m21 * n10) + (m22 * n20) + (m23 * n30);
    storage[6] = (m20 * n01) + (m21 * n11) + (m22 * n21) + (m23 * n31);
    storage[10] = (m20 * n02) + (m21 * n12) + (m22 * n22) + (m23 * n32);
    storage[14] = (m20 * n03) + (m21 * n13) + (m22 * n23) + (m23 * n33);
    storage[3] = (m30 * n00) + (m31 * n10) + (m32 * n20) + (m33 * n30);
    storage[7] = (m30 * n01) + (m31 * n11) + (m32 * n21) + (m33 * n31);
    storage[11] = (m30 * n02) + (m31 * n12) + (m32 * n22) + (m33 * n32);
    storage[15] = (m30 * n03) + (m31 * n13) + (m32 * n23) + (m33 * n33);
  }

  void draw(Canvas canvas, Paint paint) {
    // Draw the sprites
    final parentMatrix0 = Matrix4.identity();
    final parentMatrix1 = Matrix4.identity();

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
      final multipliedTransform = _globalTransforms[i];
      transform.setTranslationRaw(_position[i].x, _position[i].y, 0.0);

      final index = i * 8;
      final index0 = index;
      final index1 = index + 2;
      final index2 = index + 4;
      final index3 = index + 6;

      setToIdentity(multipliedTransform);
      multiplyBy(multipliedTransform, parentMatrix0);
      multiplyBy(multipliedTransform, parentMatrix1);
      multiplyBy(multipliedTransform, transform);

      // Calculating the transform
      transformVertsInCacheFrom(
          index0, multipliedTransform, vertexTopLeft.dx, vertexTopLeft.dy);
      transformVertsInCacheFrom(
          index1, multipliedTransform, vertexTopRight.dx, vertexTopRight.dy);
      transformVertsInCacheFrom(index2, multipliedTransform,
          vertexBottomRight.dx, vertexBottomRight.dy);
      transformVertsInCacheFrom(index3, multipliedTransform,
          vertexBottomLeft.dx, vertexBottomLeft.dy);
    }

    final vertices = Vertices.raw(VertexMode.triangles, _vertexCoordsCache,
        textureCoordinates: _textureCoordsCache, indices: _indicesCache);

    // Draw the sprite
    canvas.drawVertices(vertices, BlendMode.srcOver, paint);
  }
}
