use std::fs::File;
use std::io::{BufRead, BufReader};
use std::ptr;

#[derive(Debug, Clone)]
pub struct ProcMapInfo {
    pub start: usize,
    pub end: usize,
    pub offset: usize,
    pub perms: i32,
    pub inode: u64,
    pub dev: String,
    pub path: String,
}

pub struct Remapper;

impl Remapper {
    pub fn list_modules_with_name(name: &str) -> Vec<ProcMapInfo> {
        let mut result = Vec::new();
        if let Ok(file) = File::open("/proc/self/maps") {
            for line in BufReader::new(file).lines().flatten() {
                if line.contains(name) {
                    if let Some(info) = Self::parse_map_line(&line) { result.push(info); }
                }
            }
        }
        result
    }

    fn parse_map_line(line: &str) -> Option<ProcMapInfo> {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() < 5 { return None; }
        let addr_parts: Vec<&str> = parts[0].split('-').collect();
        if addr_parts.len() != 2 { return None; }
        let start = usize::from_str_radix(addr_parts[0], 16).ok()?;
        let end = usize::from_str_radix(addr_parts[1], 16).ok()?;
        let perms_str = parts[1];
        let mut perms = 0;
        if perms_str.contains('r') { perms |= libc::PROT_READ; }
        if perms_str.contains('w') { perms |= libc::PROT_WRITE; }
        if perms_str.contains('x') { perms |= libc::PROT_EXEC; }
        let offset = usize::from_str_radix(parts[2], 16).ok()?;
        let dev = parts[3].to_string();
        let inode = parts[4].parse::<u64>().ok()?;
        let path = if parts.len() > 5 { parts[5].to_string() } else { String::new() };
        Some(ProcMapInfo { start, end, offset, perms, inode, dev, path })
    }

    pub fn remap_simple(libname: &str) {
        for info in Self::list_modules_with_name(libname) {
            unsafe {
                let addr = info.start as *mut libc::c_void;
                let size = info.end - info.start;
                let new_mem = libc::malloc(size);
                if new_mem.is_null() { continue; }
                ptr::copy_nonoverlapping(addr as *const u8, new_mem as *mut u8, size);
                libc::mprotect(addr, size, libc::PROT_READ | libc::PROT_WRITE | libc::PROT_EXEC);
                ptr::copy_nonoverlapping(new_mem as *const u8, addr as *mut u8, size);
                libc::mprotect(addr, size, info.perms);
                libc::free(new_mem);
            }
        }
    }
}