use core::ffi::c_void;
use std::{fs::File, io::{BufRead, BufReader}, ptr, slice};
use libc::{mprotect, sysconf, _SC_PAGESIZE, PROT_READ, PROT_WRITE, PROT_EXEC};
use log::{error, info};

#[derive(Debug, Clone)]
pub struct ProcMap {
    pub start_address: u64,
    pub end_address: u64,
    pub length: usize,
    pub protection: i32,
    pub readable: bool,
    pub writeable: bool,
    pub executable: bool,
    pub is_private: bool,
    pub is_shared: bool,
    pub is_ro: bool,
    pub is_rw: bool,
    pub is_rx: bool,
    pub offset: u64,
    pub dev: String,
    pub inode: u64,
    pub pathname: String,
}

impl ProcMap {
    pub fn new() -> Self {
        ProcMap {
            start_address: 0,
            end_address: 0,
            length: 0,
            protection: 0,
            readable: false,
            writeable: false,
            executable: false,
            is_private: false,
            is_shared: false,
            is_ro: false,
            is_rw: false,
            is_rx: false,
            offset: 0,
            dev: String::new(),
            inode: 0,
            pathname: String::new(),
        }
    }

    pub fn is_valid(&self) -> bool {
        self.length > 0
    }

    pub fn is_unknown(&self) -> bool {
        self.pathname.is_empty()
    }
}

fn sys_page_size() -> usize {
    let v = unsafe { sysconf(_SC_PAGESIZE) };
    if v <= 0 {
        4096
    } else {
        v as usize
    }
}

fn page_start_of(addr: usize) -> usize {
    addr & !(sys_page_size() - 1)
}

fn page_end_of(addr: usize, len: usize) -> usize {
    page_start_of(addr + len - 1)
}

fn page_len_of(addr: usize, len: usize) -> usize {
    page_end_of(addr, len) - page_start_of(addr) + sys_page_size()
}

pub fn set_address_protection(address: *mut c_void, length: usize, protection: i32) -> i32 {
    let page_start = page_start_of(address as usize);
    let page_len = page_len_of(address as usize, length);
    let ret = unsafe { mprotect(page_start as *mut c_void, page_len, protection) };
    info!("mprotect({:p}, {}, {}) = {}", address, length, protection, ret);
    ret
}

pub fn mem_write(address: *mut c_void, buffer: *const c_void, len: usize) -> bool {
    if address.is_null() || buffer.is_null() || len == 0 || len > (i32::MAX as usize) {
        error!("memWrite invalid args");
        return false;
    }

    let map = get_address_map(address);
    if !map.is_valid() {
        error!("memWrite: address {:p} not in any map", address);
        return false;
    }

    if map.protection & PROT_WRITE != 0 {
        unsafe {
            ptr::copy_nonoverlapping(buffer as *const u8, address as *mut u8, len);
        }
        return true;
    }

    let newprot = map.protection | PROT_WRITE;
    if set_address_protection(address, len, newprot) != 0 {
        error!("memWrite: cannot add write perm to {:p}", address);
        return false;
    }

    unsafe {
        ptr::copy_nonoverlapping(buffer as *const u8, address as *mut u8, len);
    }

    if set_address_protection(address, len, map.protection) != 0 {
        error!("memWrite: cannot revert protection of {:p}", address);
        return false;
    }

    true
}

pub fn mem_read(buffer: *mut c_void, address: *const c_void, len: usize) -> bool {
    if buffer.is_null() || address.is_null() || len == 0 || len > (i32::MAX as usize) {
        error!("memRead invalid args");
        return false;
    }
    unsafe {
        ptr::copy_nonoverlapping(address as *const u8, buffer as *mut u8, len);
    }
    true
}

pub fn read2hexstr(address: *const c_void, len: usize) -> String {
    if len == 0 {
        return String::new();
    }
    let mut buf = vec![0u8; len];
    if !mem_read(buf.as_mut_ptr() as *mut c_void, address, len) {
        return String::new();
    }
    let mut ret = String::with_capacity(len * 2);
    for b in &buf {
        use core::fmt::Write;
        write!(&mut ret, "{:02X}", b).ok();
    }
    ret
}

