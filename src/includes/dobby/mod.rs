#![allow(non_camel_case_types, non_snake_case)]

use std::os::raw::{c_char, c_int, c_void};
use std::ptr;

pub type addr_t = usize;
pub type addr32_t = u32;
pub type addr64_t = u64;
pub type dobby_dummy_func_t = *mut c_void;
pub type asm_func_t = *mut c_void;

#[repr(C)]
pub struct DobbyRegisterContext {
    pub sp: u64,
    pub general: [u64; 29],
    pub fp: u64,
    pub lr: u64,
    pub floating: [FPReg; 32],
}

#[repr(C)]
pub union FPReg {
    pub q: u128,
    pub d: [f64; 2],
    pub f: [f32; 4],
}

pub const RT_FAILED: i32 = -1;
pub const RT_SUCCESS: i32 = 0;
pub type RetStatus = i32;

#[link(name = "dobby_rs")]
unsafe extern "C" {
    pub fn DobbyHook(
        address: *mut c_void,
        replace_func: dobby_dummy_func_t,
        origin_func: *mut dobby_dummy_func_t,
    ) -> c_int;

    pub fn DobbySymbolResolver(image_name: *const c_char, symbol_name: *const c_char) -> *mut c_void;

    pub fn DobbyInstrument(
        address: *mut c_void,
        pre_handler: Option<extern "C" fn(*mut c_void, *mut DobbyRegisterContext)>,
    ) -> c_int;

    pub fn DobbyDestroy(address: *mut c_void) -> c_int;

    pub fn DobbyGetVersion() -> *const c_char;

    pub fn DobbyImportTableReplace(
        image_name: *mut c_char,
        symbol_name: *mut c_char,
        fake_func: dobby_dummy_func_t,
        orig_func: *mut dobby_dummy_func_t,
    ) -> c_int;

    pub fn dobby_enable_near_branch_trampoline();
    pub fn dobby_disable_near_branch_trampoline();
}
/*
use crate::includes::dobby::*;

fn example_hook() {
    unsafe {
        let addr = DobbySymbolResolver(
            b"/system/lib64/libandroid.so\0".as_ptr() as *const _,
            b"ANativeWindow_getWidth\0".as_ptr() as *const _,
        );

        let mut orig: dobby_dummy_func_t = std::ptr::null_mut();
        DobbyHook(addr, my_hook as dobby_dummy_func_t, &mut orig);
    }
}

extern "C" fn my_hook() {
    println!("Hook called!");
}*/