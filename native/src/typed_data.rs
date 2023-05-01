use generational_arena::Index;
use rapier2d::na::Vector2;

// Standard
pub fn bytes_to<T>(bytes: &[u8]) -> &[T] {
    let len = bytes.len();
    if len == 0 {
        return &[];
    }
    assert_eq!(len % std::mem::size_of::<T>(), 0);
    let ptr = bytes.as_ptr() as *const T;
    assert_eq!(ptr as usize % std::mem::align_of::<T>(), 0);
    assert_eq!(len % std::mem::size_of::<T>(), 0);
    let typed_len = len / std::mem::size_of::<T>();
    let data = unsafe { std::slice::from_raw_parts(ptr, typed_len) };
    data
}
pub fn bytes_to_value<T>(bytes: &[u8]) -> &T {
    let data = bytes_to(bytes);
    assert_eq!(data.len(), 1);
    &data[0]
}

pub fn to_bytes<T>(data: &[T]) -> &[u8] {
    let len = data.len();
    if len == 0 {
        return &[];
    }
    let ptr = data.as_ptr() as *const u8;
    assert_eq!(ptr as usize % std::mem::align_of::<T>(), 0);
    let typed_len = len * std::mem::size_of::<T>();
    let bytes = unsafe { std::slice::from_raw_parts(ptr, typed_len) };
    bytes
}
pub fn value_to_bytes<T>(value: &T) -> &[u8] {
    let slice = std::slice::from_ref(value);
    to_bytes(slice)
}

// Specific
pub fn bytes_to_indices(bytes: &[u8]) -> Vec<Index> {
    let data = bytes_to(bytes);
    let indices = data
        .chunks_exact(2)
        .map(|chunk| Index::from_raw_parts(chunk[0] as usize, chunk[1]))
        .collect::<Vec<_>>();
    indices
}

pub fn bytes_to_vector2s(bytes: &[u8]) -> Vec<Vector2<f32>> {
    let data = bytes_to(bytes);
    let vectors = data
        .chunks_exact(2)
        .map(|chunk| Vector2::new(chunk[0], chunk[1]))
        .collect::<Vec<_>>();
    vectors
}

pub fn indices_to_bytes(indices: &[Index]) -> Vec<u8> {
    let data = indices
        .iter()
        .map(|index| index.into_raw_parts())
        .flat_map(|(index, gen)| vec![index as u64, gen])
        .collect::<Vec<_>>();
    let bytes = to_bytes(&data);
    bytes.to_vec()
}

pub fn vector2s_to_bytes(vectors: &[Vector2<f32>]) -> Vec<u8> {
    let data = vectors
        .iter()
        .flat_map(|vector| vec![vector.x, vector.y])
        .collect::<Vec<_>>();
    let bytes = to_bytes(&data);
    bytes.to_vec()
}
