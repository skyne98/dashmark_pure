import 'dart:typed_data';
import 'dart:ui';

import 'package:dashmark_pure/matrix.dart';
import 'package:dashmark_pure/typed_array/f32.dart';
import 'package:dashmark_pure/typed_array/f64.dart';
import 'package:dashmark_pure/typed_array/u16.dart';
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
  Float32Buffer _vertexCoordsCache = Float32Buffer();
  Float32Buffer _textureCoordsCache = Float32Buffer();
  Uint16Buffer _indicesCache = Uint16Buffer();

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

    final index = _transforms.length;
    _transforms.add(matrix);
    _globalTransforms.add(Affine.identity());
    _position.addXY(x, y);
    _rotation.add(0.3);
    _size.addXY(width, height);

    // Fill the cache
    final vertexOffset = index * 4;

    // Texture cache
    final sizeX = _size.getX(id);
    final sizeY = _size.getY(id);
    _textureCoordsCache.add(0.0); // top left x
    _textureCoordsCache.add(0.0); // top left y
    _textureCoordsCache.add(sizeX); // top right x
    _textureCoordsCache.add(0.0); // top right y
    _textureCoordsCache.add(sizeX); // bottom right x
    _textureCoordsCache.add(sizeY); // bottom right y
    _textureCoordsCache.add(0.0); // bottom left x
    _textureCoordsCache.add(sizeY); // bottom left y

    // Vertex cache
    _vertexCoordsCache.add(0.0); // top left x
    _vertexCoordsCache.add(0.0); // top left y
    _vertexCoordsCache.add(0.0); // top right x
    _vertexCoordsCache.add(0.0); // top right y
    _vertexCoordsCache.add(0.0); // bottom right x
    _vertexCoordsCache.add(0.0); // bottom right y
    _vertexCoordsCache.add(0.0); // bottom left x
    _vertexCoordsCache.add(0.0); // bottom left y

    // Index cache
    _indicesCache.add(vertexOffset + 0);
    _indicesCache.add(vertexOffset + 1);
    _indicesCache.add(vertexOffset + 2);
    _indicesCache.add(vertexOffset + 0);
    _indicesCache.add(vertexOffset + 2);
    _indicesCache.add(vertexOffset + 3);

    return id;
  }

  void draw(Canvas canvas, Paint paint) {
    // Draw the sprites
    final parentMatrix0 = Affine.identity();
    parentMatrix0.setRotation(45 * degrees2Radians);
    final parentMatrix1 = Affine.identity();
    parentMatrix1.setRotation(-45 * degrees2Radians);

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
      multipliedTransform.multiplyBy(parentMatrix0);
      multipliedTransform.multiplyBy(parentMatrix1);
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

    final vertices = Vertices.raw(
        VertexMode.triangles, _vertexCoordsCache.toFloat32List(),
        textureCoordinates: _textureCoordsCache.toFloat32List(),
        indices: _indicesCache.toUint16List());

    // Draw the sprite
    canvas.drawVertices(vertices, BlendMode.srcOver, paint);
    vertices.dispose();
  }
}
