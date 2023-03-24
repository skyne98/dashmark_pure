use std::cell::RefCell;

pub mod api;
mod bridge_generated; /* AUTO INJECTED BY flutter_rust_bridge. This line may not be accurate, and you can change it according to your needs. */
pub mod bvh;
pub mod entity;
pub mod index;
pub mod render;
pub mod state;
pub mod typed_data;

use anyhow::{anyhow, Result};

/// Expose the JNI interface for android below
#[cfg(target_os = "android")]
#[allow(non_snake_case)]
pub mod android {
    extern crate jni;

    use log::info;
    use ndk::surface_texture::SurfaceTexture;
    use ndk::{native_window, trace};
    use raw_window_handle::{
        AndroidDisplayHandle, HasRawDisplayHandle, HasRawWindowHandle, RawDisplayHandle,
        RawWindowHandle,
    };

    use crate::render::init_wgpu;

    use self::jni::objects::{JClass, JObject, JString};
    use self::jni::sys::jstring;
    use self::jni::JNIEnv;
    use super::*;

    struct NativeWindowWrapper {
        pub native_window: ndk::native_window::NativeWindow,
    }

    impl NativeWindowWrapper {
        fn new(native_window: ndk::native_window::NativeWindow) -> Self {
            Self { native_window }
        }
    }

    unsafe impl HasRawDisplayHandle for NativeWindowWrapper {
        fn raw_display_handle(&self) -> RawDisplayHandle {
            raw_window_handle::RawDisplayHandle::Android(AndroidDisplayHandle::empty())
        }
    }

    unsafe impl HasRawWindowHandle for NativeWindowWrapper {
        fn raw_window_handle(&self) -> RawWindowHandle {
            self.native_window.raw_window_handle()
        }
    }

    #[no_mangle]
    pub unsafe extern "C" fn Java_com_example_dashmark_1pure_RustBridge_nativeInitTexture(
        env: JNIEnv,
        _: JClass,
        surface_texture: JObject,
    ) {
        android_logger::init_once(
            android_logger::Config::default().with_max_level(log::LevelFilter::Debug),
        );
        let _trace;
        if trace::is_trace_enabled() {
            _trace = trace::Section::new("dashmark test").unwrap();
        }

        info!("Hello from Rust in initTexture!");

        let surface_texture = SurfaceTexture::from_surface_texture(
            env.get_native_interface(),
            surface_texture.as_raw(),
        )
        .expect("Failed to convert Java SurfaceTexture to Rust SurfaceTexture");

        let native_window = surface_texture
            .acquire_native_window()
            .expect("Failed to acquire native window");

        init_wgpu(NativeWindowWrapper::new(native_window)).expect("Failed to init wgpu");
    }
}
