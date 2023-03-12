// import 'dart:html';
import 'dart:math';
import 'dart:typed_data';
import 'dart:ui';
import 'package:flutter/foundation.dart';
import 'package:flutter/material.dart' show Colors;
import 'package:flutter/painting.dart';
import 'package:flutter/services.dart' show rootBundle;
import 'package:vector_math/vector_math.dart' show Matrix4, Vector2;

class World {
  Vector2 size = Vector2(0.0, 0.0);
  double lastDt = 0.0;

  Image? dashImage;
  FragmentProgram? fragmentProgram;
  FragmentShader? fragmentShader;
  double desiredSize = 100.0;
  double scaleToSize = 0.0;

  Vector2 _spawnPosition = Vector2(0.0, 0.0);
  int _spawnedThisFrame = 0;

  int _currentId = 0;

  // Matrices
  final int _batchSize = 16384;
  final List<Matrix4> _transforms = [];
  final List<Vector2> _position = [];
  final List<Vector2> _velocity = [];
  List<Float32List> _vertexCoordsCache = [];
  List<Float32List> _textureCoordsCache = [];
  List<Uint16List> _indicesCache = [];

  // FPS
  final _lastFrameTimes = <double>[];

  // Status
  String status = 'Status';

  World() {
    debugPrint('World created');
    rootBundle.load('assets/images/dash.png').then((data) {
      final asUInt8List = Uint8List.view(data.buffer);
      decodeImageFromList(asUInt8List).then((result) {
        dashImage = result;
        scaleToSize = desiredSize / dashImage!.width;
        debugPrint('Loaded image ${dashImage!.width}x${dashImage!.height}');
        status = 'Loaded image ${dashImage!.width}x${dashImage!.height}';
      });
    }).catchError((error, stackTrace) {
      debugPrint('Error loading image: $error');
      status = 'Error loading image: $error';
    });
    FragmentProgram.fromAsset('shaders/sprite.glsl').then((result) {
      fragmentProgram = result;
      fragmentShader = fragmentProgram!.fragmentShader();
      debugPrint('Loaded fragment shader');
      status = 'Loaded fragment shader';
    }).catchError((error, stackTrace) {
      debugPrint('Error loading fragment shader: $error');
      status = 'Error loading fragment shader: $error';
    });
  }

  // Utils for batches
  int getBatchSize(int batchIndex) {
    // Last batch might be smaller
    if (batchIndex == _transforms.length ~/ _batchSize) {
      return _transforms.length % _batchSize;
    }
    return _batchSize;
  }

  void addDash(double x, double y, double vx, double vy) {
    final id = _currentId++;

    final matrix = Matrix4.identity();
    matrix.translate(x, y);
    matrix.scale(100.0, 100.0);
    _transforms.add(matrix);
    _position.add(Vector2(x, y));
    _velocity.add(Vector2(vx, vy));
  }

  void input(double x, double y) {
    _spawnPosition = Vector2(x, y);
    if (dashImage != null && fragmentShader != null) {
      const amountPerSecond = 10000;
      var amount = (amountPerSecond * lastDt).toInt();
      if (amount > amountPerSecond) {
        amount = amountPerSecond;
      }
      if (_spawnedThisFrame > amountPerSecond / 60) {
        return;
      }
      _spawnedThisFrame += amount;

      for (var i = 0; i < amount; i++) {
        // Create a dash at 0,0 every frame
        final vx = 4 * cos(i * 2 * pi / amount);
        final vy = 4 * sin(i * 2 * pi / amount);
        addDash(_spawnPosition.x, _spawnPosition.y, vx, vy);
      }

      // Find out which buffers have been affected
      final start = _transforms.length - amount;
      final end = _transforms.length;
      final startBatch = start ~/ _batchSize;
      final endBatch = end ~/ _batchSize;

      // Update batches one by one
      for (var batchIndex = startBatch; batchIndex <= endBatch; ++batchIndex) {
        final batchSize = getBatchSize(batchIndex);

        final newVertexCoords = Float32List(batchSize * 8);
        final newTextureCoords = Float32List(batchSize * 8);
        final newIndices = Uint16List(batchSize * 6);
        bool batchExists = batchIndex < _vertexCoordsCache.length;
        if (batchExists) {
          // Update the vertex and texture coords cache
          _vertexCoordsCache[batchIndex] = newVertexCoords;
          _textureCoordsCache[batchIndex] = newTextureCoords;
          _indicesCache[batchIndex] = newIndices;
        } else {
          // Create new vertex and texture coords cache
          _vertexCoordsCache.add(newVertexCoords);
          _textureCoordsCache.add(newTextureCoords);
          _indicesCache.add(newIndices);
        }

        // Update the texture coords cache & indices cache
        final dashWidth = dashImage!.width;
        final dashHeight = dashImage!.height;
        for (var i = 0; i < batchSize; ++i) {
          final offset = i * 8;
          final index0 = offset + 0;
          final index1 = offset + 1;
          final index2 = offset + 2;
          final index3 = offset + 3;
          final index4 = offset + 4;
          final index5 = offset + 5;
          final index6 = offset + 6;
          final index7 = offset + 7;

          newTextureCoords[index0] = 0.0; // top left x
          newTextureCoords[index1] = 0.0; // top left y
          newTextureCoords[index2] = dashWidth.toDouble(); // top right x
          newTextureCoords[index3] = 0.0; // top right y
          newTextureCoords[index4] = dashWidth.toDouble(); // bottom right x
          newTextureCoords[index5] = dashHeight.toDouble(); // bottom right y
          newTextureCoords[index6] = 0.0; // bottom left x
          newTextureCoords[index7] = dashHeight.toDouble(); // bottom left y

          final indexOffset = i * 6;
          newIndices[indexOffset + 0] = indexOffset + 0;
          newIndices[indexOffset + 1] = indexOffset + 1;
          newIndices[indexOffset + 2] = indexOffset + 2;
          newIndices[indexOffset + 3] = indexOffset + 0;
          newIndices[indexOffset + 4] = indexOffset + 2;
          newIndices[indexOffset + 5] = indexOffset + 3;
        }
      }
    }
  }

