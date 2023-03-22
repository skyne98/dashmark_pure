use generational_arena::Index;
use rapier2d_f64::na::Vector2;

// Standard
pub fn bytes_to_f64s(bytes: &[u8]) -> &[f64] {
    let len = bytes.len();
    let typed_len = len / std::mem::size_of::<f64>();
    let data = unsafe { std::slice::from_raw_parts(bytes.as_ptr() as *const f64, typed_len) };
    data
}

pub fn bytes_to_f32s(bytes: &[u8]) -> &[f32] {
    let len = bytes.len();
    let typed_len = len / std::mem::size_of::<f32>();
    let data = unsafe { std::slice::from_raw_parts(bytes.as_ptr() as *const f32, typed_len) };
    data
}

pub fn bytes_to_u64s(bytes: &[u8]) -> &[u64] {
    let len = bytes.len();
    let typed_len = len / std::mem::size_of::<u64>();
    let data = unsafe { std::slice::from_raw_parts(bytes.as_ptr() as *const u64, typed_len) };
    data
}

pub fn bytes_to_u32s(bytes: &[u8]) -> &[u32] {
    let len = bytes.len();
    let typed_len = len / std::mem::size_of::<u32>();
    let data = unsafe { std::slice::from_raw_parts(bytes.as_ptr() as *const u32, typed_len) };
    data
}

pub fn bytes_to_u16s(bytes: &[u8]) -> &[u16] {
    let len = bytes.len();
    let typed_len = len / std::mem::size_of::<u16>();
    let data = unsafe { std::slice::from_raw_parts(bytes.as_ptr() as *const u16, typed_len) };
    data
}

pub fn bytes_to_i64s(bytes: &[u8]) -> &[i64] {
    let len = bytes.len();
    let typed_len = len / std::mem::size_of::<i64>();
    let data = unsafe { std::slice::from_raw_parts(bytes.as_ptr() as *const i64, typed_len) };
    data
}

pub fn bytes_to_i32s(bytes: &[u8]) -> &[i32] {
    let len = bytes.len();
    let typed_len = len / std::mem::size_of::<i32>();
    let data = unsafe { std::slice::from_raw_parts(bytes.as_ptr() as *const i32, typed_len) };
    data
}

pub fn bytes_to_i16s(bytes: &[u8]) -> &[i16] {
    let len = bytes.len();
    let typed_len = len / std::mem::size_of::<i16>();
    let data = unsafe { std::slice::from_raw_parts(bytes.as_ptr() as *const i16, typed_len) };
    data
}

// Specific
pub fn bytes_to_indices(bytes: &[u8]) -> Vec<Index> {
    let data = bytes_to_u64s(bytes);
    let indices = data
        .chunks_exact(2)
        .map(|chunk| Index::from_raw_parts(chunk[0] as usize, chunk[1]))
        .collect::<Vec<_>>();
    indices
}

pub fn bytes_to_vector2s(bytes: &[u8]) -> Vec<Vector2<f64>> {
    let data = bytes_to_f64s(bytes);
    let vectors = data
        .chunks_exact(2)
        .map(|chunk| Vector2::new(chunk[0], chunk[1]))
        .collect::<Vec<_>>();
    vectors
}
