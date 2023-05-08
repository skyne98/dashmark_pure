#![feature(portable_simd)]
#![feature(asm_experimental_arch)]
#![feature(atomic_mut_ptr)]
#![feature(atomic_from_ptr)]

// use mimalloc::MiMalloc;

// #[global_allocator]
// static GLOBAL: MiMalloc = MiMalloc;

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
