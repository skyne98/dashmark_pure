CPATH=/usr/lib/clang/15.0.7/include flutter_rust_bridge_codegen \
    --rust-input native/src/api/api.rs \
    --dart-output lib/api/bridge_generated.dart \
    --wasm --skip-add-mod-to-lib