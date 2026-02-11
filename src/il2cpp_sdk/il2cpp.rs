use std::collections::HashMap;
use std::ffi::{CStr, CString};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::sync::Mutex;
use libc::dlsym;

#[derive(Clone, Copy)]
struct SyncPtr(*mut std::ffi::c_void);
unsafe impl Send for SyncPtr {}
unsafe impl Sync for SyncPtr {}

static CACHE_FIELDS: Mutex<Option<HashMap<String, usize>>> = Mutex::new(None);
static CACHE_METHODS: Mutex<Option<HashMap<String, SyncPtr>>> = Mutex::new(None);
static CACHE_CLASSES: Mutex<Option<HashMap<String, SyncPtr>>> = Mutex::new(None);

static mut IL2CPP_LIB_BASE: usize = 0;

static mut IL2CPP_ASSEMBLY_GET_IMAGE: Option<unsafe extern "C" fn(*const std::ffi::c_void) -> *const std::ffi::c_void> = None;
static mut IL2CPP_DOMAIN_GET: Option<unsafe extern "C" fn() -> *mut std::ffi::c_void> = None;
static mut IL2CPP_DOMAIN_GET_ASSEMBLIES: Option<unsafe extern "C" fn(*const std::ffi::c_void, *mut usize) -> *mut *mut std::ffi::c_void> = None;
static mut IL2CPP_IMAGE_GET_NAME: Option<unsafe extern "C" fn(*mut std::ffi::c_void) -> *const i8> = None;
static mut IL2CPP_CLASS_FROM_NAME: Option<unsafe extern "C" fn(*const std::ffi::c_void, *const i8, *const i8) -> *mut std::ffi::c_void> = None;
static mut IL2CPP_CLASS_GET_FIELD_FROM_NAME: Option<unsafe extern "C" fn(*mut std::ffi::c_void, *const i8) -> *mut std::ffi::c_void> = None;
static mut IL2CPP_CLASS_GET_METHOD_FROM_NAME: Option<unsafe extern "C" fn(*mut std::ffi::c_void, *const i8, i32) -> *mut std::ffi::c_void> = None;
static mut IL2CPP_FIELD_GET_OFFSET: Option<unsafe extern "C" fn(*mut std::ffi::c_void) -> usize> = None;
static mut IL2CPP_FIELD_STATIC_GET_VALUE: Option<unsafe extern "C" fn(*mut std::ffi::c_void, *mut std::ffi::c_void)> = None;
static mut IL2CPP_FIELD_STATIC_SET_VALUE: Option<unsafe extern "C" fn(*mut std::ffi::c_void, *mut std::ffi::c_void)> = None;
static mut IL2CPP_ARRAY_NEW: Option<unsafe extern "C" fn(*mut std::ffi::c_void, usize) -> *mut std::ffi::c_void> = None;
static mut IL2CPP_TYPE_GET_NAME: Option<unsafe extern "C" fn(*mut std::ffi::c_void) -> *mut i8> = None;
static mut IL2CPP_METHOD_GET_PARAM: Option<unsafe extern "C" fn(*mut std::ffi::c_void, u32) -> *mut std::ffi::c_void> = None;
static mut IL2CPP_CLASS_GET_METHODS: Option<unsafe extern "C" fn(*mut std::ffi::c_void, *mut *mut std::ffi::c_void) -> *mut std::ffi::c_void> = None;
static mut IL2CPP_METHOD_GET_NAME: Option<unsafe extern "C" fn(*mut std::ffi::c_void) -> *const i8> = None;
static mut IL2CPP_CLASS_GET_NAME: Option<unsafe extern "C" fn(*mut std::ffi::c_void) -> *const i8> = None;
static mut IL2CPP_CLASS_GET_NESTED_TYPES: Option<unsafe extern "C" fn(*mut std::ffi::c_void, *mut *mut std::ffi::c_void) -> *mut std::ffi::c_void> = None;
static mut IL2CPP_OBJECT_NEW: Option<unsafe extern "C" fn(*mut std::ffi::c_void) -> *mut std::ffi::c_void> = None;

fn get_export_function(lib: &str, name: &str) -> Option<*mut std::ffi::c_void> {
    unsafe {
        let lib_cstr = CString::new(lib).ok()?;
        let name_cstr = CString::new(name).ok()?;
        let handle = libc::dlopen(lib_cstr.as_ptr(), libc::RTLD_NOW);
        if handle.is_null() {
            return None;
        }
        let func = dlsym(handle, name_cstr.as_ptr());
        if func.is_null() {
            None
        } else {
            Some(func)
        }
    }
}

