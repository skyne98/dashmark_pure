import 'package:flutter_rust_bridge/flutter_rust_bridge.dart';

import 'buffer.dart';

class FlatBvh {
  final Float64List minX;
  final Float64List minY;
  final Float64List maxX;
  final Float64List maxY;
  final Uint8List depth;
  final Uint8List isLeafs;

  FlatBvh(this.minX, this.minY, this.maxX, this.maxY, this.depth, this.isLeafs);

  factory FlatBvh.fromBytes(Uint8List bytes) {
    final decoder = ByteBufferDecoder(bytes);
    final minxs = decoder.readF64Array();
    final minys = decoder.readF64Array();
    final maxxs = decoder.readF64Array();
    final maxys = decoder.readF64Array();
    final depths = decoder.readU8Array();
    final isLeafs = decoder.readU8Array();
    return FlatBvh(minxs, minys, maxxs, maxys, depths, isLeafs);
  }
}
