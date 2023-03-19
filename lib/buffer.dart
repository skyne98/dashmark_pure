import 'dart:typed_data';

import 'package:flutter_rust_bridge/flutter_rust_bridge.dart';

/* Decoding */
class ByteBufferDecoder {
  final Uint8List _buffer;
  late ByteData _byteData;
  int _counter = 0;

  ByteBufferDecoder(this._buffer) {
    _byteData = ByteData.view(_buffer.buffer);
  }

  void reset() {
    _counter = 0;
  }

  int readUint64() {
    if (kIsWeb) {
      // Can read only the most important 32 bits on web
      if (Endian.host == Endian.little) {
        // Means last 32 bits are the most important
        final low = _byteData.getUint32(_counter, Endian.host);
        _counter += 8;
        return low;
      } else {
        // Means first 32 bits are the most important
        final high = _byteData.getUint32(_counter + 4, Endian.host);
        _counter += 8;
        return high;
      }
    } else {
      final value = _byteData.getUint64(_counter, Endian.host);
      _counter += 8;
      return value;
    }
  }

  int readUint128() {
    // Cannot read Uint128 at all, read as two Uint64 and combine
    if (Endian.host == Endian.little) {
      final low = readUint64();
      final high = readUint64();
      return low + (high << 64);
    } else {
      final high = readUint64();
      final low = readUint64();
      return low + (high << 64);
    }
  }

  int readUint8() {
    final value = _byteData.getUint8(_counter);
    _counter += 1;
    return value;
  }

  bool readBool() {
    final value = _byteData.getUint8(_counter);
    _counter += 1;
    return value == 1;
  }
}

/* Encoding */
