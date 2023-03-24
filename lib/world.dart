import 'dart:ui';

import 'package:flutter/painting.dart';
import 'package:vector_math/vector_math_64.dart';
import 'ffi_export.dart';

class World {
  Vector2 size = Vector2(0.0, 0.0);
  Vector2 _lastCheckedSize = Vector2(0.0, 0.0);

  World() {}

  void input(double x, double y) {}

  void update(double t) {
    if (size != _lastCheckedSize) {
      api.requestResize(width: size.x.toInt(), height: size.y.toInt());
      _lastCheckedSize = size;
    }
    api.setCurrentTime(time: DateTime.now().millisecondsSinceEpoch.toDouble());
  }

  void render(double t, Canvas canvas) {
    api.requestDraw();
  }
}
