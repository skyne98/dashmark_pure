import 'dart:math';
import 'dart:ui';
import 'package:flutter/services.dart' show rootBundle;
import 'package:flutter/widgets.dart' show debugPrint;

class World {
  var _turn = 0.0;
  double _x;
  double _y;
  Image? dashImage;

  World(this._x, this._y) {
    rootBundle.load('assets/images/dash.png').then((data) {
      final asUInt8List = data.buffer.asUint8List();
      decodeImageFromList(asUInt8List, (result) {
        dashImage = result;
        debugPrint('Loaded image ${dashImage!.width}x${dashImage!.height}');
      });
    });
  }

  void input(double x, double y) {
    _x = x;
    _y = y;
  }

  void render(double t, Canvas canvas) {
    var tau = pi * 2;

    canvas.drawPaint(Paint()..color = const Color(0xff880000));
    canvas.save();
    canvas.translate(_x, _y);
    canvas.rotate(tau * _turn);
    var white = Paint()..color = const Color(0xffffffff);
    var size = 200.0;
    canvas.drawRect(Rect.fromLTWH(-size / 2, -size / 2, size, size), white);
    canvas.restore();

    // Draw the dash image
    if (dashImage != null) {
      canvas.drawImage(dashImage!, const Offset(0, 0), Paint());
    }
  }

  void update(double t) {
    var rotationsPerSecond = 0.25;
    _turn += t * rotationsPerSecond;
  }
}
