use crate::includes::kittymemory::kitty_utils::{validate_hex_string, from_hex, to_hex};
use crate::includes::kittymemory::{ProcMap, mem_read, mem_write, read_hex_str};
use core::ffi::c_void;

pub struct MemoryPatch {
    address: usize,
    size: usize,
    orig_code: Vec<u8>,
    patch_code: Vec<u8>,
}

impl MemoryPatch {
    pub fn new() -> Self {
        Self {
            address: 0,
            size: 0,
            orig_code: Vec::new(),
            patch_code: Vec::new(),
        }
    }

    pub fn from_map(map: &ProcMap, rel_addr: usize, patch: &[u8]) -> Self {
        let mut obj = Self::new();

        if !map.is_valid() || rel_addr == 0 || patch.is_empty() {
            return obj;
        }

        obj.address = (map.start_address as usize) + rel_addr;
        obj.size = patch.len();

        obj.orig_code.resize(obj.size, 0);
        obj.patch_code = patch.to_vec();

        mem_read(
            obj.orig_code.as_mut_ptr() as *mut c_void,
            obj.address as *const c_void,
            obj.size,
        );

        obj
    }

    pub fn from_absolute(address: usize, patch: &[u8]) -> Self {
        let mut obj = Self::new();

        if address == 0 || patch.is_empty() {
            return obj;
        }

        obj.address = address;
        obj.size = patch.len();

        obj.orig_code.resize(obj.size, 0);
        obj.patch_code = patch.to_vec();

        mem_read(
            obj.orig_code.as_mut_ptr() as *mut c_void,
            obj.address as *const c_void,
            obj.size,
        );

        obj
    }

    pub fn create_with_hex(map: &ProcMap, rel: usize, hex: &str) -> Self {
        let mut obj = Self::new();

        let mut hex_mut = hex.to_string();
        if !map.is_valid() || rel == 0 || !validate_hex_string(&mut hex_mut) {
            return obj;
        }

        obj.address = (map.start_address as usize) + rel;
        obj.size = hex_mut.len() / 2;

        obj.orig_code.resize(obj.size, 0);
        obj.patch_code = from_hex(&hex_mut);

        mem_read(
            obj.orig_code.as_mut_ptr() as *mut c_void,
            obj.address as *const c_void,
            obj.size,
        );

        obj
    }

    pub fn create_with_hex_absolute(address: usize, hex: &str) -> Self {
        let mut obj = Self::new();

        let mut hex_mut = hex.to_string();
        if address == 0 || !validate_hex_string(&mut hex_mut) {
            return obj;
        }

        obj.address = address;
        obj.size = hex_mut.len() / 2;

        obj.orig_code.resize(obj.size, 0);
        obj.patch_code = from_hex(&hex_mut);

        mem_read(
            obj.orig_code.as_mut_ptr() as *mut c_void,
            obj.address as *const c_void,
            obj.size,
        );

        obj
    }

    pub fn is_valid(&self) -> bool {
        self.address != 0
            && self.size > 0
            && self.orig_code.len() == self.size
            && self.patch_code.len() == self.size
    }

    pub fn get_patch_size(&self) -> usize {
        self.size
    }

    pub fn get_target_address(&self) -> usize {
        self.address
    }

    pub fn restore(&self) -> bool {
        if !self.is_valid() {
            return false;
        }
        mem_write(
            self.address as *mut c_void,
            self.orig_code.as_ptr() as *const c_void,
            self.size,
        )
    }

    pub fn modify(&self) -> bool {
        if !self.is_valid() {
            return false;
        }
        mem_write(
            self.address as *mut c_void,
            self.patch_code.as_ptr() as *const c_void,
            self.size,
        )
    }

    pub fn get_curr_bytes(&self) -> String {
        if !self.is_valid() {
            return String::new();
        }
        read_hex_str(self.address, self.size)
    }

    pub fn get_orig_bytes(&self) -> String {
        if !self.is_valid() {
            return String::new();
        }
        to_hex(&self.orig_code)
    }

    pub fn get_patch_bytes(&self) -> String {
        if !self.is_valid() {
            return String::new();
        }
        to_hex(&self.patch_code)
    }
}