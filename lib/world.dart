import 'dart:math';
import 'dart:typed_data';
import 'dart:ui';
import 'package:dashmark_pure/api/rendering.dart' as rendering;
import 'package:dashmark_pure/api/transform.dart';

import 'package:flutter/foundation.dart';
import 'package:flutter/material.dart' show Colors;
import 'package:flutter/painting.dart';
import 'package:flutter/services.dart' show rootBundle;
import 'package:vector_math/vector_math_64.dart' show Vector2;
import 'bvh.dart';
import 'ffi_export.dart';
import 'typed_buffer/mod.dart';

class World {
  static const double desiredSize = 64.0;

  Vector2 size = Vector2(0.0, 0.0);
  double lastDt = 0.0;

  Image? dashImage;
  FragmentProgram? fragmentProgram;
  FragmentShader? fragmentShader;
  double scaleToSize = 0.0;

  int _spawnedThisFrame = 0;

  final GenerationalIndexBuffer _entityIndices = GenerationalIndexBuffer();
  final Int32Buffer _priorities = Int32Buffer();
  final ColorBuffer _colors = ColorBuffer();
  final Vector64Buffer _velocity = Vector64Buffer();
  final Vector64Buffer _position = Vector64Buffer();
  final Float64Buffer _rotation = Float64Buffer();
  final Vector64Buffer _scale = Vector64Buffer();
  final Vector64Buffer _origin = Vector64Buffer();
  final Vector64Buffer _size = Vector64Buffer();

  // FPS
  final _lastFrameTimes = List.filled(60, 0.0, growable: true);

  // Status
  String status = 'Status';

  World() {
    debugPrint('World created');
    rootBundle.load('assets/images/dash_128.png').then((data) {
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
    if (dashImage != null && fragmentShader != null) {
      const amountPerSecond = 5000;
      var amount = (amountPerSecond * lastDt).toInt();
      if (amount > amountPerSecond) {
        amount = amountPerSecond;
      }
      if (_spawnedThisFrame > amountPerSecond / 60) {
        return;
      }
      _spawnedThisFrame += amount;

      for (var i = 0; i < amount; i++) {
        final vx = 4 * cos(i * 2 * pi / amount);
        final vy = 4 * sin(i * 2 * pi / amount);
        _position.add(Vector2(x, y));
        _velocity.add(Vector2(vx, vy));
        _rotation.add(0.0);
        _scale.add(Vector2(1.0, 1.0));
        final origin = Vector2(desiredSize / 2, desiredSize / 2);
        _origin.add(origin);

        // Create the entity
        final entity = api.createEntity();
        _entityIndices.add(entity);
        setPosition(entity, x, y);
        const shape = Shape.ball(radius: World.desiredSize / 2);
        api.entitySetShape(index: entity, shape: shape);
        setOrigin(entity, origin.x, origin.y);
        final vertices = Vector64Buffer();
        vertices.add(Vector2(0.0, 0.0));
        vertices.add(Vector2(0.0, 64.0));
        vertices.add(Vector2(64.0, 64.0));
        vertices.add(Vector2(64.0, 0.0));
        rendering.setVertices(entity, vertices);
        final texCoords = Vector64Buffer();
        texCoords.add(Vector2(0.0, 0.0));
        texCoords.add(Vector2(128.0, 0.0));
        texCoords.add(Vector2(128.0, 128.0));
        texCoords.add(Vector2(0.0, 128.0));
        rendering.setTexCoords(entity, texCoords);
        rendering.setIndices(entity, Uint16Buffer.fromList([0, 1, 2, 0, 2, 3]));
        rendering.setColor(entity, Colors.red);
      }
    }
  }

  void update(double t) {
    _spawnedThisFrame = 0;
    var updateStopwatch = Stopwatch()..start();
    if (dashImage != null && fragmentShader != null) {
      // Jump around the dashes
      final length = _velocity.length;
      for (var i = 0; i < length; ++i) {
        var position = _position[i];
        var rotation = _rotation[i];
        var velocity = _velocity[i];

        // Move the dash
        position += velocity;

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

        // Rotate slightly
        rotation += 3.14 * lastDt;

        _position[i] = position;
        _rotation[i] = rotation;
        _velocity[i] = velocity;
      }
      lastDt = t;

      // Send all data to the native world
      var stopwatch = Stopwatch()..start();
      setTransformsBulk(_entityIndices, _position, _origin, _rotation, _scale);
      stopwatch.stop();
      debugPrint('Set data in ${stopwatch.elapsedMilliseconds}ms');

      // Call the native world update
      stopwatch = Stopwatch()..start();
      api.update(dt: t);
      stopwatch.stop();
      debugPrint('Native update in ${stopwatch.elapsedMilliseconds}ms');

      // Make a test query and print the count of entities
      stopwatch = Stopwatch()..start();
      final center = Vector2(size.x / 2, size.y / 2);
      final screenThird = Vector2(size.x / 3, size.y / 3);
      final queryResults = Int32List.view(api
          .queryAabbRaw(
              x: center.x - screenThird.x / 2,
              y: center.y - screenThird.y / 2,
              width: screenThird.x,
              height: screenThird.y)
          .buffer);
      stopwatch.stop();
      debugPrint(
          'Query results: ${queryResults.length / 2} in ${stopwatch.elapsedMilliseconds}ms');

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
          'Dashmark - $fpsRounded FPS - $percentileFpsRounded FPS (95%) - $medianFpsRounded FPS (50%) - ${_velocity.length} dashes';
      status = title;
    }

    var updateTime = updateStopwatch.elapsedMilliseconds;
    updateStopwatch.stop();
    if (updateTime > 0) {
      debugPrint('Update time: $updateTime ms');
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
      var time = stopwatch.elapsedMilliseconds;
      stopwatch.stop();
      debugPrint('Received all rendering data in $time ms');

      // Render the batches
      debugPrint('Rendering $batchCount batches');
      stopwatch = Stopwatch()..start();
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
      time = stopwatch.elapsedMilliseconds;
      stopwatch.stop();
      debugPrint('Rendered all batches in $time ms');

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
