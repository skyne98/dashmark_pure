import 'package:dashmark_pure/buffer.dart';
import 'package:dashmark_pure/typed_array/f64.dart';

import '../ffi_export.dart';

List<Vector2Buffer> transformedVertices() {
  var data = api.transformedVertices();
  var vertices = <Vector2Buffer>[];
  var buffer = ByteBufferDecoder(data);
  var length = buffer.readU64();

  for (var i = 0; i < length; i++) {
    var vertexCount = buffer.readU64();
    var vertexBuffer = Vector2Buffer();
    for (var j = 0; j < vertexCount; j++) {
      var x = buffer.readF64();
      var y = buffer.readF64();
      vertexBuffer.addXY(x, y);
    }
    vertices.add(vertexBuffer);
  }

  return vertices;
}
