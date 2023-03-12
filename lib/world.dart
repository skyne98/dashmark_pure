// import 'dart:html';
import 'dart:math';
import 'dart:ui';
import 'package:flutter/foundation.dart';
import 'package:flutter/material.dart' show Colors;
import 'package:flutter/painting.dart';
import 'package:flutter/services.dart' show rootBundle;
import 'package:vector_math/vector_math_64.dart' show Matrix4, Vector2;

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
  Float32List _transforms = Float32List(0);
  Float32List _rects = Float32List(0);
  final List<Vector2> _position = [];
  final List<Vector2> _velocity = [];

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
    final index0 = id * 4;
    final index1 = index0 + 1;
    final index2 = index0 + 2;
    final index3 = index0 + 3;

    // Add a new transform by copying the existing ones
    final anchorX = dashImage!.width / 2;
    final anchorY = dashImage!.height / 2;
    final double scos = cos(0.0) * scaleToSize;
    final double ssin = sin(0.0) * scaleToSize;
    final double tx = x + -scos * anchorX + ssin * anchorY;
    final double ty = y + -ssin * anchorX - scos * anchorY;
    _transforms[index0] = scos;
    _transforms[index1] = ssin;
    _transforms[index2] = tx;
    _transforms[index3] = ty;

    // Add a new rect by copying the existing ones
    _rects[index0] = 0.0;
    _rects[index1] = 0.0;
    _rects[index2] = dashImage!.width.toDouble();
    _rects[index3] = dashImage!.height.toDouble();

    _position.add(Vector2(x, y));
    _velocity.add(Vector2(vx, vy));
  }

  void input(double x, double y) {
    _spawnPosition = Vector2(x, y);
    if (dashImage != null && fragmentShader != null) {
      const amountPerSecond = 10000;
      final amount = (amountPerSecond * lastDt).toInt();

      // Create the buffers of new size
      final length = _velocity.length;
      final newTransforms = Float32List((length + amount) * 4);
      newTransforms.setAll(0, _transforms);
      _transforms = newTransforms;
      final newRects = Float32List((length + amount) * 4);
      newRects.setAll(0, _rects);
      _rects = newRects;

      for (var i = 0; i < amount; i++) {
        // Create a dash at 0,0 every frame
        final vx = 4 * cos(i * 2 * pi / amount);
        final vy = 4 * sin(i * 2 * pi / amount);
        addDash(_spawnPosition.x, _spawnPosition.y, vx, vy);
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

        final index0 = i * 4;
        final index1 = index0 + 1;
        final index2 = index0 + 2;
        final index3 = index0 + 3;

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
        final anchorX = dashImage!.width / 2;
        final anchorY = dashImage!.height / 2;
        final double scos = _transforms[index0];
        final double ssin = _transforms[index1];
        final double tx = position.x + -scos * anchorX + ssin * anchorY;
        final double ty = position.y + -ssin * anchorX - scos * anchorY;
        _transforms[index0] = scos;
        _transforms[index1] = ssin;
        _transforms[index2] = tx;
        _transforms[index3] = ty;
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

  void render(double t, Canvas canvas) {
    if (dashImage != null && fragmentShader != null) {
      canvas.drawColor(const Color(0xFF000000), BlendMode.srcOver);
      // Draw the dashes using Cavnas.drawAtlas
      const color = Color(0xFFFFFFFF);
      canvas.drawRawAtlas(dashImage!, _transforms, _rects, null, null, null,
          Paint()..color = color);

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

      // Draw a vertex based sprite
      Matrix4 transform = Matrix4.identity();
      transform.scale(100.0, 100.0);
      final vertexTopLeft = Vector2(0.0, 0.0);
      final vertexBottomLeft = Vector2(0.0, 1.0);
      final vertexBottomRight = Vector2(1.0, 1.0);
      final vertexTopRight = Vector2(1.0, 0.0);

      // Transform them
      final transformedTopLeft = transform2(vertexTopLeft, transform);
      final transformedTopRight = transform2(vertexTopRight, transform);
      final transformedBottomRight = transform2(vertexBottomRight, transform);
      final transformedBottomLeft = transform2(vertexBottomLeft, transform);

      final vertices = Vertices(VertexMode.triangleStrip, [
        toOffset(transformedTopLeft),
        toOffset(transformedTopRight),
        toOffset(transformedBottomLeft),
        toOffset(transformedBottomRight),
      ], textureCoordinates: [
        const Offset(0, 0),
        Offset(dashImage!.width.toDouble(), 0),
        Offset(0, dashImage!.height.toDouble()),
        Offset(dashImage!.width.toDouble(), dashImage!.height.toDouble()),
      ]);

      // Prepare the shader
      fragmentShader!.setImageSampler(0, dashImage!);
      fragmentShader!.setFloat(0, dashImage!.width.toDouble());
      fragmentShader!.setFloat(1, dashImage!.height.toDouble());
      final paint = Paint();
      paint.shader = fragmentShader;

      // Draw the sprite
      canvas.drawVertices(vertices, BlendMode.srcOver, paint);
      // canvas.drawPaint(paint);
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
