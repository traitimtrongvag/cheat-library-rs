#![allow(non_snake_case, dead_code)]

use std::ffi::CString;
use std::os::raw::{c_char, c_float, c_int};

include!(concat!(env!("OUT_DIR"), "/imgui_bindings.rs"));

pub const WINDOW_NO_TITLE_BAR: i32 = 1 << 0;
pub const WINDOW_NO_RESIZE: i32 = 1 << 1;
pub const WINDOW_NO_MOVE: i32 = 1 << 2;
pub const WINDOW_NO_BACKGROUND: i32 = 1 << 9;
pub const WINDOW_ALWAYS_AUTO_RESIZE: i32 = 1 << 6;

pub fn setup_imgui(width: i32, height: i32) {
    unsafe {
        ImGui_CreateContext();
        DrawImGuiStyle_Wrapper();
        
        let glsl_version = CString::new("#version 300 es").unwrap();
        ImGui_InitOpenGL3(glsl_version.as_ptr());
        
        log::info!("ImGui setup complete at {}x{}", width, height);
    }
}

pub fn imgui_new_frame(width: i32, height: i32) {
    unsafe {
        ImGui_OpenGL3NewFrame();
        ImGui_AndroidNewFrame(width, height);
        ImGui_NewFrame();
    }
}

pub fn imgui_render() {
    unsafe {
        ImGui_Render();
        ImGui_EndFrame();
        ImGui_RenderDrawData();
    }
}

pub fn text(s: &str) {
    let c_str = CString::new(s).unwrap();
    unsafe { ImGui_Text(c_str.as_ptr()) }
}

pub fn button(label: &str) -> bool {
    let c_str = CString::new(label).unwrap();
    unsafe { ImGui_Button(c_str.as_ptr(), 0.0, 0.0) }
}

pub fn button_size(label: &str, width: f32, height: f32) -> bool {
    let c_str = CString::new(label).unwrap();
    unsafe { ImGui_Button(c_str.as_ptr(), width, height) }
}

pub fn checkbox(label: &str, v: &mut bool) -> bool {
    let c_str = CString::new(label).unwrap();
    unsafe { ImGui_Checkbox(c_str.as_ptr(), v as *mut bool) }
}

pub fn slider_int(label: &str, v: &mut i32, min: i32, max: i32) -> bool {
    let c_str = CString::new(label).unwrap();
    unsafe { ImGui_SliderInt(c_str.as_ptr(), v as *mut c_int, min, max) }
}

pub fn slider_float(label: &str, v: &mut f32, min: f32, max: f32) -> bool {
    let c_str = CString::new(label).unwrap();
    unsafe { ImGui_SliderFloat(c_str.as_ptr(), v as *mut c_float, min, max) }
}

pub fn separator() {
    unsafe { ImGui_Separator() }
}

pub fn same_line() {
    unsafe { ImGui_SameLine() }
}

pub fn spacing() {
    unsafe { ImGui_Spacing() }
}

pub fn begin_window(name: &str, p_open: Option<&mut bool>, flags: i32) -> bool {
    let c_str = CString::new(name).unwrap();
    unsafe {
        match p_open {
            Some(p) => ImGui_Begin(c_str.as_ptr(), p as *mut bool, flags),
            None => ImGui_Begin(c_str.as_ptr(), std::ptr::null_mut(), flags),
        }
    }
}

pub fn end_window() {
    unsafe { ImGui_End() }
}

pub fn begin_child(str_id: &str, width: f32, height: f32, border: bool, flags: i32) -> bool {
    let c_str = CString::new(str_id).unwrap();
    unsafe { ImGui_BeginChild(c_str.as_ptr(), width, height, border, flags) }
}

pub fn end_child() {
    unsafe { ImGui_EndChild() }
}

pub fn open_popup(str_id: &str) {
    let c_str = CString::new(str_id).unwrap();
    unsafe { ImGui_OpenPopup(c_str.as_ptr()) }
}

pub fn begin_popup_modal(name: &str, p_open: Option<&mut bool>, flags: i32) -> bool {
    let c_str = CString::new(name).unwrap();
    unsafe {
        match p_open {
            Some(p) => ImGui_BeginPopupModal(c_str.as_ptr(), p as *mut bool, flags),
            None => ImGui_BeginPopupModal(c_str.as_ptr(), std::ptr::null_mut(), flags),
        }
    }
}

pub fn end_popup() {
    unsafe { ImGui_EndPopup() }
}

pub fn push_style_color(idx: i32, r: f32, g: f32, b: f32, a: f32) {
    unsafe { ImGui_PushStyleColor(idx, r, g, b, a) }
}

pub fn pop_style_color(count: i32) {
    unsafe { ImGui_PopStyleColor(count) }
}

pub fn push_style_var(idx: i32, val: f32) {
    unsafe { ImGui_PushStyleVar(idx, val) }
}

pub fn pop_style_var(count: i32) {
    unsafe { ImGui_PopStyleVar(count) }
}

pub fn push_item_width(width: f32) {
    unsafe { ImGui_PushItemWidth(width) }
}

pub fn pop_item_width() {
    unsafe { ImGui_PopItemWidth() }
}

pub fn input_text(label: &str, buf: &mut [u8], flags: i32) -> bool {
    let c_str = CString::new(label).unwrap();
    unsafe {
        ImGui_InputText(
            c_str.as_ptr(),
            buf.as_mut_ptr() as *mut c_char,
            buf.len(),
            flags,
        )
    }
}

pub fn columns(count: i32, id: Option<&str>, border: bool) {
    let c_str = id.map(|s| CString::new(s).unwrap());
    unsafe {
        ImGui_Columns(
            count,
            c_str.as_ref().map_or(std::ptr::null(), |s| s.as_ptr()),
            border,
        )
    }
}

pub fn next_column() {
    unsafe { ImGui_NextColumn() }
}

pub fn set_column_offset(column_index: i32, offset: f32) {
    unsafe { ImGui_SetColumnOffset(column_index, offset) }
}

pub fn begin_table(str_id: &str, column: i32, flags: i32) -> bool {
    let c_str = CString::new(str_id).unwrap();
    unsafe { ImGui_BeginTable(c_str.as_ptr(), column, flags) }
}

pub fn end_table() {
    unsafe { ImGui_EndTable() }
}

pub fn table_next_column() {
    unsafe { ImGui_TableNextColumn() }
}

pub fn get_framerate() -> f32 {
    unsafe { ImGui_GetFramerate() }
}