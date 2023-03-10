import 'dart:math';
import 'dart:ui';

class World {
  var _turn = 0.0;
  double _x;
  double _y;

  World(this._x, this._y);

  void input(double x, double y) {
    _x = x;
    _y = y;
  }

  void render(double t, Canvas canvas) {
    var tau = pi * 2;

    canvas.drawPaint(Paint()..color = Color(0xff880000));
    canvas.save();
    canvas.translate(_x, _y);
    canvas.rotate(tau * _turn);
    var white = Paint()..color = Color(0xffffffff);
    var size = 200.0;
    canvas.drawRect(Rect.fromLTWH(-size / 2, -size / 2, size, size), white);
    canvas.restore();
  }

  void update(double t) {
    var rotationsPerSecond = 0.25;
    _turn += t * rotationsPerSecond;
  }
}
