use generational_arena::Index;
use rapier2d::na::Vector2;

// Standard
pub fn from_to<'a, F, T>(from: &'a [F]) -> &'a [T] {
    let from_len = from.len();
    if from_len == 0 {
        return &[];
    }
    let ratio = std::mem::size_of::<F>() as f32 / std::mem::size_of::<T>() as f32;
    let len = (from_len as f32 * ratio) as usize;
    let from_ptr = from.as_ptr() as *const T;
    unsafe { std::slice::from_raw_parts(from_ptr, len) }
}
pub fn vec_from_to<'a, F, T>(from: &'a Vec<F>) -> &'a [T] {
    let (head, body, tail) = unsafe { from.align_to::<T>() };
    assert!(head.is_empty());
    assert!(tail.is_empty());
    body
}

// Specific
pub fn u32s_to_indices(u32s: &[u32]) -> Vec<Index> {
    let data = u32s
        .chunks_exact(2)
        .map(|chunk| Index::from_raw_parts(chunk[0] as usize, chunk[1] as u64))
        .collect::<Vec<_>>();
    data
}
pub fn indices_to_u32s(indices: &[Index]) -> Vec<u32> {
    let data = indices
        .iter()
        .map(|index| index.into_raw_parts())
        .flat_map(|(index, gen)| vec![index as u32, gen as u32])
        .collect::<Vec<_>>();
    data
}

pub fn f32s_to_vec2s(f32s: &[f32]) -> Vec<Vector2<f32>> {
    let data = f32s
        .chunks_exact(2)
        .map(|chunk| Vector2::new(chunk[0], chunk[1]))
        .collect::<Vec<_>>();
    data
}
pub fn vec2s_to_f32s(vec2s: &[Vector2<f32>]) -> Vec<f32> {
    let data = vec2s
        .iter()
        .flat_map(|vec2| vec![vec2.x, vec2.y])
        .collect::<Vec<_>>();
    data
}

pub fn f32s_to_vec2_arrays(f32s: &[f32]) -> &[[f32; 2]] {
    let data =
        unsafe { std::slice::from_raw_parts(f32s.as_ptr() as *const [f32; 2], f32s.len() / 2) };
    data
}
pub fn vec2_arrays_to_f32s(vec2_arrays: &[[f32; 2]]) -> &[f32] {
    let data = unsafe {
        std::slice::from_raw_parts(vec2_arrays.as_ptr() as *const f32, vec2_arrays.len() * 2)
    };
    data
}
