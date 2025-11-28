use crate::includes::kittymemory::kitty_utils::{validate_hex_string, from_hex};
use crate::includes::kittymemory::ProcMap;
use std::ffi::CString;


#[inline]
fn compare(data: *const u8, pattern: *const u8, mask: *const u8) -> bool {
    unsafe {
        let mut d = data;
        let mut p = pattern;
        let mut m = mask;

        while *m != 0 {
            if *m == b'x' && *d != *p {
                return false;
            }
            d = d.add(1);
            p = p.add(1);
            m = m.add(1);
        }
        true
    }
}
 
pub fn find_in_range(start: usize, end: usize, pattern: *const u8, mask: &str) -> usize {
    let scan_size = mask.len();
    if scan_size < 1 || start + scan_size > end {
        return 0;
    }

    let length = end - start;
    let mask_c = CString::new(mask).unwrap();
    let mask_ptr = mask_c.as_ptr() as *const u8;

    for i in 0..length {
        let current_end = start + i + scan_size;
        if current_end > end {
            break;
        }

        let addr = (start + i) as *const u8;
        if compare(addr, pattern, mask_ptr) {
            return start + i;
        }
    }

    0
}


pub fn find_bytes_all(start: usize, end: usize, bytes: &[u8], mask: &str) -> Vec<usize> {
    let mut out = vec![];
    if start >= end {
        return out;
    }

    let scan_size = mask.len();
    let mut current = start;

    loop {
        if !out.is_empty() {
            current = out[out.len() - 1] + scan_size;
        }

        let found = find_in_range(current, end, bytes.as_ptr(), mask);
        if found == 0 {
            break;
        }

        out.push(found);
    }

    out
}

pub fn find_bytes_first(start: usize, end: usize, bytes: &[u8], mask: &str) -> usize {
    if start >= end || bytes.is_empty() || mask.is_empty() {
        return 0;
    }
    find_in_range(start, end, bytes.as_ptr(), mask)
}

pub fn find_hex_all(start: usize, end: usize, hex: &str, mask: &str) -> Vec<usize> {
    if start >= end || mask.is_empty() {
        return vec![];
    }

    let mut hex_mut = hex.to_string();
    if !validate_hex_string(&mut hex_mut) {
        return vec![];
    }

    if hex_mut.len() / 2 != mask.len() {
        return vec![];
    }

    let pattern = from_hex(&hex_mut);
    find_bytes_all(start, end, &pattern, mask)
}

pub fn find_hex_first(start: usize, end: usize, hex: &str, mask: &str) -> usize {
    if start >= end || mask.is_empty() {
        return 0;
    }

    let mut hex_mut = hex.to_string();
    if !validate_hex_string(&mut hex_mut) {
        return 0;
    }

    if hex_mut.len() / 2 != mask.len() {
        return 0;
    }

    let pattern = from_hex(&hex_mut);
    find_bytes_first(start, end, &pattern, mask)
}


pub fn find_data_all(start: usize, end: usize, data: &[u8]) -> Vec<usize> {
    if start >= end || data.is_empty() {
        return vec![];
    }

    let mask = "x".repeat(data.len());
    find_bytes_all(start, end, data, &mask)
}

pub fn find_data_first(start: usize, end: usize, data: &[u8]) -> usize {
    if start >= end || data.is_empty() {
        return 0;
    }

    let mask = "x".repeat(data.len());
    find_bytes_first(start, end, data, &mask)
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct RegisterNativeFn {
    pub name_ptr: usize,
    pub sig_ptr: usize,
    pub fn_ptr: usize,
}

pub fn find_register_native_fn(maps: &[ProcMap], name: &str) -> Option<RegisterNativeFn> {
    if name.is_empty() || maps.is_empty() {
        return None;
    }

    let name_bytes = name.as_bytes();
    let mut string_loc = 0usize;

    
    for m in maps.iter() {
        if m.is_rx {
            let found = find_data_first(m.start_address as usize, m.end_address as usize, name_bytes);
            if found != 0 {
                string_loc = found;
                break;
            }
        }
    }

    if string_loc == 0 {
        return None;
    }

    
    let mut xref = 0usize;
    for m in maps.iter() {
        if m.is_rw {
            let ptr_bytes = string_loc.to_ne_bytes();
            let found = find_data_first(m.start_address as usize, m.end_address as usize, &ptr_bytes);
            if found != 0 {
                xref = found;
                break;
            }
        }
    }

    if xref == 0 {
        return None;
    }

    
    unsafe {
        let fn_ptr = xref as *const RegisterNativeFn;
        Some(*fn_ptr)
    }
}

pub struct Scanner {
    pub start: usize,
    pub end: usize,
}

impl Scanner {
    pub fn from_range(start: usize, end: usize) -> Self {
        Scanner { start, end }
    }

    pub fn get_lib_base(lib_name: &str) -> Option<usize> {
        let maps = crate::includes::kittymemory::get_maps_by_name(lib_name);
        maps.get(0).map(|m| m.start_address as usize)
    }

    pub fn get_lib_size(lib_name: &str) -> Option<usize> {
        let maps = crate::includes::kittymemory::get_maps_by_name(lib_name);
        maps.get(0).map(|m| (m.end_address - m.start_address) as usize)
    }

    pub fn find_bytes_first(&self, bytes: &[u8], mask: &str) -> Option<usize> {
        let addr = crate::includes::kittymemory::find_bytes_first(self.start, self.end, bytes, mask);
        if addr == 0 { None } else { Some(addr) }
    }

    pub fn find_bytes_all(&self, bytes: &[u8], mask: &str) -> Vec<usize> {
        crate::includes::kittymemory::find_bytes_all(self.start, self.end, bytes, mask)
    }

    pub fn find_hex_first(&self, hex: &str, mask: &str) -> Option<usize> {
        let addr = crate::includes::kittymemory::find_hex_first(self.start, self.end, hex, mask);
        if addr == 0 { None } else { Some(addr) }
    }

    pub fn find_hex_all(&self, hex: &str, mask: &str) -> Vec<usize> {
        crate::includes::kittymemory::find_hex_all(self.start, self.end, hex, mask)
    }
}