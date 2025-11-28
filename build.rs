use std::env;
use std::path::PathBuf;
use std::process::Command;

fn main() {
    println!("cargo:rerun-if-changed=src/includes/dobby/libraries/arm64-v8a/");
    println!("cargo:rustc-link-search=native=src/includes/dobby/libraries/arm64-v8a");
    println!("cargo:rustc-link-lib=dylib=dobby_rs");
    println!("cargo:rustc-link-lib=dylib=dobby");

    let required_files = [
        "src/ImGui/imgui_wrapper.h",
        "src/ImGui/imgui_wrapper.cpp", 
        "src/ImGui/imgui.cpp",
        "src/ImGui/imgui_draw.cpp",
        "src/ImGui/imgui_tables.cpp",
        "src/ImGui/imgui_widgets.cpp",
        "src/ImGui/backends/imgui_impl_android.cpp",
        "src/ImGui/backends/imgui_impl_opengl3.cpp",
    ];

    for file in &required_files {
        if !std::path::Path::new(file).exists() {
            panic!("Missing required file: {}", file);
        }
        println!("cargo:rerun-if-changed={}", file);
    }

    let mut build = cc::Build::new();
    build
        .cpp(true)
        .flag("-std=c++17")
        .flag("-fPIC")
        .flag("-fno-rtti")
        .flag("-fno-exceptions")
        .flag("-Wno-nontrivial-memcall")
        .flag("-Wno-deprecated-declarations")
        .flag("-Wno-unused-variable")
        .flag("-Wno-unused-parameter")
        .flag("-Wno-sign-compare")
        .include("src/ImGui")
        .include("src/ImGui/backends")
        .file("src/ImGui/imgui.cpp")
        .file("src/ImGui/imgui_draw.cpp")
        .file("src/ImGui/imgui_tables.cpp")
        .file("src/ImGui/imgui_widgets.cpp")
        .file("src/ImGui/backends/imgui_impl_android.cpp")
        .file("src/ImGui/backends/imgui_impl_opengl3.cpp")
        .file("src/ImGui/imgui_wrapper.cpp"); 

    if std::path::Path::new("src/ImGui/thirdparty").exists() {
        build.include("src/ImGui/thirdparty");
        if std::path::Path::new("src/ImGui/thirdparty/GLES3").exists() {
            build.include("src/ImGui/thirdparty/GLES3");
        }
    }

    build.target("aarch64-linux-android");
    
    build.compile("imgui_android");

    let out_dir = env::var("OUT_DIR").unwrap();
    println!("cargo:rustc-link-search=native={}", out_dir);
    println!("cargo:rustc-link-lib=static=imgui_android");

    let out_path = PathBuf::from(&out_dir);
    bindgen::Builder::default()
        .header("src/ImGui/imgui_wrapper.h")
        .clang_arg("-Isrc/ImGui")
        .clang_arg("-Isrc/ImGui/backends")
        .clang_arg("-x")
        .clang_arg("c++")
        .clang_arg("-std=c++17")
        .generate()
        .expect("Unable to generate ImGui bindings")
        .write_to_file(out_path.join("imgui_bindings.rs"))
        .expect("Couldn't write bindings!");

    println!("cargo:rustc-link-lib=GLESv3");
    println!("cargo:rustc-link-lib=EGL");
    println!("cargo:rustc-link-lib=android");
    println!("cargo:rustc-link-lib=log");
    println!("cargo:rustc-link-lib=c++_shared");
}