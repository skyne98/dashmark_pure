import 'dart:math';
import 'dart:typed_data';
import 'dart:ui';
import 'package:dashmark_pure/api/rendering.dart' as rendering;
import 'package:dashmark_pure/api/transform.dart';

import 'package:flutter/foundation.dart';
import 'package:flutter/material.dart' show Colors;
import 'package:flutter/painting.dart';
import 'package:flutter/services.dart' show rootBundle;
import 'package:vector_math/vector_math.dart' show Vector2;
import 'bvh.dart';
import 'ffi_export.dart';
import 'typed_buffer/mod.dart';

class World {
  static const double spriteSize = 16.0;
  static const double desiredSize = 8.0;

  Vector2 size = Vector2(0.0, 0.0);
  double lastDt = 0.0;

  Image? dashImage;
  FragmentProgram? fragmentProgram;
  FragmentShader? fragmentShader;
  double scaleToSize = 0.0;

  DateTime _lastSpawned = DateTime.now();

  final GenerationalIndexBuffer _entityIndices = GenerationalIndexBuffer();
  final Int32Buffer _priorities = Int32Buffer();
  final ColorBuffer _colors = ColorBuffer();
  final Vector32Buffer _velocity = Vector32Buffer();
  final Vector32Buffer _position = Vector32Buffer();
  final Float32Buffer _rotation = Float32Buffer();
  final Vector32Buffer _scale = Vector32Buffer();
  final Vector32Buffer _origin = Vector32Buffer();
  final Vector32Buffer _size = Vector32Buffer();

  // FPS
  final _lastFrameTimes = List.filled(60, 0.0, growable: true);

  // Status
  String status = 'Status';

