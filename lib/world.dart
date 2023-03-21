import 'dart:collection';
import 'dart:math';
import 'dart:ui';
import 'batch.dart';
import 'buffer.dart';
import 'package:flutter/foundation.dart';
import 'package:flutter/material.dart' show Colors;
import 'package:flutter/painting.dart';
import 'package:flutter/services.dart' show rootBundle;
import 'package:vector_math/vector_math_64.dart' show Matrix4, Vector2;
import 'bvh.dart';
import 'ffi.dart' if (dart.library.html) 'ffi_web.dart';

class World {
  static const double desiredSize = 64.0;

  Vector2 size = Vector2(0.0, 0.0);
  double lastDt = 0.0;

  Image? dashImage;
  FragmentProgram? fragmentProgram;
  FragmentShader? fragmentShader;
  double scaleToSize = 0.0;

  int _spawnedThisFrame = 0;

  final List<Vector2> _velocity = [];
  final List<Vector2> _position = [];
  final List<double> _rotation = [];

  // Batches
  final List<Batch> _batches = [];

  // FPS
  final _lastFrameTimes = <double>[];

  // Status
  String status = 'Status';

  // Native communication
  final List<RawIndex> _entityIndices = [];
  late RawIndex _bvhIndex;

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

    _bvhIndex = api.createBvh();
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

      HashSet<Batch> touchedBatches = HashSet();
      for (var i = 0; i < amount; i++) {
        // Find a batch where it fits or create a new one
        Batch? batch;
        for (final b in _batches) {
          if (b.length < Batch.batchSize) {
            batch = b;
            break;
          }
        }
        if (batch == null) {
          batch = Batch();
          _batches.add(batch);
          debugPrint('Created new batch (#${_batches.length})');
        }

        final vx = 4 * cos(i * 2 * pi / amount);
        final vy = 4 * sin(i * 2 * pi / amount);
        final indexInBatch = batch.add(
            x, y, dashImage!.width.toDouble(), dashImage!.height.toDouble());
        _position.add(Vector2(x, y));
        _velocity.add(Vector2(vx, vy));
        _rotation.add(0.0);

        // Create the entity
        final entity = api.createEntity();
        _entityIndices.add(entity);
        api.entitySetPosition(index: entity, x: x, y: y);
        api.entitySetOrigin(index: entity, relative: true, x: 0.5, y: 0.5);
        const shape = Shape.ball(radius: 50.0);
        api.entitySetShape(index: entity, shape: shape);

        // Record for later expansion
        touchedBatches.add(batch);
        if (indexInBatch <
            (batch.populateTextureAndIndexCacheFrom ?? batch.length)) {
          batch.populateTextureAndIndexCacheFrom = indexInBatch;
        }
      }

      // Expand the batches
      for (final entry in touchedBatches) {
        entry.cachesNeedExpanding = true;
      }
    }
  }

  void update(double t) {
    _spawnedThisFrame = 0;
    if (dashImage != null && fragmentShader != null) {
      // Jump around the dashes
      final length = _velocity.length;
      for (var i = 0; i < length; ++i) {
        final batch = _batches[i ~/ Batch.batchSize];
        final indexInBatch = i % Batch.batchSize;
        final velocity = _velocity[i];
        final position = _position[i];
        final rotation = _rotation[i];

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

        batch.setPositionFrom(indexInBatch, position);

        // Rotate slightly
        _rotation[i] = rotation + 3.14 * lastDt;
        batch.setRotationFrom(indexInBatch, _rotation[i]);
      }
      lastDt = t;

      // Send all positions to the native world
      final encoder = ByteBufferEncoder();
      RawIndexByteBufferExtensions.encodeArray(
        encoder,
        _entityIndices,
      );
      Vector2ByteBufferExtensions.encodeArray(
        encoder,
        _position,
      );
      final positionsBuffer = encoder.build();
      api.entitiesSetPosition(data: positionsBuffer);

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

      // Calculate the BVH
      // final start = DateTime.now().millisecondsSinceEpoch;
      final entitiesBuilder = ByteBufferEncoder();
      RawIndexByteBufferExtensions.encodeArray(
        entitiesBuilder,
        _entityIndices,
      );
      api.bvhClearAndRebuildRaw(
          index: _bvhIndex, data: entitiesBuilder.build(), dilationFactor: 0.0);
      // final end = DateTime.now().millisecondsSinceEpoch;
      // final bvhTime = end - start;
      // debugPrint('BVH time: $bvhTime ms');
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
      for (final batch in _batches) {
        batch.draw(canvas, paint);
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

void drawFlatBVH(RawIndex bvh, FlatBvh flat, Canvas canvas) {
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
