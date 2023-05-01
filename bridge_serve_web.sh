RUSTFLAGS="-O -C target-feature=+simd128" \
    dart run flutter_rust_bridge:serve "$@"