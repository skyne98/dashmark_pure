import 'package:dashmark_pure/world.dart';
import 'package:flutter/widgets.dart';
import 'package:vector_math/vector_math_64.dart';

class MyGame extends CustomPainter {
  final World world;
  final double x;
  final double y;
  final double t;

  MyGame(this.world, this.x, this.y, this.t);

  @override
  void paint(Canvas canvas, Size size) {
    world.input(x, y);
    world.update(t);
    world.render(t, canvas);
    world.size = Vector2(size.width, size.height);
  }

  @override
  bool shouldRepaint(CustomPainter oldDelegate) => true;
}
