use std::fs::File;
use std::io::{BufRead, BufReader};
use jni::JNIEnv;
use jni::objects::{JObject, JValue};

const PAGE_SIZE: usize = 4096;

pub fn get_page_protection(addr: *const std::ffi::c_void) -> i32 {
    let mut result = 0;

    let maps_file = match File::open("/proc/self/maps") {
        Ok(f) => f,
        Err(_) => return 0,
    };

    let reader = BufReader::new(maps_file);
    let addr_val = addr as usize;

    for line in reader.lines().flatten() {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() < 2 {
            continue;
        }

        let addr_range: Vec<&str> = parts[0].split('-').collect();
        if addr_range.len() != 2 {
            continue;
        }

        let start = usize::from_str_radix(addr_range[0], 16).unwrap_or(0);
        let end = usize::from_str_radix(addr_range[1], 16).unwrap_or(0);
        let prot = parts[1];

        if addr_val >= start && addr_val <= end {
            if prot.contains('r') {
                result |= libc::PROT_READ;
            }
            if prot.contains('w') {
                result |= libc::PROT_WRITE;
            }
            if prot.contains('x') {
                result |= libc::PROT_EXEC;
            }
            break;
        }
    }

    result
}

pub fn set_page_protection(addr: *mut std::ffi::c_void, prot: i32) -> bool {
    unsafe {
        let page = ((addr as usize) & !(PAGE_SIZE - 1)) as *mut std::ffi::c_void;
        let len = ((addr as usize) + PAGE_SIZE) - (page as usize);
        libc::mprotect(page, len, prot) == 0
    }
}

pub fn calculate_checksum(buffer: &[u8]) -> u32 {
    buffer.iter().map(|&b| b as u32).sum()
}

pub fn hook(target: *mut std::ffi::c_void, replace: *mut std::ffi::c_void, backup: *mut *mut std::ffi::c_void) -> bool {
    let prot = get_page_protection(target);
    set_page_protection(target, libc::PROT_READ | libc::PROT_WRITE | libc::PROT_EXEC);
    
    unsafe {
        let result = crate::includes::dobby::DobbyHook(target, replace, backup) == 0;
        set_page_protection(target, prot);
        result
    }
}

pub fn read(addr: *const std::ffi::c_void, buffer: &mut [u8]) -> bool {
    unsafe {
        std::ptr::copy_nonoverlapping(addr as *const u8, buffer.as_mut_ptr(), buffer.len());
        true
    }
}

pub fn write(addr: *mut std::ffi::c_void, buffer: &[u8]) -> bool {
    unsafe {
        std::ptr::copy_nonoverlapping(buffer.as_ptr(), addr as *mut u8, buffer.len());
        true
    }
}

pub fn read_addr(addr: *const std::ffi::c_void, buffer: &mut [u8]) -> bool {
    unsafe {
        let page_size = libc::sysconf(libc::_SC_PAGESIZE) as usize;
        let size = page_size * std::mem::size_of::<usize>();
        let page_addr = ((addr as usize) - ((addr as usize) % page_size) - page_size) as *mut std::ffi::c_void;
        
        if libc::mprotect(
            page_addr,
            size,
            libc::PROT_EXEC | libc::PROT_READ | libc::PROT_WRITE,
        ) == 0
        {
            std::ptr::copy_nonoverlapping(addr as *const u8, buffer.as_mut_ptr(), buffer.len());
            true
        } else {
            false
        }
    }
}

pub fn write_addr(addr: *mut std::ffi::c_void, buffer: &[u8]) -> bool {
    unsafe {
        let page_size = libc::sysconf(libc::_SC_PAGESIZE) as usize;
        let size = page_size * std::mem::size_of::<usize>();
        let page_addr = ((addr as usize) - ((addr as usize) % page_size) - page_size) as *mut std::ffi::c_void;
        
        if libc::mprotect(
            page_addr,
            size,
            libc::PROT_EXEC | libc::PROT_READ | libc::PROT_WRITE,
        ) == 0
        {
            std::ptr::copy_nonoverlapping(buffer.as_ptr(), addr as *mut u8, buffer.len());
            true
        } else {
            false
        }
    }
}

