use crate::includes::kittymemory::{self, ProcMap};

pub struct Remapper;

impl Remapper {
    // Thin wrapper — kittymemory::get_maps_by_name is the single source of
    // truth for /proc/self/maps parsing. ProcMapInfo duplicated that struct
    // with near-identical fields and a separate parser that could silently
    // diverge on bug fixes.
    pub fn list_modules_with_name(name: &str) -> Vec<ProcMap> {
        kittymemory::get_maps_by_name(name)
    }

    pub fn remap_simple(libname: &str) {
        for info in kittymemory::get_maps_by_name(libname) {
            unsafe {
                let addr = info.start_address as *mut libc::c_void;
                let size = (info.end_address - info.start_address) as usize;

                let new_mem = libc::malloc(size);
                if new_mem.is_null() {
                    continue;
                }

                std::ptr::copy_nonoverlapping(addr as *const u8, new_mem as *mut u8, size);

                if libc::mprotect(addr, size, libc::PROT_READ | libc::PROT_WRITE | libc::PROT_EXEC) != 0 {
                    libc::free(new_mem);
                    continue;
                }

                std::ptr::copy_nonoverlapping(new_mem as *const u8, addr as *mut u8, size);

                // Restore original protection flags from the parsed map entry
                libc::mprotect(addr, size, info.protection);

                libc::free(new_mem);
            }
        }
    }
}