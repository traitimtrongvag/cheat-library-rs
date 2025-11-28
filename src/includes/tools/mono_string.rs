use std::os::raw::c_char;

#[repr(C)]
pub struct MonoString {
    klass: *mut std::ffi::c_void,
    monitor: *mut std::ffi::c_void,
    length: i32,
    chars: [c_char; 1],
}

impl MonoString {
    pub fn get_length(&self) -> i32 {
        self.length
    }

    fn get_chars(&self) -> *const u16 {
        self.chars.as_ptr() as *const u16
    }

    fn get_chars_mut(&mut self) -> *mut u16 {
        self.chars.as_mut_ptr() as *mut u16
    }

    pub fn to_string(&self) -> String {
        unsafe {
            let chars = std::slice::from_raw_parts(self.get_chars(), self.length as usize);
            String::from_utf16_lossy(chars)
        }
    }

    pub fn set_mono_string(&mut self, s: &str) {
        self.length = s.len() as i32;
        let utf16: Vec<u16> = s.encode_utf16().collect();
        
        unsafe {
            let chars_ptr = self.get_chars_mut();
            std::ptr::copy_nonoverlapping(
                utf16.as_ptr(),
                chars_ptr,
                self.length as usize,
            );
        }
    }
}

pub fn utf16_to_utf8(u16str: &[u16]) -> String {
    String::from_utf16_lossy(u16str)
}

pub fn utf16le_to_utf8(u16str: &[u16]) -> String {
    let data = if !u16str.is_empty() && u16str[0] == 0xFEFF {
        &u16str[1..]
    } else {
        u16str
    };
    
    String::from_utf16_lossy(data)
}

pub fn utf16be_to_utf8(u16str: &[u16]) -> String {
    let start = if !u16str.is_empty() && u16str[0] == 0xFFFE { 1 } else { 0 };
    
    let swapped: Vec<u16> = u16str[start..]
        .iter()
        .map(|&c| c.swap_bytes())
        .collect();
    
    String::from_utf16_lossy(&swapped)
}

pub fn utf8_to_utf16le(u8str: &str, add_bom: bool) -> Vec<u16> {
    let mut result: Vec<u16> = if add_bom {
        vec![0xFEFF]
    } else {
        Vec::new()
    };
    
    result.extend(u8str.encode_utf16());
    result
}

pub fn utf8_to_utf16be(u8str: &str, add_bom: bool) -> Vec<u16> {
    let mut result = utf8_to_utf16le(u8str, add_bom);
    
    for c in &mut result {
        *c = c.swap_bytes();
    }
    
    result
}