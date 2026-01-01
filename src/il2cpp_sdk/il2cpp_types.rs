use std::ffi::c_void;

unsafe impl Send for *mut c_void {}
unsafe impl Sync for *mut c_void {}

#[repr(C)]
pub struct VirtualInvokeData {
    pub method_ptr: usize,
    pub method: *mut c_void,
}

#[repr(C)]
pub struct Il2CppType {
    pub data: *mut c_void,
    pub bits: u32,
}

#[repr(C)]
pub struct Il2CppClass {
    pub image: *mut c_void,
    pub gc_desc: *mut c_void,
    pub name: *const i8,
    pub namespaze: *const i8,
    pub byval_arg: *mut Il2CppType,
    pub this_arg: *mut Il2CppType,
    pub element_class: *mut Il2CppClass,
    pub cast_class: *mut Il2CppClass,
    pub declaring_type: *mut Il2CppClass,
    pub parent: *mut Il2CppClass,
    pub generic_class: *mut c_void,
    pub type_definition: *mut c_void,
    pub interop_data: *mut c_void,
    pub fields: *mut c_void,
    pub events: *mut c_void,
    pub properties: *mut c_void,
    pub methods: *mut c_void,
    pub nested_types: *mut *mut Il2CppClass,
    pub implemented_interfaces: *mut *mut Il2CppClass,
    pub interface_offsets: *mut c_void,
    pub static_fields: *mut c_void,
    pub rgctx_data: *mut c_void,
    pub type_hierarchy: *mut *mut Il2CppClass,
    pub cctor_started: u32,
    pub cctor_finished: u32,
    pub cctor_thread: u64,
    pub generic_container_index: i32,
    pub custom_attribute_index: i32,
    pub instance_size: u32,
    pub actual_size: u32,
    pub element_size: u32,
    pub native_size: i32,
    pub static_fields_size: u32,
    pub thread_static_fields_size: u32,
    pub thread_static_fields_offset: i32,
    pub flags: u32,
    pub token: u32,
    pub method_count: u16,
    pub property_count: u16,
    pub field_count: u16,
    pub event_count: u16,
    pub nested_type_count: u16,
    pub vtable_count: u16,
    pub interfaces_count: u16,
    pub interface_offsets_count: u16,
    pub type_hierarchy_depth: u8,
    pub generic_recursion_depth: u8,
    pub rank: u8,
    pub minimum_alignment: u8,
    pub packing_size: u8,
    pub bitflags1: u8,
    pub bitflags2: u8,
    pub vtable: [VirtualInvokeData; 255],
}

#[repr(C)]
pub struct Il2CppObject {
    pub klass: *mut Il2CppClass,
    pub monitor: *mut c_void,
}

#[repr(C)]
pub struct Il2CppArray<T> {
    pub klass: *mut Il2CppClass,
    pub monitor: *mut c_void,
    pub bounds: *mut c_void,
    pub max_length: i32,
    pub items: [T; 0],
}

impl<T> Il2CppArray<T> {
    pub fn length(&self) -> i32 {
        self.max_length
    }

    pub fn get_pointer(&self) -> *const T {
        self.items.as_ptr()
    }

    pub fn get_pointer_mut(&mut self) -> *mut T {
        self.items.as_mut_ptr()
    }
}

#[repr(C)]
pub struct Il2CppString {
    pub klass: *mut Il2CppClass,
    pub monitor: *mut c_void,
    pub length: i32,
    pub start_char: u16,
}

impl Il2CppString {
    pub fn to_string(&self) -> String {
        if self.length <= 0 {
            return String::new();
        }
        unsafe {
            let ptr = &self.start_char as *const u16;
            let slice = std::slice::from_raw_parts(ptr, self.length as usize);
            String::from_utf16_lossy(slice)
        }
    }

    pub fn get_chars(&self) -> *const u16 {
        &self.start_char as *const u16
    }

    pub fn get_length(&self) -> i32 {
        self.length
    }
}

#[repr(C)]
pub struct Il2CppList<T> {
    pub klass: *mut Il2CppClass,
    pub unk1: *mut c_void,
    pub items: *mut Il2CppArray<T>,
    pub size: i32,
    pub version: i32,
}

impl<T> Il2CppList<T> {
    pub fn get_items(&self) -> *const T {
        unsafe {
            if self.items.is_null() {
                std::ptr::null()
            } else {
                (*self.items).get_pointer()
            }
        }
    }

    pub fn get_size(&self) -> i32 {
        self.size
    }

    pub fn get_version(&self) -> i32 {
        self.version
    }
}

#[repr(C)]
pub struct Il2CppDictionary<K, V> {
    pub klass: *mut Il2CppClass,
    pub unk1: *mut c_void,
    pub table: *mut Il2CppArray<*mut *mut i32>,
    pub link_slots: *mut Il2CppArray<*mut *mut c_void>,
    pub keys: *mut Il2CppArray<K>,
    pub values: *mut Il2CppArray<V>,
    pub touched_slots: i32,
    pub empty_slot: i32,
    pub size: i32,
}

