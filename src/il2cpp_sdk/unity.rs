use crate::il2cpp_sdk::vector3::Vector3;
use crate::il2cpp_sdk::quaternion::Quaternion;
use std::ffi::c_void;

#[repr(C)]
pub struct MonoString {
    pub klass: *mut c_void,
    pub monitor: *mut c_void,
    pub length: i32,
    pub chars: [u16; 0],
}

impl MonoString {
    pub fn get_string(&self) -> String {
        if self.length <= 0 {
            return String::new();
        }
        unsafe {
            let ptr = self.chars.as_ptr();
            let slice = std::slice::from_raw_parts(ptr, self.length as usize);
            String::from_utf16_lossy(slice)
        }
    }

    pub fn set(&mut self, s: &str) {
        let utf16: Vec<u16> = s.encode_utf16().collect();
        self.length = utf16.len() as i32;
        unsafe {
            let chars_ptr = self.chars.as_mut_ptr();
            std::ptr::copy_nonoverlapping(utf16.as_ptr(), chars_ptr, utf16.len());
        }
    }

    pub fn get_chars(&self) -> *const u16 {
        self.chars.as_ptr()
    }

    pub fn get_length(&self) -> i32 {
        self.length
    }
}

#[repr(C)]
pub struct MonoArray<T> {
    pub klass: *mut c_void,
    pub monitor: *mut c_void,
    pub bounds: *mut c_void,
    pub max_length: i32,
    pub vector: [T; 0],
}

impl<T> MonoArray<T> {
    pub fn get_length(&self) -> i32 {
        self.max_length
    }

    pub fn get_pointer(&self) -> *const T {
        self.vector.as_ptr()
    }

    pub fn get_pointer_mut(&mut self) -> *mut T {
        self.vector.as_mut_ptr()
    }
}

#[repr(C)]
pub struct MonoList<T> {
    pub unk0: *mut c_void,
    pub unk1: *mut c_void,
    pub items: *mut MonoArray<T>,
    pub size: i32,
    pub version: i32,
}

impl<T> MonoList<T> {
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
pub struct MonoDictionary<K, V> {
    pub unk0: *mut c_void,
    pub unk1: *mut c_void,
    pub keys: *mut MonoArray<K>,
    pub values: *mut MonoArray<V>,
    pub size: i32,
}

impl<K, V> MonoDictionary<K, V> {
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

    pub fn get_size(&self) -> i32 {
        self.size
    }
}

pub fn normalize_angle(angle: f32) -> f32 {
    let mut a = angle;
    while a > 360.0 {
        a -= 360.0;
    }
    while a < 0.0 {
        a += 360.0;
    }
    a
}

pub fn normalize_angles(angles: Vector3) -> Vector3 {
    Vector3::new(
        normalize_angle(angles.x),
        normalize_angle(angles.y),
        normalize_angle(angles.z),
    )
}

pub fn to_euler_rad(q: Quaternion) -> Vector3 {
    let sqw = q.w * q.w;
    let sqx = q.x * q.x;
    let sqy = q.y * q.y;
    let sqz = q.z * q.z;
    let unit = sqx + sqy + sqz + sqw;
    let test = q.x * q.w - q.y * q.z;
    let mut v = Vector3::zero();
    if test > 0.4995 * unit {
        v.y = 2.0 * q.y.atan2(q.x);
        v.x = std::f32::consts::FRAC_PI_2;
        v.z = 0.0;
        return normalize_angles(v * crate::il2cpp_sdk::quaternion::RAD2DEG);
    }
    if test < -0.4995 * unit {
        v.y = -2.0 * q.y.atan2(q.x);
        v.x = -std::f32::consts::FRAC_PI_2;
        v.z = 0.0;
        return normalize_angles(v * crate::il2cpp_sdk::quaternion::RAD2DEG);
    }
    let q_rotated = Quaternion::new(q.w, q.z, q.x, q.y);
    v.y = (2.0 * q_rotated.x * q_rotated.w + 2.0 * q_rotated.y * q_rotated.z)
        .atan2(1.0 - 2.0 * (q_rotated.z * q_rotated.z + q_rotated.w * q_rotated.w));
    v.x = (2.0 * (q_rotated.x * q_rotated.z - q_rotated.w * q_rotated.y)).asin();
    v.z = (2.0 * q_rotated.x * q_rotated.y + 2.0 * q_rotated.z * q_rotated.w)
        .atan2(1.0 - 2.0 * (q_rotated.y * q_rotated.y + q_rotated.z * q_rotated.z));
    normalize_angles(v * crate::il2cpp_sdk::quaternion::RAD2DEG)
}

pub fn get_rotation_to_location(target_location: Vector3, y_bias: f32, my_loc: Vector3) -> Quaternion {
    Quaternion::look_rotation(
        (target_location + Vector3::new(0.0, y_bias, 0.0)) - my_loc,
        Vector3::new(0.0, 1.0, 0.0),
    )
}

#[repr(C)]
union IntFloat {
    i: i32,
    f: f32,
}

pub fn get_obscured_int_value(location: usize) -> i32 {
    unsafe {
        let crypto_key = *(location as *const i32);
        let obfuscated_value = *((location + 0x4) as *const i32);
        obfuscated_value ^ crypto_key
    }
}

pub fn get_obscured_bool_value(location: usize) -> bool {
    unsafe {
        let crypto_key = *((location + 0x8) as *const i32);
        let obfuscated_value = *((location + 0xC) as *const i32);
        (obfuscated_value ^ crypto_key) != 0
    }
}

pub fn set_obscured_int_value(location: usize, value: i32) {
    unsafe {
        let crypto_key = *(location as *const i32);
        *((location + 0x4) as *mut i32) = value ^ crypto_key;
    }
}

pub fn get_obscured_float_value(location: usize) -> f32 {
    unsafe {
        let crypto_key = *(location as *const i32);
        let obfuscated_value = *((location + 0x4) as *const i32);
        let int_val = obfuscated_value ^ crypto_key;
        let if_union = IntFloat { i: int_val };
        if_union.f
    }
}

pub fn set_obscured_float_value(location: usize, value: f32) {
    unsafe {
        let crypto_key = *(location as *const i32);
        let if_union = IntFloat { f: value };
        let int_representation = if_union.i;
        let if_union2 = IntFloat {
            i: int_representation ^ crypto_key,
        };
        *((location + 0x4) as *mut f32) = if_union2.f;
    }
}