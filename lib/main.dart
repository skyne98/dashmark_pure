import 'package:dashmark_pure/painter.dart';
import 'package:dashmark_pure/world.dart';
import 'package:flutter/material.dart';
import 'ffi_export.dart';
import 'package:flutter/services.dart' as services;

main() {
  api.sayHello().then((_) {
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
  int? textureId;
  services.MethodChannel? channel;

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
    debugPrint("====> build");
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

            return textureId != null
                ? CustomPaint(
                    size: MediaQuery.of(context).size,
                    painter: MyGame(world, pointerX, pointerY, dt),
                    child: Center(
                      child: Texture(textureId: textureId!),
                    ))
                : const Text("Loading...");
          },
        ),
      ),
    );
  }

  @override
  void initState() {
    super.initState();
    previous = currentTime;
    _controller =
        AnimationController(vsync: this, duration: const Duration(seconds: 1))
          ..repeat();
    _animation = Tween<double>(begin: 0.0, end: 1.0).animate(_controller);

    channel = const services.MethodChannel('sturdykeep.com/channel');
    // Initialize the texture here
    channel!.invokeMethod('initTexture').then((value) {
      setState(() {
        textureId = value;
        api.moveStateToUiThread();
      });
    });
  }

  void pointerUpdate(details) {
    pointerX = details.globalPosition.dx;
    pointerY = details.globalPosition.dy;
    world.input(pointerX, pointerY);
  }
}
