#![feature(portable_simd)]
#![feature(asm_experimental_arch)]
#![feature(atomic_from_ptr)]

#[cfg(not(target_arch = "wasm32"))]
use mimalloc::MiMalloc;

#[cfg(not(target_arch = "wasm32"))]
#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

pub mod api;
pub mod bvh;
pub mod entity;
pub mod fast_list;
pub mod grid;
pub mod index;
pub mod matrix;
pub mod state;
pub mod thread;
pub mod time;
pub mod transform;
pub mod typed_data;
pub mod verlet;