pub fn il2cpp_base() -> usize {
    unsafe {
        if IL2CPP_LIB_BASE != 0 {
            return IL2CPP_LIB_BASE;
        }
        if let Ok(file) = File::open("/proc/self/maps") {
            let reader = BufReader::new(file);
            for line in reader.lines().flatten() {
                if line.contains("libil2cpp.so") {
                    if let Some(addr_str) = line.split('-').next() {
                        if let Ok(addr) = usize::from_str_radix(addr_str, 16) {
                            IL2CPP_LIB_BASE = addr;
                            return addr;
                        }
                    }
                }
            }
        }
        0
    }
}

pub fn attach(libname: &str) -> Result<(), i32> {
    unsafe {
        *CACHE_FIELDS.lock().unwrap() = Some(HashMap::new());
        *CACHE_METHODS.lock().unwrap() = Some(HashMap::new());
        *CACHE_CLASSES.lock().unwrap() = Some(HashMap::new());

        IL2CPP_ASSEMBLY_GET_IMAGE = get_export_function(libname, "il2cpp_assembly_get_image")
            .map(|f| std::mem::transmute(f));
        IL2CPP_DOMAIN_GET = get_export_function(libname, "il2cpp_domain_get")
            .map(|f| std::mem::transmute(f));
        IL2CPP_DOMAIN_GET_ASSEMBLIES = get_export_function(libname, "il2cpp_domain_get_assemblies")
            .map(|f| std::mem::transmute(f));
        IL2CPP_IMAGE_GET_NAME = get_export_function(libname, "il2cpp_image_get_name")
            .map(|f| std::mem::transmute(f));
        IL2CPP_CLASS_FROM_NAME = get_export_function(libname, "il2cpp_class_from_name")
            .map(|f| std::mem::transmute(f));
        IL2CPP_CLASS_GET_FIELD_FROM_NAME = get_export_function(libname, "il2cpp_class_get_field_from_name")
            .map(|f| std::mem::transmute(f));
        IL2CPP_CLASS_GET_METHOD_FROM_NAME = get_export_function(libname, "il2cpp_class_get_method_from_name")
            .map(|f| std::mem::transmute(f));
        IL2CPP_FIELD_GET_OFFSET = get_export_function(libname, "il2cpp_field_get_offset")
            .map(|f| std::mem::transmute(f));
        IL2CPP_FIELD_STATIC_GET_VALUE = get_export_function(libname, "il2cpp_field_static_get_value")
            .map(|f| std::mem::transmute(f));
        IL2CPP_FIELD_STATIC_SET_VALUE = get_export_function(libname, "il2cpp_field_static_set_value")
            .map(|f| std::mem::transmute(f));
        IL2CPP_ARRAY_NEW = get_export_function(libname, "il2cpp_array_new")
            .map(|f| std::mem::transmute(f));
        IL2CPP_TYPE_GET_NAME = get_export_function(libname, "il2cpp_type_get_name")
            .map(|f| std::mem::transmute(f));
        IL2CPP_METHOD_GET_PARAM = get_export_function(libname, "il2cpp_method_get_param")
            .map(|f| std::mem::transmute(f));
        IL2CPP_CLASS_GET_METHODS = get_export_function(libname, "il2cpp_class_get_methods")
            .map(|f| std::mem::transmute(f));
        IL2CPP_METHOD_GET_NAME = get_export_function(libname, "il2cpp_method_get_name")
            .map(|f| std::mem::transmute(f));
        IL2CPP_CLASS_GET_NAME = get_export_function(libname, "il2cpp_class_get_name")
            .map(|f| std::mem::transmute(f));
        IL2CPP_CLASS_GET_NESTED_TYPES = get_export_function(libname, "il2cpp_class_get_nested_types")
            .map(|f| std::mem::transmute(f));
        IL2CPP_OBJECT_NEW = get_export_function(libname, "il2cpp_object_new")
            .map(|f| std::mem::transmute(f));

        if IL2CPP_DOMAIN_GET.is_none() || IL2CPP_CLASS_FROM_NAME.is_none() {
            return Err(-1);
        }
        Ok(())
    }
}

pub fn get_image(image_name: &str) -> Option<*mut std::ffi::c_void> {
    unsafe {
        let domain_get = IL2CPP_DOMAIN_GET?;
        let get_assemblies = IL2CPP_DOMAIN_GET_ASSEMBLIES?;
        let get_image_fn = IL2CPP_ASSEMBLY_GET_IMAGE?;
        let get_name = IL2CPP_IMAGE_GET_NAME?;

        let domain = domain_get();
        let mut size = 0usize;
        let assemblies = get_assemblies(domain, &mut size);

        for i in 0..size {
            let assembly = *assemblies.add(i);
            let img = get_image_fn(assembly) as *mut std::ffi::c_void;
            let name_ptr = get_name(img);
            if !name_ptr.is_null() {
                let name = CStr::from_ptr(name_ptr).to_str().ok()?;
                if name == image_name {
                    return Some(img);
                }
            }
        }
        None
    }
}