  void update(double t) {
    _spawnedThisFrame = 0;
    if (dashImage != null && fragmentShader != null) {
      // Jump around the dashes
      final length = _velocity.length;
      for (var i = 0; i < length; ++i) {
        final velocity = _velocity[i];
        final position = _position[i];

        // Move the dash
        position.x += velocity.x;
        position.y += velocity.y;

        // Bounce off walls
        if (position.x < 0) {
          position.x = 0;
          velocity.x = -velocity.x;
        } else if (position.x > size.x) {
          position.x = size.x;
          velocity.x = -velocity.x;
        }

        if (position.y < 0) {
          position.y = 0;
          velocity.y = -velocity.y;
        } else if (position.y > size.y) {
          position.y = size.y;
          velocity.y = -velocity.y;
        }

        // Add gravity
        velocity.y += 0.3;
        _transforms[i].setTranslationRaw(position.x, position.y, 0.0);
      }
      lastDt = t;

      // FPS
      _lastFrameTimes.add(t);
      // keep the list max length
      if (_lastFrameTimes.length > 10) {
        _lastFrameTimes.removeAt(0);
      }
      final fps = 1 /
          (_lastFrameTimes.fold(0.0, (a, b) => a + b) / _lastFrameTimes.length);
      final fpsRounded = fps.round();
      final title = 'Dashmark - $fpsRounded FPS - ${_velocity.length} entities';
      status = title;
    }
  }

  void transformVertsInCache(int batchIndex, int indexInBatch, Matrix4 matrix) {
    final indexX = indexInBatch;
    final indexY = indexInBatch + 1;

    final cache = _vertexCoordsCache[batchIndex];
    final x = cache[indexX];
    final y = cache[indexY];

    cache[indexX] = (x * matrix[0]) + (y * matrix[4]) + matrix[12];
    cache[indexY] = (x * matrix[1]) + (y * matrix[5]) + matrix[13];
  }

  void setVertInCache(int batchIndex, int indexInBatch, double x, double y) {
    final indexX = indexInBatch;
    final indexY = indexInBatch + 1;

    final cache = _vertexCoordsCache[batchIndex];
    cache[indexX] = x;
    cache[indexY] = y;
  }

  void render(double t, Canvas canvas) {
    if (dashImage != null && fragmentShader != null) {
      canvas.drawColor(const Color(0xFF000000), BlendMode.srcOver);

      // Prepare the shader
      fragmentShader!.setImageSampler(0, dashImage!);
      fragmentShader!.setFloat(0, dashImage!.width.toDouble());
      fragmentShader!.setFloat(1, dashImage!.height.toDouble());
      final paint = Paint();
      paint.shader = fragmentShader;

      // Draw the dashes
      final vertexTopLeft = Vector2(0.0, 0.0);
      final vertexBottomLeft = Vector2(0.0, 1.0);
      final vertexBottomRight = Vector2(1.0, 1.0);
      final vertexTopRight = Vector2(1.0, 0.0);

      final batches = _vertexCoordsCache.length;
      for (var batchIndex = 0; batchIndex < batches; ++batchIndex) {
        final length = _vertexCoordsCache[batchIndex].length ~/ 8;
        for (var i = 0; i < length; ++i) {
          final globalIndex = batchIndex * _batchSize + i;
          final transform = _transforms[globalIndex];
          final localIndex = i * 8;

          final index0 = localIndex;
          final index1 = localIndex + 2;
          final index2 = localIndex + 4;
          final index3 = localIndex + 6;

          setVertInCache(batchIndex, index0, vertexTopLeft.x, vertexTopLeft.y);
          setVertInCache(
              batchIndex, index1, vertexTopRight.x, vertexTopRight.y);
          setVertInCache(
              batchIndex, index2, vertexBottomRight.x, vertexBottomRight.y);
          setVertInCache(
              batchIndex, index3, vertexBottomLeft.x, vertexBottomLeft.y);

          transformVertsInCache(batchIndex, index0, transform);
          transformVertsInCache(batchIndex, index1, transform);
          transformVertsInCache(batchIndex, index2, transform);
          transformVertsInCache(batchIndex, index3, transform);
        }

        final vertices = Vertices.raw(
            VertexMode.triangles, _vertexCoordsCache[batchIndex],
            textureCoordinates: _textureCoordsCache[batchIndex],
            indices: _indicesCache[batchIndex]);

        // Draw the sprite
        canvas.drawVertices(vertices, BlendMode.srcOver, paint);
      }

      // Draw status in the middle
      final text = TextSpan(
        text: status,
        style: const TextStyle(
          color: Colors.red,
          fontSize: 15.0,
        ),
      );
      final textPainter = TextPainter(
        text: text,
        textAlign: TextAlign.center,
        textDirection: TextDirection.ltr,
      );
      textPainter.layout();
      textPainter.paint(
        canvas,
        Offset(
          (size.x - textPainter.width) / 2,
          (size.y - textPainter.height) / 2,
        ),
      );
    }
  }
}

Vector2 transform2(Vector2 position, Matrix4 matrix) {
  return Vector2(
    (position.x * matrix[0]) + (position.y * matrix[4]) + matrix[12],
    (position.x * matrix[1]) + (position.y * matrix[5]) + matrix[13],
  );
}

Offset toOffset(Vector2 vector) {
  return Offset(vector.x, vector.y);
}
