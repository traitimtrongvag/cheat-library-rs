use crate::includes::kittymemory::{ProcMap, mem_read, mem_write, read_hex_str};
use core::ffi::c_void;

pub struct MemoryBackup {
    address: usize,
    size: usize,
    orig_code: Vec<u8>,
}

impl MemoryBackup {
    pub fn new() -> Self {
        Self {
            address: 0,
            size: 0,
            orig_code: Vec::new(),
        }
    }

    pub fn from_map(map: &ProcMap, rel_address: usize, backup_size: usize) -> Self {
        let mut obj = Self::new();

        if !map.is_valid() || rel_address == 0 || backup_size < 1 {
            return obj;
        }

        obj.address = (map.start_address as usize) + rel_address;

        if obj.address == 0 {
            return obj;
        }

        obj.size = backup_size;
        obj.orig_code.resize(backup_size, 0);

        mem_read(
            obj.orig_code.as_mut_ptr() as *mut c_void,
            obj.address as *const c_void,
            obj.size,
        );

        obj
    }

    pub fn from_absolute(address: usize, backup_size: usize) -> Self {
        let mut obj = Self::new();

        if address == 0 || backup_size < 1 {
            return obj;
        }

        obj.address = address;
        obj.size = backup_size;
        obj.orig_code.resize(backup_size, 0);

        mem_read(
            obj.orig_code.as_mut_ptr() as *mut c_void,
            obj.address as *const c_void,
            obj.size,
        );

        obj
    }

    pub fn is_valid(&self) -> bool {
        self.address != 0 && self.size > 0 && self.orig_code.len() == self.size
    }

    pub fn get_backup_size(&self) -> usize {
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
        crate::includes::kittymemory::kitty_utils::to_hex(&self.orig_code)
    }
}