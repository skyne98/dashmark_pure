import 'dart:html';
import 'dart:math';
import 'dart:ui';
import 'package:flutter/foundation.dart';
import 'package:flutter/services.dart' show rootBundle;
import 'package:vector_math/vector_math_64.dart';
import 'package:window_manager/window_manager.dart';

class World {
  Vector2 size = Vector2(0.0, 0.0);
  double lastDt = 0.0;

  Image? dashImage;
  double desiredSize = 100.0;
  double scaleToSize = 0.0;

  int _currentId = 0;
  // Matrices
  List<RSTransform> _transforms = [];
  List<Vector2> _positions = [];
  List<Vector2> _velocity = [];

  // FPS
  final _lastFrameTimes = <double>[];

  World() {
    rootBundle.load('assets/images/dash.png').then((data) {
      final asUInt8List = data.buffer.asUint8List();
      decodeImageFromList(asUInt8List, (result) {
        dashImage = result;
        scaleToSize = desiredSize / dashImage!.width;
        debugPrint('Loaded image ${dashImage!.width}x${dashImage!.height}');
      });
    });
  }

  void addDash(double x, double y, double vx, double vy) {
    final id = _currentId++;
    final transform = RSTransform.fromComponents(
      rotation: 0.0,
      scale: scaleToSize,
      anchorX: 0.0,
      anchorY: 0.0,
      translateX: x,
      translateY: y,
    );
    _transforms.add(transform);
    _positions.add(Vector2(x, y));
    _velocity.add(Vector2(vx, vy));
  }

  void input(double x, double y) {
    if (dashImage != null) {
      const amountPerSecond = 1000;
      final amount = (amountPerSecond * lastDt).toInt();
      for (var i = 0; i < amount; i++) {
        // Create a dash at 0,0 every frame
        final vx = 4 * cos(i * 2 * pi / amount);
        final vy = 4 * sin(i * 2 * pi / amount);
        addDash(x, y, vx, vy);
      }
    }
  }

  void update(double t) {
    // Jump around the dashes
    final length = _transforms.length;
    for (var i = 0; i < length; i++) {
      final velocity = _velocity[i];
      final position = _positions[i];

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
      _transforms[i] = RSTransform.fromComponents(
        rotation: 0.0,
        scale: scaleToSize,
        anchorX: 0.0,
        anchorY: 0.0,
        translateX: position.x,
        translateY: position.y,
      );
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
    final title = 'Dashmark - $fpsRounded FPS - ${_transforms.length} entities';
    if (kIsWeb) {
      document.title = title;
    } else {
      windowManager.ensureInitialized();
      windowManager.setTitle(title);
    }
  }

  void render(double t, Canvas canvas) {
    if (dashImage != null) {
      canvas.drawColor(const Color(0xFF000000), BlendMode.srcOver);
      // Draw the dashes using Cavnas.drawAtlas
      final length = _transforms.length;
      final rect = Rect.fromLTWH(
        0,
        0,
        dashImage!.width.toDouble(),
        dashImage!.height.toDouble(),
      );
      final rects = List<Rect>.filled(length, rect);
      final transforms = _transforms;
      const color = Color(0xFFFFFFFF);
      final colors = List<Color>.filled(length, color);
      canvas.drawAtlas(dashImage!, transforms, rects, colors,
          BlendMode.modulate, null, Paint()..color = color);
    }
  }
}
