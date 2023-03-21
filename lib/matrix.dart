// Inspired by: https://github.com/chrvadala/transformation-matrix
import 'dart:math';
import 'dart:typed_data';

import 'package:vector_math/vector_math_64.dart';

class Affine {
  // Matrix description visual:
  // | a c e |
  // | b d f |

  double a = 1.0;
  double b = 0.0;
  double c = 0.0;
  double d = 1.0;
  double e = 0.0;
  double f = 0.0;

  // Static methods
  static Affine fromList(List<double> list) {
    final matrix = Affine();
    matrix.a = list[0];
    matrix.b = list[1];
    matrix.c = list[2];
    matrix.d = list[3];
    matrix.e = list[4];
    matrix.f = list[5];
    return matrix;
  }

  static Affine fromFloat32List(Float32List list) {
    final matrix = Affine();
    matrix.a = list[0];
    matrix.b = list[1];
    matrix.c = list[2];
    matrix.d = list[3];
    matrix.e = list[4];
    matrix.f = list[5];
    return matrix;
  }

  static Affine fromFloat64List(Float64List list) {
    final matrix = Affine();
    matrix.a = list[0];
    matrix.b = list[1];
    matrix.c = list[2];
    matrix.d = list[3];
    matrix.e = list[4];
    matrix.f = list[5];
    return matrix;
  }

  static Affine identity() {
    final matrix = Affine();
    matrix.setIdentity();
    return matrix;
  }

  static Affine multiply(Affine a, Affine b) {
    final matrix = Affine();
    /*
    a: m1.a * m2.a + m1.c * m2.b,
    c: m1.a * m2.c + m1.c * m2.d,
    e: m1.a * m2.e + m1.c * m2.f + m1.e,
    b: m1.b * m2.a + m1.d * m2.b,
    d: m1.b * m2.c + m1.d * m2.d,
    f: m1.b * m2.e + m1.d * m2.f + m1.f
    */
    matrix.a = a.a * b.a + a.c * b.b;
    matrix.c = a.a * b.c + a.c * b.d;
    matrix.e = a.a * b.e + a.c * b.f + a.e;
    matrix.b = a.b * b.a + a.d * b.b;
    matrix.d = a.b * b.c + a.d * b.d;
    matrix.f = a.b * b.e + a.d * b.f + a.f;
    return matrix;
  }

  // Methods
  void setIdentity() {
    a = 1.0;
    c = 0.0;
    e = 0.0;
    b = 0.0;
    d = 1.0;
    f = 0.0;
  }

  void setTranslation(double x, double y) {
    setIdentity();
    e = x;
    f = y;
  }

  void setScale(double x, double y) {
    setIdentity();
    a = x;
    d = y;
  }

  void setRotation(double angle) {
    setIdentity();
    final ccos = cos(angle);
    final ssin = sin(angle);
    a = ccos;
    b = ssin;
    c = -ssin;
    d = ccos;
  }

  void setTransform(double x, double y, double rotation, double scaleX,
      double scaleY, double originX, double originY) {
    // Calculate the combined transformation matrix directly
    final ccos = cos(rotation);
    final ssin = sin(rotation);
    a = ccos * scaleX;
    b = ssin * scaleX;
    c = -ssin * scaleY;
    d = ccos * scaleY;
    e = x - originX * a - originY * c;
    f = y - originX * b - originY * d;
  }

  void multiplyBy(Affine other) {
    final a = this.a * other.a + this.c * other.b;
    final c = this.a * other.c + this.c * other.d;
    final e = this.a * other.e + this.c * other.f + this.e;
    final b = this.b * other.a + this.d * other.b;
    final d = this.b * other.c + this.d * other.d;
    final f = this.b * other.e + this.d * other.f + this.f;
    this.a = a;
    this.c = c;
    this.e = e;
    this.b = b;
    this.d = d;
    this.f = f;
  }

  void transformRaw<L extends List<double>>(L list, int offset, int length) {
    for (var i = offset; i < length; i += 2) {
      final offsetZero = i;
      final offsetOne = i + 1;
      final x = list[offsetZero];
      final y = list[offsetOne];
      list[offsetZero] = (x * a) + (y * c) + e;
      list[offsetOne] = (x * b) + (y * d) + f;
    }
  }

  void transformRawFrom<L extends List<double>>(
      L list, int offset, double x, double y) {
    list[offset] = (x * a) + (y * c) + e;
    list[offset + 1] = (x * b) + (y * d) + f;
  }

  void transformVector2(Vector2 vector) {
    final x = vector.x;
    final y = vector.y;
    vector.x = (x * a) + (y * c) + e;
    vector.y = (x * b) + (y * d) + f;
  }
}
