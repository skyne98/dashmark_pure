import 'dart:math';

import 'package:flutter/widgets.dart';
import 'package:vector_math/vector_math.dart';

class AABB {
  double x = 0.0;
  double y = 0.0;

  double width = 0.0;
  double height = 0.0;

  double centerX = 0.0;
  double centerY = 0.0;

  dynamic data;

  AABB([this.x = 0.0, this.y = 0.0, this.width = 0.0, this.height = 0.0]) {
    centerX = x + width / 2;
    centerY = y + height / 2;
  }

  factory AABB.fromList(List<AABB> aabbs) {
    var aabb = AABB();
    var minX = double.maxFinite;
    var minY = double.maxFinite;
    var maxX = -double.maxFinite;
    var maxY = -double.maxFinite;
    for (var other in aabbs) {
      final otherPositionX = other.x;
      final otherPositionY = other.y;
      minX = min(minX, otherPositionX);
      minY = min(minY, otherPositionY);
      maxX = max(maxX, otherPositionX + other.width);
      maxY = max(maxY, otherPositionY + other.height);
    }
    aabb.x = minX;
    aabb.y = minY;
    aabb.width = maxX - minX;
    aabb.height = maxY - minY;
    aabb.centerX = aabb.x + aabb.width / 2;
    aabb.centerY = aabb.y + aabb.height / 2;
    return aabb;
  }

  void setPositionFrom(Vector2 other) {
    x = other.x;
    y = other.y;
    centerX = x + width / 2;
    centerY = y + height / 2;
  }

  // Collisions
  bool collidesWith(AABB other) {
    final otherPositionX = other.x;
    final otherPositionY = other.y;
    final otherSizeX = other.width;
    final otherSizeY = other.height;

    return x <= otherPositionX + otherSizeX &&
        x + width >= otherPositionX &&
        y <= otherPositionY + otherSizeY &&
        y + height >= otherPositionY;
  }

  bool containsPoint(Vector2 point) {
    final pointX = point.x;
    final pointY = point.y;
    return x <= pointX &&
        x + width >= pointX &&
        y <= pointY &&
        y + height >= pointY;
  }

  bool containsAABB(AABB other) {
    // Without the use of existing methods
    final otherPositionX = other.x;
    final otherPositionY = other.y;
    final otherSizeX = other.width;
    final otherSizeY = other.height;

    return x <= otherPositionX &&
        x + width >= otherPositionX + otherSizeX &&
        y <= otherPositionY &&
        y + height >= otherPositionY + otherSizeY;
  }
}

// A up-to-bottom built bounding volume hierarchy
class BVH {
  static int desiredSize = 4;

  late BVHNode _root;
  final List<AABB> _leafs;

  BVH(this._leafs) {
    // Stopwatch stopwatch = Stopwatch()..start();
    _root = _buildNonRecursive(_leafs);
    // stopwatch.stop();
    // final time = stopwatch.elapsedMilliseconds;
    // debugPrint(
    //     'Built BVH with depth ${_root.depth} and ${_root.nodesCount} nodes, ${_root.leafNodesCount} leaf nodes and $averageLeafesPerLeafNode average leafes per leaf node in $time ms');
  }

  int get depth => _root.depth;
  int get nodesCount => _root.nodesCount;
  int get leafNodesCount => _root.leafNodesCount;
  double get averageLeafesPerLeafNode => _leafs.length / leafNodesCount;

  // Building it
  BVHNode _buildNonRecursive(List<AABB> leafs) {
    // debugPrint('Building BVH with ${leafs.length} leafs');

    var stack = <BVHNode>[];
    var aabb = AABB.fromList(leafs);
    var root = BVHNode(aabb, null, null, leafs);
    stack.add(root);

    while (stack.isNotEmpty) {
      var node = stack.removeLast();
      if (node.leafs!.length <= desiredSize) {
        continue;
      }

      var left = <AABB>[];
      var right = <AABB>[];
      bool isVertical = node.aabb.height > node.aabb.width;
      for (var leaf in node.leafs!) {
        if (isVertical) {
          if (leaf.centerY <= node.aabb.centerY) {
            left.add(leaf);
          } else {
            right.add(leaf);
          }
        } else {
          if (leaf.centerX <= node.aabb.centerX) {
            left.add(leaf);
          } else {
            right.add(leaf);
          }
        }
      }

      if (left.isEmpty && right.isNotEmpty) {
        continue;
      }
      if (right.isEmpty && left.isNotEmpty) {
        continue;
      }

      node.left = BVHNode(AABB.fromList(left), null, null, left);
      node.right = BVHNode(AABB.fromList(right), null, null, right);
      node.leafs = null;

      stack.add(node.left!);
      stack.add(node.right!);
    }

    return root;
  }

