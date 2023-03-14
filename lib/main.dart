import 'dart:math';
import 'dart:typed_data';

import 'package:dashmark_pure/painter.dart';
import 'package:dashmark_pure/world.dart';
import 'package:flutter/material.dart';
import 'ffi.dart' if (dart.library.html) 'ffi_web.dart';

main() {
  api.sayHelloAsync().then((_) {
    runApp(MyApp());
  });
}

class MyApp extends StatelessWidget {
  @override
  Widget build(BuildContext context) {
    return MaterialApp(
      home: MyPage(),
    );
  }
}

class MyPage extends StatefulWidget {
  @override
  _MyPageState createState() => _MyPageState();
}

class _MyPageState extends State<MyPage> with SingleTickerProviderStateMixin {
  late AnimationController _controller;
  late Animation<double> _animation;
  late World world;
  final DateTime _initialTime = DateTime.now();
  double previous = 0.0;
  double pointerX = 0.0;
  double pointerY = 0.0;
  double get currentTime =>
      DateTime.now().difference(_initialTime).inMilliseconds / 1000.0;

  _MyPageState() {
    world = World();
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      body: GestureDetector(
        onTapDown: pointerUpdate,
        onTapUp: pointerUpdate,
        onVerticalDragUpdate: pointerUpdate,
        onHorizontalDragUpdate: pointerUpdate,
        child: AnimatedBuilder(
          animation: _animation,
          builder: (BuildContext contex, Widget? child) {
            var curr = currentTime;
            var dt = curr - previous;
            previous = curr;

            return CustomPaint(
              size: MediaQuery.of(context).size,
              painter: MyGame(world, pointerX, pointerY, dt),
            );
          },
        ),
      ),
    );
  }

  @override
  void initState() {
    previous = currentTime;
    _controller =
        AnimationController(vsync: this, duration: const Duration(seconds: 1))
          ..repeat();
    _animation = Tween<double>(begin: 0.0, end: 1.0).animate(_controller);

    // Test out the speed of generating morton codes
    const gridSize = 1000000;
    final gridSide = sqrt(gridSize);
    final valuesX = <double>[];
    for (var i = 0; i < gridSide; i++) {
      for (var j = 0; j < gridSide; j++) {
        valuesX.add(i.toDouble());
      }
    }
    final valuesXFloat64List = Float64List.fromList(valuesX);
    final valuesY = <double>[];
    for (var i = 0; i < gridSide; i++) {
      for (var j = 0; j < gridSide; j++) {
        valuesY.add(j.toDouble());
      }
    }
    final valuesYFloat64List = Float64List.fromList(valuesY);
    final stopwatch = Stopwatch()..start();
    final result =
        api.mortonCodes(xs: valuesXFloat64List, ys: valuesYFloat64List);
    stopwatch.stop();
    final elapsed = stopwatch.elapsedMilliseconds;
    debugPrint('Generated $gridSize morton codes in $elapsed ms');
    final average = elapsed / gridSize;
    debugPrint('Average: $average ms');

    // AABB
    final testAABB = api.aabbNew(minX: 0.0, minY: 0.0, maxX: 1.0, maxY: 1.0);
    final center = api.aabbCenter(aabb: testAABB);
    debugPrint('AABB center: ${center[0]}, ${center[1]}');

    final random = Random();
    // Generate 1 million random AABBs
    final minXS = <double>[];
    final minYS = <double>[];
    final maxXS = <double>[];
    final maxYS = <double>[];
    // ... generate random values
    for (var i = 0; i < gridSize; i++) {
      minXS.add(random.nextDouble());
      minYS.add(random.nextDouble());
      final width = random.nextDouble();
      final height = random.nextDouble();
      maxXS.add(minXS[i] + width);
      maxYS.add(minYS[i] + height);
    }
    final minXSFloat64List = Float64List.fromList(minXS);
    final minYSFloat64List = Float64List.fromList(minYS);
    final maxXSFloat64List = Float64List.fromList(maxXS);
    final maxYSFloat64List = Float64List.fromList(maxYS);
    stopwatch.reset();
    stopwatch.start();
    final aabbs_one_by_one = <RwLockAabb>[];
    for (var i = 0; i < gridSize; i++) {
      aabbs_one_by_one.add(api.aabbNew(
          minX: minXS[i], minY: minYS[i], maxX: maxXS[i], maxY: maxYS[i]));
    }
    stopwatch.stop();
    debugPrint(
        'Generated $gridSize AABBs one by one in ${stopwatch.elapsedMilliseconds} ms');
    stopwatch.reset();
    stopwatch.start();
    final aabbs_bulk = api.aabbNewBulk(
        minXs: minXSFloat64List,
        minYs: minYSFloat64List,
        maxXs: maxXSFloat64List,
        maxYs: maxYSFloat64List);
    stopwatch.stop();
    debugPrint(
        'Generated $gridSize AABBs in bulk in ${stopwatch.elapsedMilliseconds} ms');

    // Test the internal benchmark
    final benchmarkResult = api.aabbNewBulkBenchmark(
        minXs: minXSFloat64List,
        minYs: minYSFloat64List,
        maxXs: maxXSFloat64List,
        maxYs: maxYSFloat64List);
    debugPrint('Benchmark result: $benchmarkResult ms');

    // Make sure the two methods of generating AABBs are equivalent
    for (var i = 0; i < gridSize; i++) {
      final aabbOneByOne = aabbs_one_by_one[i];
      final aabbBulk = aabbs_bulk[i];
      final aabbOneByOneMin = api.aabbMin(aabb: aabbOneByOne);
      final aabbOneByOneMax = api.aabbMax(aabb: aabbOneByOne);
      final aabbBulkMin = api.aabbMin(aabb: aabbBulk);
      final aabbBulkMax = api.aabbMax(aabb: aabbBulk);
      if (aabbOneByOneMin[0] != aabbBulkMin[0] ||
          aabbOneByOneMin[1] != aabbBulkMin[1] ||
          aabbOneByOneMax[0] != aabbBulkMax[0] ||
          aabbOneByOneMax[1] != aabbBulkMax[1]) {
        debugPrint(
            'AABBs are not equivalent at index $i ($aabbOneByOneMin, $aabbOneByOneMax) vs ($aabbBulkMin, $aabbBulkMax)');
        break;
      }
    }

    // Test out the speed of AABB intersection
    stopwatch.reset();
    stopwatch.start();
    for (var i = 0; i < gridSize; i++) {
      api.aabbIntersects(aabbLeft: aabbs_bulk[i], aabbRight: testAABB);
    }
    stopwatch.stop();
    debugPrint(
        'Tested $gridSize AABB intersections in ${stopwatch.elapsedMilliseconds} ms');

    // Test the speed of building a BVH
    stopwatch.reset();
    stopwatch.start();
    final bvh = api.bvhNew(aabbs: aabbs_bulk);
    stopwatch.stop();
    debugPrint(
        'Built BVH with $gridSize AABBs in ${stopwatch.elapsedMilliseconds} ms');
  }

  void pointerUpdate(details) {
    pointerX = details.globalPosition.dx;
    pointerY = details.globalPosition.dy;
    world.input(pointerX, pointerY);
  }
}