impl<K, V> Il2CppDictionary<K, V> {
    pub fn get_keys(&self) -> *const K {
        unsafe {
            if self.keys.is_null() {
                std::ptr::null()
            } else {
                (*self.keys).get_pointer()
            }
        }
    }

    pub fn get_values(&self) -> *const V {
        unsafe {
            if self.values.is_null() {
                std::ptr::null()
            } else {
                (*self.values).get_pointer()
            }
        }
    }

    pub fn get_num_keys(&self) -> i32 {
        unsafe {
            if self.keys.is_null() {
                0
            } else {
                (*self.keys).length()
            }
        }
    }

    pub fn get_num_values(&self) -> i32 {
        unsafe {
            if self.values.is_null() {
                0
            } else {
                (*self.values).length()
            }
        }
    }

    pub fn get_size(&self) -> i32 {
        self.size
    }
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct Il2CppColor {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

impl Il2CppColor {
    pub fn black() -> Self {
        Self { r: 0.0, g: 0.0, b: 0.0, a: 1.0 }
    }
    pub fn blue() -> Self {
        Self { r: 0.0, g: 0.0, b: 1.0, a: 1.0 }
    }
    pub fn cyan() -> Self {
        Self { r: 0.0, g: 1.0, b: 1.0, a: 1.0 }
    }
    pub fn green() -> Self {
        Self { r: 0.0, g: 1.0, b: 0.0, a: 1.0 }
    }
    pub fn orange() -> Self {
        Self { r: 1.0, g: 0.5, b: 0.0, a: 1.0 }
    }
    pub fn red() -> Self {
        Self { r: 1.0, g: 0.0, b: 0.0, a: 1.0 }
    }
    pub fn white() -> Self {
        Self { r: 1.0, g: 1.0, b: 1.0, a: 1.0 }
    }
    pub fn gray() -> Self {
        Self { r: 0.5, g: 0.5, b: 0.5, a: 1.0 }
    }
    pub fn yellow() -> Self {
        Self { r: 1.0, g: 0.921568632, b: 0.0156862754, a: 1.0 }
    }
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct Il2CppVector2 {
    pub x: f32,
    pub y: f32,
}

impl Il2CppVector2 {
    pub fn zero() -> Self {
        Self { x: 0.0, y: 0.0 }
    }
    pub fn up() -> Self {
        Self { x: 0.0, y: 1.0 }
    }
    pub fn down() -> Self {
        Self { x: 0.0, y: -1.0 }
    }
    pub fn right() -> Self {
        Self { x: 1.0, y: 0.0 }
    }
    pub fn left() -> Self {
        Self { x: -1.0, y: 0.0 }
    }

    pub fn distance(a: Self, b: Self) -> f32 {
        let dx = a.x - b.x;
        let dy = a.y - b.y;
        (dx * dx + dy * dy).sqrt()
    }
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct Il2CppVector3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Il2CppVector3 {
    pub fn zero() -> Self {
        Self { x: 0.0, y: 0.0, z: 0.0 }
    }
    pub fn up() -> Self {
        Self { x: 0.0, y: 1.0, z: 0.0 }
    }
    pub fn down() -> Self {
        Self { x: 0.0, y: -1.0, z: 0.0 }
    }
    pub fn right() -> Self {
        Self { x: 1.0, y: 0.0, z: 0.0 }
    }
    pub fn left() -> Self {
        Self { x: -1.0, y: 0.0, z: 0.0 }
    }
    pub fn forward() -> Self {
        Self { x: 0.0, y: 0.0, z: 1.0 }
    }
    pub fn back() -> Self {
        Self { x: 0.0, y: 0.0, z: -1.0 }
    }

    pub fn distance(a: Self, b: Self) -> f32 {
        let dx = a.x - b.x;
        let dy = a.y - b.y;
        let dz = a.z - b.z;
        (dx * dx + dy * dy + dz * dz).sqrt()
    }

    pub fn magnitude(vector: Self) -> f32 {
        (vector.x * vector.x + vector.y * vector.y + vector.z * vector.z).sqrt()
    }

    pub fn normalize(value: Self) -> Self {
        let num = Self::magnitude(value);
        if num > 1e-5 {
            Self {
                x: value.x / num,
                y: value.y / num,
                z: value.z / num,
            }
        } else {
            Self::zero()
        }
    }
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct Il2CppQuaternion {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

impl Il2CppQuaternion {
    pub fn dot(a: Self, b: Self) -> f32 {
        a.x * b.x + a.y * b.y + a.z * b.z + a.w * b.w
    }
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct Il2CppRect {
    pub m_x_min: f32,
    pub m_y_min: f32,
    pub m_width: f32,
    pub m_height: f32,
}

impl Il2CppRect {
    pub fn new(x: f32, y: f32, width: f32, height: f32) -> Self {
        Self {
            m_x_min: x,
            m_y_min: y,
            m_width: width,
            m_height: height,
        }
    }
}