pub fn set_writable(addr: *mut std::ffi::c_void) -> bool {
    unsafe {
        let page_size = libc::sysconf(libc::_SC_PAGESIZE) as usize;
        let protect_size = page_size * std::mem::size_of::<usize>();
        let page_addr = ((addr as usize) - ((addr as usize) % page_size) - page_size) as *mut std::ffi::c_void;
        
        libc::mprotect(
            page_addr,
            protect_size,
            libc::PROT_EXEC | libc::PROT_READ | libc::PROT_WRITE,
        ) == 0
    }
}

pub fn random_string(len: usize) -> String {
    use rand::Rng;
    const CHARSET: &[u8] = b"0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz";
    let mut rng = rand::thread_rng();
    
    (0..len)
        .map(|_| {
            let idx = rng.gen_range(0..CHARSET.len());
            CHARSET[idx] as char
        })
        .collect()
}

pub fn get_base_address(name: &str) -> usize {
    let maps_file = match File::open("/proc/self/maps") {
        Ok(f) => f,
        Err(_) => return 0,
    };

    let reader = BufReader::new(maps_file);

    for line in reader.lines().flatten() {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() < 6 {
            continue;
        }

        let lib_name = parts[5];
        if lib_name.ends_with(name) {
            let addr_range: Vec<&str> = parts[0].split('-').collect();
            if let Some(addr_str) = addr_range.first() {
                if let Ok(addr) = usize::from_str_radix(addr_str, 16) {
                    return addr;
                }
            }
        }
    }

    0
}

pub fn get_end_address(name: &str) -> usize {
    let maps_file = match File::open("/proc/self/maps") {
        Ok(f) => f,
        Err(_) => return 0,
    };

    let reader = BufReader::new(maps_file);
    let mut found = false;
    let mut end = 0;

    for line in reader.lines().flatten() {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() < 6 {
            continue;
        }

        let lib_name = parts[5];
        
        if lib_name.ends_with(name) {
            if !found {
                found = true;
            }
            
            let addr_range: Vec<&str> = parts[0].split('-').collect();
            if let Some(addr_str) = addr_range.get(1) {
                if let Ok(addr) = usize::from_str_radix(addr_str, 16) {
                    end = addr;
                }
            }
        } else if found {
            break;
        }
    }

    end
}

fn get_bits(x: u8) -> u8 {
    if x >= b'0' && x <= b'9' {
        x - b'0'
    } else {
        (x & !0x20) - b'A' + 0xa
    }
}

fn get_byte(pattern: &[u8]) -> u8 {
    (get_bits(pattern[0]) << 4) | get_bits(pattern[1])
}

pub fn find_pattern(lib: &str, pattern: &str) -> usize {
    let start = get_base_address(lib);
    if start == 0 {
        return 0;
    }
    
    let end = get_end_address(lib);
    if end == 0 {
        return 0;
    }

    let pattern_bytes = pattern.as_bytes();
    let mut cur_pat = 0;
    let mut first_match = 0;

    for p_cur in start..end {
        unsafe {
            let cur_byte = *(p_cur as *const u8);
            
            if pattern_bytes[cur_pat] == b'?' || 
               (cur_pat + 1 < pattern_bytes.len() && 
                pattern_bytes[cur_pat] == b'?' && 
                pattern_bytes[cur_pat + 1] == b'?') ||
               cur_byte == get_byte(&pattern_bytes[cur_pat..])
            {
                if first_match == 0 {
                    first_match = p_cur;
                }
                
                if pattern_bytes[cur_pat] == b'?' && 
                   cur_pat + 1 < pattern_bytes.len() && 
                   pattern_bytes[cur_pat + 1] == b'?'
                {
                    cur_pat += 2;
                } else if pattern_bytes[cur_pat] == b'?' {
                    cur_pat += 1;
                } else {
                    cur_pat += 2;
                }
                
                if cur_pat >= pattern_bytes.len() {
                    return first_match;
                }
                
                if cur_pat < pattern_bytes.len() && pattern_bytes[cur_pat] == b' ' {
                    cur_pat += 1;
                }
                
                if cur_pat >= pattern_bytes.len() {
                    return first_match;
                }
            } else if first_match != 0 {
                cur_pat = 0;
                first_match = 0;
            }
        }
    }

    0
}