  // Querying it
  List<AABB> queryCollisions(AABB aabb) {
    var collisions = <AABB>[];
    _queryCollisions(_root, aabb, collisions);
    return collisions;
  }

  void _queryCollisions(BVHNode node, AABB aabb, List<AABB> collisions) {
    if (node.aabb.collidesWith(aabb) == false) {
      return;
    }

    if (node.leafs != null) {
      for (var leaf in node.leafs!) {
        if (leaf.collidesWith(aabb)) {
          collisions.add(leaf);
        }
      }
    } else {
      _queryCollisions(node.left!, aabb, collisions);
      _queryCollisions(node.right!, aabb, collisions);
    }
  }

  List<AABB> queryPoint(Vector2 point) {
    var collisions = <AABB>[];
    _queryPoint(_root, point, collisions);
    return collisions;
  }

  void _queryPoint(BVHNode node, Vector2 point, List<AABB> collisions) {
    if (node.aabb.containsPoint(point) == false) {
      return;
    }

    if (node.leafs != null) {
      for (var leaf in node.leafs!) {
        if (leaf.containsPoint(point)) {
          collisions.add(leaf);
        }
      }
    } else {
      _queryPoint(node.left!, point, collisions);
      _queryPoint(node.right!, point, collisions);
    }
  }

  List<AABB> queryContainsThis(AABB aabb) {
    var collisions = <AABB>[];
    _queryContainsThis(_root, aabb, collisions);
    return collisions;
  }

  void _queryContainsThis(BVHNode node, AABB aabb, List<AABB> collisions) {
    if (node.aabb.containsAABB(aabb) == false) {
      return;
    }

    if (node.leafs != null) {
      final length = node.leafs!.length;
      for (var i = 0; i < length; i++) {
        var leaf = node.leafs![i];
        if (aabb.containsAABB(leaf)) {
          collisions.add(leaf);
        }
      }
    } else {
      _queryContainsThis(node.left!, aabb, collisions);
      _queryContainsThis(node.right!, aabb, collisions);
    }
  }

  // Rendering
  void draw(Canvas canvas) {
    _root.draw(canvas);
  }
}

class BVHNode {
  AABB aabb;
  BVHNode? left;
  BVHNode? right;
  List<AABB>? leafs;

  int get depth {
    if (leafs != null) {
      return 1;
    }
    return 1 + max(left!.depth, right!.depth);
  }

  int get nodesCount {
    if (leafs != null) {
      return 1;
    }
    return 1 + left!.nodesCount + right!.nodesCount;
  }

  int get leafNodesCount {
    if (leafs != null) {
      return 1;
    }
    return left!.leafNodesCount + right!.leafNodesCount;
  }

  BVHNode(this.aabb, this.left, this.right, this.leafs);

  void draw(Canvas canvas) {
    if ((leafs == null || leafs!.isEmpty) && left == null && right == null) {
      return;
    }
    const fromColor = Color.fromARGB(255, 255, 0, 0);
    const toColor = Color.fromARGB(255, 0, 255, 0);
    final paint = Paint()
      ..color = fromColor
      ..style = PaintingStyle.stroke;
    _draw(canvas, paint, fromColor, toColor);
  }

  void _draw(Canvas canvas, Paint paint, Color fromColor, Color toColor,
      {int currentDepth = 0}) {
    paint.color = Color.lerp(
        fromColor, toColor, currentDepth.toDouble() / depth.toDouble())!;
    final rect = Rect.fromLTWH(aabb.x, aabb.y, aabb.width, aabb.height);
    canvas.drawRect(rect, paint);
    if (left != null) {
      left!._draw(canvas, paint, fromColor, toColor,
          currentDepth: currentDepth + 1);
    }
    if (right != null) {
      right!._draw(canvas, paint, fromColor, toColor,
          currentDepth: currentDepth + 1);
    }
  }
}