  World() {
    debugPrint('World created');
    rootBundle.load('assets/images/dash_16.png').then((data) {
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
    FragmentProgram.fromAsset('shaders/sprite.frag').then((result) {
      fragmentProgram = result;
      fragmentShader = fragmentProgram!.fragmentShader();
      debugPrint('Loaded fragment shader');
      status = 'Loaded fragment shader';
    }).catchError((error, stackTrace) {
      debugPrint('Error loading fragment shader: $error');
      status = 'Error loading fragment shader: $error';
    });
  }

  void input(double x, double y) {
    if (DateTime.now().difference(_lastSpawned) >
        const Duration(milliseconds: 1)) {
      final mousePos = Vector2(x, y);
      if (dashImage != null && fragmentShader != null) {
        // Spawn 6 sprites in a circle around a non-existent sprite
        // in the middle
        for (var i = 0; i < 6; i++) {
          final angle = i * pi / 3;
          final offset =
              Vector2(cos(angle), sin(angle)).normalized() * desiredSize;
          final position = mousePos + offset;

          _position.add(position);
          _rotation.add(0.0);
          _scale.add(Vector2(1.0, 1.0));
          final origin = Vector2(desiredSize / 2, desiredSize / 2);
          _origin.add(origin);

          // Create the entity
          final entity = api.createEntity();
          _entityIndices.add(entity);
          setPosition(entity, position.x, position.y);
          const shape = Shape.ball(radius: World.desiredSize / 2);
          api.entitySetShape(index: entity, shape: shape);
          setOrigin(entity, origin.x, origin.y);
          final vertices = Vector32Buffer();
          vertices.add(Vector2(0.0, 0.0));
          vertices.add(Vector2(0.0, desiredSize));
          vertices.add(Vector2(desiredSize, desiredSize));
          vertices.add(Vector2(desiredSize, 0.0));
          rendering.setVertices(entity, vertices);
          final texCoords = Vector32Buffer();
          texCoords.add(Vector2(0.0, 0.0));
          texCoords.add(Vector2(spriteSize, 0.0));
          texCoords.add(Vector2(spriteSize, spriteSize));
          texCoords.add(Vector2(0.0, spriteSize));
          rendering.setTexCoords(entity, texCoords);
          rendering.setIndices(
              entity, Uint16Buffer()..cloneFromIterable([0, 1, 2, 0, 2, 3]));

          // Create a rainbow color (RGB) over time
          final time = DateTime.now().millisecondsSinceEpoch;
          final color = generateRainbowColor(time, saturation: 0.8);
          rendering.setColor(entity, color);
        }
      }
      _lastSpawned = DateTime.now();
    }
  }

  Color generateRainbowColor(int timeMs,
      {double saturation = 1.0, double lightness = 0.5}) {
    int hue = (timeMs ~/ 10) % 360; // Change hue value over time
    double h = hue / 360;
    return Color.fromARGB(
        255,
        (hslToRgb(h, saturation, lightness, 0) * 255).round(),
        (hslToRgb(h, saturation, lightness, 1) * 255).round(),
        (hslToRgb(h, saturation, lightness, 2) * 255).round());
  }

  // HSL to RGB conversion
  double hslToRgb(double h, double s, double l, int rgbIndex) {
    double t1, t2, tRgb;
    if (s == 0.0) {
      return l;
    }
    if (l < 0.5) {
      t2 = l * (1.0 + s);
    } else {
      t2 = l + s - l * s;
    }
    t1 = 2.0 * l - t2;
    double hue = h + rgbIndex / 3.0;
    if (hue < 0) hue += 1;
    if (hue > 1) hue -= 1;
    if (6 * hue < 1) {
      tRgb = t1 + (t2 - t1) * 6 * hue;
    } else if (2 * hue < 1) {
      tRgb = t2;
    } else if (3 * hue < 2) {
      tRgb = t1 + (t2 - t1) * (2 / 3 - hue) * 6;
    } else {
      tRgb = t1;
    }
    return tRgb;
  }

  void update(double t) {
    if (dashImage != null && fragmentShader != null) {
      // Jump around the dashes
      // final length = _velocity.length;
      // for (var i = 0; i < length; ++i) {
      //   var position = _position[i];
      //   var rotation = _rotation[i];
      //   var velocity = _velocity[i];

      //   // Move the dash
      //   position += velocity;

      //   // Bounce off walls
      //   if (position.x < 0) {
      //     position.x = 0;
      //     velocity.x = -velocity.x;
      //   } else if (position.x > size.x) {
      //     position.x = size.x;
      //     velocity.x = -velocity.x;
      //   }

      //   if (position.y < 0) {
      //     position.y = 0;
      //     velocity.y = -velocity.y;
      //   } else if (position.y > size.y) {
      //     position.y = size.y;
      //     velocity.y = -velocity.y;
      //   }

      //   // Add gravity
      //   velocity.y += 0.3;

      //   // Rotate slightly
      //   rotation += 3.14 * lastDt;

      //   _position[i] = position;
      //   _rotation[i] = rotation;
      //   _velocity[i] = velocity;
      // }
      lastDt = t;

      // Send all data to the native world
      // setTransformsBulk(_entityIndices, _position, _origin, _rotation, _scale);

      // Update the screen size
      api.screenSizeChanged(width: size.x, height: size.y);

      // Call the native world update
      api.update(dt: t);

      // Make a test query and print the count of entities
      final center = Vector2(size.x / 2, size.y / 2);
      final screenThird = Vector2(size.x / 3, size.y / 3);
      Int32List.view(api
          .queryAabbRaw(
              x: center.x - screenThird.x / 2,
              y: center.y - screenThird.y / 2,
              width: screenThird.x,
              height: screenThird.y)
          .buffer);

      // FPS
      _lastFrameTimes.add(t);
      // keep the list max length
      if (_lastFrameTimes.length > 100) {
        _lastFrameTimes.removeAt(0);
      }
      final fps = 1 /
          (_lastFrameTimes.fold(0.0, (a, b) => a + b) / _lastFrameTimes.length);
      final fpsRounded = fps.round();
      // Calculate the 95th percentile frame rate
      const percentile = 0.95;
      final sortedFrameTimes = _lastFrameTimes.toList()..sort();
      final percentileFrameTimes = sortedFrameTimes
          .sublist((sortedFrameTimes.length.toDouble() * percentile).round());
      final percentileFrameTime =
          percentileFrameTimes.fold(0.0, (a, b) => a + b) /
              percentileFrameTimes.length;
      final percentileFps = 1 / percentileFrameTime;
      final percentileFpsRounded = percentileFps.round();
      // Calculate the median frame rate
      final medianFrameTimes = sortedFrameTimes
          .sublist((sortedFrameTimes.length.toDouble() * 0.5).round());
      final medianFrameTime =
          medianFrameTimes.fold(0.0, (a, b) => a + b) / medianFrameTimes.length;
      final medianFps = 1 / medianFrameTime;
      final medianFpsRounded = medianFps.round();
      final title =
          'Dashmark - $fpsRounded FPS - $percentileFpsRounded FPS (95%) - $medianFpsRounded FPS (50%) - ${_position.length} dashes';
      status = title;
    }
  }

  void render(double t, Canvas canvas) {
    if (dashImage != null && fragmentShader != null) {
      canvas.drawColor(const Color(0xFF000000), BlendMode.srcOver);

      // Get the Flat BVH
      // final start = DateTime.now().millisecondsSinceEpoch;
      // final bvh = FlatBvh.fromBytes(api.bvhFlatten(index: _bvhIndex));
      // drawFlatBVH(_bvhIndex, bvh, canvas);
      // final end = DateTime.now().millisecondsSinceEpoch;
      // final time = end - start;
      // debugPrint('BVH flatten time: $time ms');

      // Prepare the shader
      fragmentShader!.setImageSampler(0, dashImage!);
      fragmentShader!.setFloat(0, dashImage!.width.toDouble());
      fragmentShader!.setFloat(1, dashImage!.height.toDouble());
      final paint = Paint();
      paint.shader = fragmentShader;

      // Draw the batches
      var stopwatch = Stopwatch()..start();
      final vertBatches = <Float32List>[];
      final indexBatches = <Uint16List>[];
      final texCoordBatches = <Float32List>[];
      final colorBatches = <Int32List>[];
      final batchCount = rendering.batchesCount();

      for (var i = 0; i < batchCount; ++i) {
        vertBatches.add(rendering.vertices(i));
        indexBatches.add(rendering.indices(i));
        texCoordBatches.add(rendering.texCoords(i));
        colorBatches.add(rendering.colors(i));
      }

      // Render the batches
      for (var i = 0; i < batchCount; ++i) {
        final verts = vertBatches[i];
        final indices = indexBatches[i];
        final texCoords = texCoordBatches[i];
        final colors = colorBatches[i];
        final vertBuffer = Vertices.raw(
          VertexMode.triangles,
          verts,
          indices: indices,
          textureCoordinates: texCoords,
          colors: colors,
        );
        canvas.drawVertices(vertBuffer, BlendMode.modulate, paint);
      }

      // Draw status in the middle
      final text = TextSpan(
        text: status,
        style: const TextStyle(
          color: Colors.green,
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

void drawFlatBVH(GenerationalIndex bvh, FlatBvh flat, Canvas canvas) {
  var overallDepth = 0;

  // Find the max depth of the BVH
  final length = flat.depth.length;
  for (var i = 0; i < length; i++) {
    final depth = flat.depth[i].toInt();
    if (depth > overallDepth) {
      overallDepth = depth;
    }
  }

  final paint = Paint()
    ..color = Color.fromARGB(255, 255, 0, 0)
    ..style = PaintingStyle.stroke
    ..strokeWidth = 1.0;

  for (var i = 0; i < length; i++) {
    final minX = flat.minX[i];
    final minY = flat.minY[i];
    final maxX = flat.maxX[i];
    final maxY = flat.maxY[i];
    final depth = flat.depth[i].toInt();
    final color =
        Color.fromARGB(255, 255, 255 - depth * 255 ~/ overallDepth, 0);
    paint.color = color;
    canvas.drawRect(Rect.fromLTRB(minX, minY, maxX, maxY), paint);
  }
}
