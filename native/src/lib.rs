#![feature(portable_simd)]

// use mimalloc::MiMalloc;

// #[global_allocator]
// static GLOBAL: MiMalloc = MiMalloc;

pub mod api;
pub mod bvh;
pub mod entity;
pub mod index;
pub mod matrix;
pub mod state;
pub mod time;
pub mod transform;
pub mod typed_data;
pub mod verlet;
