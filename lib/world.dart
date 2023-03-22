import 'dart:math';
import 'dart:ui';
import 'package:dashmark_pure/typed_array/f64.dart';
import 'package:dashmark_pure/typed_array/u64.dart';

import 'batch.dart';
import 'package:flutter/foundation.dart';
import 'package:flutter/material.dart' show Colors;
import 'package:flutter/painting.dart';
import 'package:flutter/services.dart' show rootBundle;
import 'package:vector_math/vector_math_64.dart' show Matrix4, Vector2;
import 'bvh.dart';
import 'ffi_export.dart';

class World {
  static const double desiredSize = 64.0;

  Vector2 size = Vector2(0.0, 0.0);
  double lastDt = 0.0;

  Image? dashImage;
  FragmentProgram? fragmentProgram;
  FragmentShader? fragmentShader;
  double scaleToSize = 0.0;

  int _spawnedThisFrame = 0;

  final Vector2Buffer _velocity = Vector2Buffer();
  final Vector2Buffer _position = Vector2Buffer();
  final Float64Buffer _rotation = Float64Buffer();

  // Batches
  final List<Batch> _batches = [];

  // FPS
  final _lastFrameTimes = List.filled(60, 0.0, growable: true);

  // Status
  String status = 'Status';

  // Native communication
  final RawIndexBuffer _entityIndices = RawIndexBuffer();
  late RawIndex _bvhIndex;

  World() {
    debugPrint('World created');
    rootBundle.load('assets/images/dash_128.png').then((data) {
      // final asUInt8List = Uint8List.view(data.buffer);
      // decodeImageFromList(asUInt8List).then((result) {
      //   dashImage = result;
      //   scaleToSize = desiredSize / dashImage!.width;
      //   debugPrint('Loaded image ${dashImage!.width}x${dashImage!.height}');
      //   status = 'Loaded image ${dashImage!.width}x${dashImage!.height}';
      // });

      // Instead, will will procedurally create an image using a PictureRecorder, a Picture, and a Canvas.
      // First, create a 1x1 pixel image to use in `drawRawAtlas` for each pixel in the image.
      final start = DateTime.now().millisecondsSinceEpoch;
      var pictureRecorder = PictureRecorder();
      var canvas = Canvas(pictureRecorder);
      var paint = Paint();

      // Draw a single pixel
      paint.color = Colors.white;
      canvas.drawRect(const Rect.fromLTWH(0, 0, 1, 1), paint);
      final pixelPicture = pictureRecorder.endRecording();
      final pixelImage = pixelPicture.toImageSync(1, 1);

      // Draw a grid of pixels
      pictureRecorder = PictureRecorder();
      canvas = Canvas(pictureRecorder);

      int size = 512;
      int numVertices = size * size;

      Float32List rstBuffer = Float32List(numVertices * 4);
      Float32List rectBuffer = Float32List(numVertices * 4);
      Int32List colorBuffer = Int32List(numVertices);

      int index = 0;
      for (var x = 0; x < size; x++) {
        for (var y = 0; y < size; y++) {
          int colorA = 255;
          int colorR = 255;
          int colorG = (y / size * 255).toInt();
          int colorB = (x / size * 255).toInt();

          // Set RSTransform data (scos, ssin, tx, ty)
          rstBuffer[index * 4] = 1.0; // scos
          rstBuffer[index * 4 + 1] = 0.0; // ssin
          rstBuffer[index * 4 + 2] = x.toDouble(); // tx
          rstBuffer[index * 4 + 3] = y.toDouble(); // ty

          // Set Rect data (left, top, right, bottom)
          rectBuffer[index * 4] = 0.0; // left
          rectBuffer[index * 4 + 1] = 0.0; // top
          rectBuffer[index * 4 + 2] = 1.0; // right
          rectBuffer[index * 4 + 3] = 1.0; // bottom

          // Set Color data (a, r, g, b)
          colorBuffer[index] =
              Color.fromARGB(colorA, colorR, colorG, colorB).value; // color

          index++;
        }
      }
      var end = DateTime.now().millisecondsSinceEpoch;
      debugPrint('Time to create buffers: ${end - start}ms');

      canvas.drawRawAtlas(pixelImage, rstBuffer, rectBuffer, colorBuffer,
          BlendMode.dst, null, paint);
      end = DateTime.now().millisecondsSinceEpoch;
      debugPrint('Time to record commands: ${end - start}ms');

      final picture = pictureRecorder.endRecording();
      final image = picture.toImageSync(size, size);
      end = DateTime.now().millisecondsSinceEpoch;
      debugPrint('Time to create image: ${end - start}ms');

      dashImage = image;
      scaleToSize = desiredSize / dashImage!.width;
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
        batch.add(
            x, y, dashImage!.width.toDouble(), dashImage!.height.toDouble());
        _position.addXY(x, y);
        _velocity.addXY(vx, vy);
        _rotation.add(0.0);

        // Create the entity
        final entity = api.createEntity();
        _entityIndices.addRawIndex(entity);
        api.entitySetPosition(index: entity, x: x, y: y);
        api.entitySetOrigin(index: entity, relative: true, x: 0.5, y: 0.5);
        const shape = Shape.ball(radius: World.desiredSize / 2);
        api.entitySetShape(index: entity, shape: shape);
      }
    }
  }

  void update(double t) {
    _spawnedThisFrame = 0;
    if (dashImage != null && fragmentShader != null) {
      // Jump around the dashes
      final length = _velocity.vectorsLength;
      for (var i = 0; i < length; ++i) {
        final batch = _batches[i ~/ Batch.batchSize];
        final indexInBatch = i % Batch.batchSize;
        final rotation = _rotation[i];

        // Move the dash
        final newPosX = _position.getX(i) + _velocity.getX(i); // * lastDt;
        final newPosY = _position.getY(i) + _velocity.getY(i); // * lastDt;
        _position.setXY(i, newPosX, newPosY);

        // Bounce off walls
        if (_position.getX(i) < 0) {
          _position.setX(i, 0);
          _velocity.setX(i, -_velocity.getX(i));
        } else if (_position.getX(i) > size.x) {
          _position.setX(i, size.x);
          _velocity.setX(i, -_velocity.getX(i));
        }

        if (_position.getY(i) < 0) {
          _position.setY(i, 0);
          _velocity.setY(i, -_velocity.getY(i));
        } else if (_position.getY(i) > size.y) {
          _position.setY(i, size.y);
          _velocity.setY(i, -_velocity.getY(i));
        }

        // Add gravity
        _velocity.setY(i, _velocity.getY(i) + 0.3);

        batch.setPositionFrom(
            indexInBatch, _position.getX(i), _position.getY(i));

        // Rotate slightly
        _rotation[i] = rotation + 3.14 * lastDt;
        batch.setRotationFrom(indexInBatch, _rotation[i]);
      }
      lastDt = t;

      // Send all positions to the native world
      api.entitiesSetPositionRaw(
          indices: _entityIndices.toUint8List(),
          positions: _position.toUint8List());
      // Send all rotations to the native world
      api.entitiesSetRotationRaw(
          indices: _entityIndices.toUint8List(),
          rotations: _rotation.toUint8List());

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
      api.bvhClearAndRebuildRaw(
          index: _bvhIndex,
          data: _entityIndices.toUint8List(),
          dilationFactor: 0.0);
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