pub fn get_real_offset(library_name: &str, relative_addr: usize) -> usize {
    let lib_base = get_base_address(library_name);
    if lib_base == 0 {
        return 0;
    }
    lib_base + relative_addr
}

pub fn string_to_offset(s: &str) -> usize {
    if s.starts_with("0x") || s.starts_with("0X") {
        usize::from_str_radix(&s[2..], 16).unwrap_or(0)
    } else {
        usize::from_str_radix(s, 16).unwrap_or(0)
    }
}

pub fn get_android_id(env: &mut JNIEnv, context: JObject) -> Option<String> {
    let resolver_result = env.call_method(
        context,
        "getContentResolver",
        "()Landroid/content/ContentResolver;",
        &[]
    ).ok()?;
    
    let resolver = resolver_result.l().ok()?;
    
    let settings_secure_class = env.find_class("android/provider/Settings$Secure").ok()?;
    let android_id_key = env.new_string("android_id").ok()?;
    
    let result = env.call_static_method(
        settings_secure_class,
        "getString",
        "(Landroid/content/ContentResolver;Ljava/lang/String;)Ljava/lang/String;",
        &[JValue::Object(&resolver), JValue::Object(&android_id_key.into())]
    ).ok()?;
    
    let jstring = result.l().ok()?;
    let rust_string: String = env.get_string((&jstring).into()).ok()?.into();
    Some(rust_string)
}

pub fn get_device_model(env: &mut JNIEnv) -> Option<String> {
    let build_class = env.find_class("android/os/Build").ok()?;
    
    let result = env.get_static_field(
        build_class,
        "MODEL",
        "Ljava/lang/String;"
    ).ok()?;
    
    let jstring = result.l().ok()?;
    let rust_string: String = env.get_string((&jstring).into()).ok()?.into();
    Some(rust_string)
}

pub fn get_device_brand(env: &mut JNIEnv) -> Option<String> {
    let build_class = env.find_class("android/os/Build").ok()?;
    
    let result = env.get_static_field(
        build_class,
        "BRAND",
        "Ljava/lang/String;"
    ).ok()?;
    
    let jstring = result.l().ok()?;
    let rust_string: String = env.get_string((&jstring).into()).ok()?.into();
    Some(rust_string)
}

pub fn get_device_unique_identifier(env: &mut JNIEnv, uuid: &str) -> Option<String> {
    let uuid_class = env.find_class("java/util/UUID").ok()?;
    
    let bytes = uuid.as_bytes();
    let jbyte_array = env.new_byte_array(bytes.len() as i32).ok()?;
    env.set_byte_array_region(&jbyte_array, 0, unsafe {  // THÊM & trước jbyte_array
        std::slice::from_raw_parts(bytes.as_ptr() as *const i8, bytes.len())
    }).ok()?;

    let uuid_result = env.call_static_method(
        uuid_class,
        "nameUUIDFromBytes",
        "([B)Ljava/util/UUID;",
        &[JValue::Object(&jbyte_array.into())]
    ).ok()?;
    
    let uuid_obj = uuid_result.l().ok()?;

    let string_result = env.call_method(
        uuid_obj,
        "toString",
        "()Ljava/lang/String;",
        &[]
    ).ok()?;
    
    let jstring = string_result.l().ok()?;
    let rust_string: String = env.get_string((&jstring).into()).ok()?.into();
    Some(rust_string)
}

pub fn calc_md5(s: &str) -> String {
    format!("{:x}", md5::compute(s.as_bytes()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_random_string() {
        let s = random_string(10);
        assert_eq!(s.len(), 10);
    }

    #[test]
    fn test_string_to_offset() {
        assert_eq!(string_to_offset("0x1000"), 0x1000);
        assert_eq!(string_to_offset("1000"), 0x1000);
    }

    #[test]
    fn test_calc_md5() {
        let hash = calc_md5("Hello, World!");
        assert_eq!(hash, "65a8e27d8879283831b664bd8b7f0ad4");
    }
}