pub fn read_hex_str(address: usize, len: usize) -> String {
    read2hexstr(address as *const c_void, len)
}

fn parse_protection(perms: &str) -> i32 {
    let mut prot = 0;
    if perms.contains('r') {
        prot |= PROT_READ;
    }
    if perms.contains('w') {
        prot |= PROT_WRITE;
    }
    if perms.contains('x') {
        prot |= PROT_EXEC;
    }
    prot
}

pub fn parse_maps_line(line: &str) -> Option<ProcMap> {
    let parts: Vec<&str> = line.split_whitespace().collect();
    if parts.len() < 5 {
        return None;
    }

    let (start_s, end_s) = parts[0].split_once('-')?;
    let start = u64::from_str_radix(start_s, 16).ok()?;
    let end = u64::from_str_radix(end_s, 16).ok()?;

    let perms = parts[1];
    let offset = u64::from_str_radix(parts[2], 16).ok()?;

    let pathname = if parts.len() >= 6 {
        parts[5..].join(" ")
    } else {
        String::new()
    };

    Some(ProcMap {
        start_address: start,
        end_address: end,
        length: (end - start) as usize,
        protection: parse_protection(perms),
        readable: perms.contains('r'),
        writeable: perms.contains('w'),
        executable: perms.contains('x'),
        is_private: perms.contains('p'),
        is_shared: perms.contains('s'),
        is_ro: perms == "r--p" || perms == "r--s",
        is_rw: perms.starts_with("rw"),
        is_rx: perms.starts_with("r-x"),
        offset,
        dev: parts[3].to_string(),
        inode: parts[4].parse().unwrap_or(0),
        pathname,
    })
}

pub fn get_address_map(address: *const c_void) -> ProcMap {
    if address.is_null() {
        return ProcMap::new();
    }
    let addr_val = address as usize as u64;
    match File::open("/proc/self/maps") { Ok(f) => {
        for line_res in BufReader::new(f).lines() {
            if let Ok(line) = line_res {
                if let Some((start_s, end_s)) = line
                    .split_whitespace()
                    .next()
                    .and_then(|r| r.split_once('-'))
                {
                    if let (Ok(s), Ok(e)) = (
                        u64::from_str_radix(start_s, 16),
                        u64::from_str_radix(end_s, 16),
                    ) {
                        if addr_val >= s && addr_val <= e {
                            if let Some(map) = parse_maps_line(&line) {
                                return map;
                            }
                        }
                    }
                }
            }
        }
    } _ => {
        error!("getAddressMap: cannot open /proc/self/maps");
    }}
    ProcMap::new()
}

pub fn get_maps_by_name(name: &str) -> Vec<ProcMap> {
    let mut maps = Vec::new();
    if let Ok(f) = File::open("/proc/self/maps") {
        for line in BufReader::new(f).lines().flatten() {
            if line.contains(name) {
                if let Some(map) = parse_maps_line(&line) {
                    maps.push(map);
                }
            }
        }
    }
    maps
}

pub fn get_library_base_map_from_maps(maps: &[ProcMap]) -> ProcMap {
    let mut ret_map = ProcMap::new();
    for m in maps {
        if !m.is_valid() || m.writeable || !m.is_private {
            continue;
        }
        let addr = m.start_address as usize as *const u8;
        unsafe {
            if !addr.is_null() {
                let first4 = slice::from_raw_parts(addr, 4);
                if first4.len() >= 4
                    && first4[0] == 0x7f
                    && first4[1] == b'E'
                    && first4[2] == b'L'
                    && first4[3] == b'F'
                {
                    ret_map = m.clone();
                    break;
                }
            }
        }
    }
    ret_map
}

pub fn get_library_base_map_by_name(name: &str) -> ProcMap {
    get_library_base_map_from_maps(&get_maps_by_name(name))
}