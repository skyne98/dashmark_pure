import '../ffi_export.dart';
import '../typed_buffer/mod.dart';

void setPriority(GenerationalIndex entity, int priority) {
  var indicesBuffer = GenerationalIndexBuffer();
  indicesBuffer.add(entity);
  var intBuffer = Int32Buffer();
  intBuffer.add(priority);
  api.entitiesSetPriorityRaw(
      indices: indicesBuffer.buffer.toUint32List(),
      priorities: intBuffer.toInt32List());
}

void setPrioritiesBulk(
    GenerationalIndexBuffer entities, Int32Buffer priorities) {
  api.entitiesSetPriorityRaw(
      indices: entities.buffer.toUint32List(),
      priorities: priorities.toInt32List());
}
