// import 'dart:html';
import 'dart:math';
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

  int _currentId = 0;

  // Matrices
  final List<Matrix4> _transforms = [];
  final List<Vector2> _position = [];
  final List<Vector2> _velocity = [];
  Float32List _vertexCoordsCache = Float32List(0);
  Float32List _textureCoordsCache = Float32List(0);

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
      final amount = (amountPerSecond * lastDt).toInt();

      for (var i = 0; i < amount; i++) {
        // Create a dash at 0,0 every frame
        final vx = 4 * cos(i * 2 * pi / amount);
        final vy = 4 * sin(i * 2 * pi / amount);
        addDash(_spawnPosition.x, _spawnPosition.y, vx, vy);
      }

      // Update the vertex and texture coords cache
      final length = _transforms.length;
      _vertexCoordsCache = Float32List(length * 12);
      _textureCoordsCache = Float32List(length * 12);

      // Update the texture coords cache
      final dashWidth = dashImage!.width;
      final dashHeight = dashImage!.height;
      for (var i = 0; i < length; ++i) {
        final offset = i * 12;
        _textureCoordsCache[offset + 0] = 0.0;
        _textureCoordsCache[offset + 1] = 0.0;
        _textureCoordsCache[offset + 2] = dashWidth.toDouble();
        _textureCoordsCache[offset + 3] = 0.0;
        _textureCoordsCache[offset + 4] = dashWidth.toDouble();
        _textureCoordsCache[offset + 5] = dashHeight.toDouble();
        _textureCoordsCache[offset + 6] = 0.0;
        _textureCoordsCache[offset + 7] = 0.0;
        _textureCoordsCache[offset + 8] = dashWidth.toDouble();
        _textureCoordsCache[offset + 9] = dashHeight.toDouble();
        _textureCoordsCache[offset + 10] = 0.0;
        _textureCoordsCache[offset + 11] = dashHeight.toDouble();
      }
    }
  }

  void update(double t) {
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

  void render(double t, Canvas canvas) {
    if (dashImage != null && fragmentShader != null) {
      canvas.drawColor(const Color(0xFF000000), BlendMode.srcOver);

      // Draw the dashes
      final vertexTopLeft = Vector2(0.0, 0.0);
      final vertexBottomLeft = Vector2(0.0, 1.0);
      final vertexBottomRight = Vector2(1.0, 1.0);
      final vertexTopRight = Vector2(1.0, 0.0);

      final length = _transforms.length;
      for (var i = 0; i < length; ++i) {
        final transform = _transforms[i];
        final index = i * 12;

        setVertInCache(index, vertexTopLeft.x, vertexTopLeft.y);
        setVertInCache(index + 2, vertexBottomLeft.x, vertexBottomLeft.y);
        setVertInCache(index + 4, vertexBottomRight.x, vertexBottomRight.y);
        setVertInCache(index + 6, vertexTopRight.x, vertexTopRight.y);
        setVertInCache(index + 8, vertexTopLeft.x, vertexTopLeft.y);
        setVertInCache(index + 10, vertexBottomRight.x, vertexBottomRight.y);

        transformVertsInCache(index, transform);
        transformVertsInCache(index + 2, transform);
        transformVertsInCache(index + 4, transform);
        transformVertsInCache(index + 6, transform);
        transformVertsInCache(index + 8, transform);
        transformVertsInCache(index + 10, transform);
      }

      // Prepare the shader
      fragmentShader!.setImageSampler(0, dashImage!);
      fragmentShader!.setFloat(0, dashImage!.width.toDouble());
      fragmentShader!.setFloat(1, dashImage!.height.toDouble());
      final paint = Paint();
      paint.shader = fragmentShader;

      final vertices = Vertices.raw(VertexMode.triangles, _vertexCoordsCache,
          textureCoordinates: _textureCoordsCache);

      // Draw the sprite
      canvas.drawVertices(vertices, BlendMode.srcOver, paint);

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
