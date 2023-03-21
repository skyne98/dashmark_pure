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

  // Single-value read methods
  bool readBool() {
    final value = _byteData.getUint8(_counter);
    _counter += 1;
    return value == 1;
  }

  int readU8() {
    final value = _byteData.getUint8(_counter);
    _counter += 1;
    return value;
  }

  int readI8() {
    final value = _byteData.getInt8(_counter);
    _counter += 1;
    return value;
  }

  int readU64() {
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

  int readyU128() {
    final value = _byteData.getUint64(_counter, Endian.host);
    _counter += 8;
    return value;
  }

  double readF32() {
    final value = _byteData.getFloat32(_counter, Endian.host);
    _counter += 4;
    return value;
  }

  double readF64() {
    final value = _byteData.getFloat64(_counter, Endian.host);
    _counter += 8;
    return value;
  }

  // Typed array read methods
  Uint8List readU8Array() {
    final length = readU64();
    final array = Uint8List.view(_buffer.buffer, _counter, length);
    _counter += length;
    return array;
  }

  Float64List readF64Array() {
    final length = readU64();
    final array = Float64List.view(_buffer.buffer, _counter, length);
    _counter += length * 8;
    return array;
  }
}

/* Encoding */
