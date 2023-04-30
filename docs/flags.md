## WASM
`RUSTFLAGS="-O -C target-feature=+simd128,+atomics,+bulk-memory,+mutable-globals,+nontrapping-float-to, +sign-ext"`

## X86
`RUSTFLAGS="-O -C target-cpu=native -C target-feature=+ssse3,+sse4.1,+sse4.2,+avx"`