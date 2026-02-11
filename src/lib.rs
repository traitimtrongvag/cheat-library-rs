#![allow(non_snake_case, non_upper_case_globals)]

mod imgui_api;
mod remap;
mod includes;
mod il2cpp_sdk;
mod and64inlinehook;

use jni::JNIEnv;
use jni::objects::{JClass, JString};
use jni::sys::jstring;
use log::LevelFilter;
use android_logger::Config;

static mut IS_INITIALIZED: bool = false;

#[no_mangle]
pub extern "C" fn JNI_OnLoad(_vm: jni::JavaVM, _reserved: *mut std::ffi::c_void) -> jni::sys::jint {
    android_logger::init_once(
        Config::default()
            .with_max_level(LevelFilter::Debug)
            .with_tag("CheatLib")
    );

    log::info!("Cheat Library loaded");
    jni::sys::JNI_VERSION_1_6
}

#[no_mangle]
pub unsafe extern "C" fn Java_com_example_CheatLib_initialize(
    _env: JNIEnv,
    _class: JClass,
) {
    if IS_INITIALIZED {
        log::warn!("Library already initialized");
        return;
    }

    log::info!("Initializing cheat library");
    IS_INITIALIZED = true;
}

#[no_mangle]
pub unsafe extern "C" fn Java_com_example_CheatLib_getVersion(
    env: JNIEnv,
    _class: JClass,
) -> jstring {
    let version = env.new_string("1.0.0")
        .expect("Failed to create Java string");
    version.into_raw()
}

pub use imgui_api::*;
pub use remap::*;
pub use includes::*;
pub use il2cpp_sdk::*;
pub use and64inlinehook::*;