pub fn get_class(image_name: &str, namespace: &str, class_name: &str) -> Option<*mut std::ffi::c_void> {
    let sig = format!("{}{}{}", image_name, namespace, class_name);
    if let Some(cache) = CACHE_CLASSES.lock().unwrap().as_ref() {
        if let Some(&ptr) = cache.get(&sig) {
            return Some(ptr.0);
        }
    }

    unsafe {
        let img = get_image(image_name)?;
        let class_from_name = IL2CPP_CLASS_FROM_NAME?;

        let parts: Vec<&str> = class_name.split('.').collect();
        let namespace_cstr = CString::new(namespace).ok()?;
        let class_cstr = CString::new(parts[0]).ok()?;
        let mut klass = class_from_name(img, namespace_cstr.as_ptr() as *const i8, class_cstr.as_ptr() as *const i8);

        if parts.len() > 1 {
            let get_nested = IL2CPP_CLASS_GET_NESTED_TYPES?;
            let get_name = IL2CPP_CLASS_GET_NAME?;
            let mut iter = std::ptr::null_mut();
            let target_name = CString::new(parts[1]).ok()?;
            loop {
                let nested = get_nested(klass, &mut iter);
                if nested.is_null() {
                    break;
                }
                let name_ptr = get_name(nested);
                if !name_ptr.is_null() {
                    let name = CStr::from_ptr(name_ptr);
                    if name.to_bytes() == target_name.as_bytes() {
                        klass = nested;
                        break;
                    }
                }
            }
        }

        if !klass.is_null() {
            if let Some(cache) = CACHE_CLASSES.lock().unwrap().as_mut() {
                cache.insert(sig, SyncPtr(klass));
            }
            Some(klass)
        } else {
            None
        }
    }
}

pub fn get_field_offset(image_name: &str, namespace: &str, class_name: &str, field_name: &str) -> Option<usize> {
    let sig = format!("{}{}{}{}", image_name, namespace, class_name, field_name);
    if let Some(cache) = CACHE_FIELDS.lock().unwrap().as_ref() {
        if let Some(&offset) = cache.get(&sig) {
            return Some(offset);
        }
    }

    unsafe {
        let klass = get_class(image_name, namespace, class_name)?;
        let get_field = IL2CPP_CLASS_GET_FIELD_FROM_NAME?;
        let get_offset = IL2CPP_FIELD_GET_OFFSET?;

        let field_cstr = CString::new(field_name).ok()?;
        let field = get_field(klass, field_cstr.as_ptr() as *const i8);
        if field.is_null() {
            return None;
        }

        let offset = get_offset(field);
        if let Some(cache) = CACHE_FIELDS.lock().unwrap().as_mut() {
            cache.insert(sig, offset);
        }
        Some(offset)
    }
}

pub fn get_method_offset(image_name: &str, namespace: &str, class_name: &str, method_name: &str, args_count: i32) -> Option<*mut std::ffi::c_void> {
    let sig = format!("{}{}{}{}{}", image_name, namespace, class_name, method_name, args_count);
    if let Some(cache) = CACHE_METHODS.lock().unwrap().as_ref() {
        if let Some(&ptr) = cache.get(&sig) {
            return Some(ptr.0);
        }
    }

    unsafe {
        let klass = get_class(image_name, namespace, class_name)?;
        let get_method = IL2CPP_CLASS_GET_METHOD_FROM_NAME?;

        let method_cstr = CString::new(method_name).ok()?;
        let method_ptr = get_method(klass, method_cstr.as_ptr() as *const i8, args_count);
        if method_ptr.is_null() {
            return None;
        }

        let method = *(method_ptr as *const *mut std::ffi::c_void);
        if let Some(cache) = CACHE_METHODS.lock().unwrap().as_mut() {
            cache.insert(sig, SyncPtr(method));
        }
        Some(method)
    }
}

pub fn is_assemblies_loaded() -> bool {
    unsafe {
        if let Some(domain_get) = IL2CPP_DOMAIN_GET {
            if let Some(get_assemblies) = IL2CPP_DOMAIN_GET_ASSEMBLIES {
                let domain = domain_get();
                let mut size = 0usize;
                let assemblies = get_assemblies(domain, &mut size);
                return size != 0 && !assemblies.is_null();
            }
        }
        false
    }
}