/*
 *  @date   : 2025/11/17
 *  https://github.com/traitimtrongvag/And64InlineHook-rs
 *  Rust port
 */

use std::ptr;
use std::sync::atomic::{AtomicI32, Ordering};

const A64_MAX_INSTRUCTIONS: usize = 5;
const A64_MAX_REFERENCES: usize = A64_MAX_INSTRUCTIONS * 2;
const A64_MAX_BACKUPS: usize = 256;
const A64_NOP: u32 = 0xd503201f;
const PAGE_SIZE: usize = 4096;

macro_rules! log_info {
    ($($arg:tt)*) => {
        #[cfg(all(target_os = "android", debug_assertions))] 
        {
            use log::info;
            info!($($arg)*);
        }
        #[cfg(not(target_os = "android"))]
        {
            #[cfg(debug_assertions)] 
            println!($($arg)*);
        }
    };
}

macro_rules! log_error {
    ($($arg:tt)*) => {
        #[cfg(target_os = "android")]
        {
            use log::error;
            error!($($arg)*);
        }
        #[cfg(not(target_os = "android"))]
        eprintln!($($arg)*);
    };
}

#[repr(C, align(4096))]
struct InsnsPool {
    data: [[u32; A64_MAX_INSTRUCTIONS * 10]; A64_MAX_BACKUPS],
}

static mut INSNS_POOL: InsnsPool = InsnsPool {
    data: [[0; A64_MAX_INSTRUCTIONS * 10]; A64_MAX_BACKUPS],
};

static POOL_INDEX: AtomicI32 = AtomicI32::new(-1);

#[derive(Debug, Clone, Copy)]
struct FixInfo {
    bp: *mut u32,
    ls: u32,
    ad: u32,
}

impl Default for FixInfo {
    fn default() -> Self {
        Self {
            bp: ptr::null_mut(),
            ls: 0,
            ad: 0xffffffff,
        }
    }
}

#[derive(Clone, Copy)]
union InsValue {
    insu: u64,
    ins: i64,
    insp: *mut u32,
}

struct InsnsInfo {
    val: InsValue,
    fmap: [FixInfo; A64_MAX_REFERENCES],
}

impl InsnsInfo {
    fn new() -> Self {
        Self {
            val: InsValue { ins: 0 },
            fmap: [FixInfo::default(); A64_MAX_REFERENCES],
        }
    }
}

struct Context {
    basep: i64,
    endp: i64,
    dat: [InsnsInfo; A64_MAX_INSTRUCTIONS],
}

impl Context {
    fn new(inp: *const u32, count: i32) -> Self {
        Self {
            basep: inp as i64,
            endp: unsafe { inp.add(count as usize) } as i64,
            dat: [
                InsnsInfo::new(),
                InsnsInfo::new(),
                InsnsInfo::new(),
                InsnsInfo::new(),
                InsnsInfo::new(),
            ],
        }
    }

    #[inline]
    fn is_in_fixing_range(&self, absolute_addr: i64) -> bool {
        absolute_addr >= self.basep && absolute_addr < self.endp
    }

    #[inline]
    fn get_ref_ins_index(&self, absolute_addr: i64) -> isize {
        ((absolute_addr - self.basep) / 4) as isize
    }

    #[inline]
    fn get_and_set_current_index(&mut self, inp: *const u32, outp: *mut u32) -> isize {
        let current_idx = self.get_ref_ins_index(inp as i64);
        self.dat[current_idx as usize].val.insp = outp;
        current_idx
    }

    #[inline]
    fn reset_current_ins(&mut self, idx: isize, outp: *mut u32) {
        self.dat[idx as usize].val.insp = outp;
    }

    fn insert_fix_map(&mut self, idx: isize, bp: *mut u32, ls: u32, ad: u32) {
        for f in &mut self.dat[idx as usize].fmap {
            if f.bp.is_null() {
                f.bp = bp;
                f.ls = ls;
                f.ad = ad;
                return;
            }
        }
    }

    fn process_fix_map(&mut self, idx: isize) {
        unsafe {
            let ins = self.dat[idx as usize].val.ins;
            for f in &mut self.dat[idx as usize].fmap {
                if f.bp.is_null() {
                    break;
                }
                let offset = ((ins - f.bp as i64) >> 2) as i32;
                *f.bp |= ((offset << f.ls as i32) as u32) & f.ad;
                f.bp = ptr::null_mut();
            }
        }
    }
}

unsafe fn flush_cache(addr: *const u32, size: usize) {
    #[cfg(target_os = "android")]
    {
        extern "C" {
            fn __clear_cache(start: *const u8, end: *const u8);
        }
        __clear_cache(addr as *const u8, (addr as usize + size) as *const u8);
    }
    
    #[cfg(not(target_os = "android"))]
    {
        let end = (addr as usize + size) as *const u32;
        let mut p = addr;
        while p < end {
            std::arch::asm!(
                "dc civac, {0}",
                in(reg) p,
                options(nostack)
            );
            p = p.add(16); // Cache line size
        }
        std::arch::asm!(
            "dsb sy",
            "isb",
            options(nostack)
        );
    }
}


unsafe fn make_rwx(addr: *mut u32, size: usize) -> i32 {
    let page_addr = (addr as usize & !(PAGE_SIZE - 1)) as *mut libc::c_void;
    let page_size = if (addr as usize + size) & (PAGE_SIZE - 1) != (addr as usize) & (PAGE_SIZE - 1) {
        ((size + PAGE_SIZE - 1) & !(PAGE_SIZE - 1)) + PAGE_SIZE
    } else {
        (size + PAGE_SIZE - 1) & !(PAGE_SIZE - 1)
    };

    libc::mprotect(
        page_addr,
        page_size,
        libc::PROT_READ | libc::PROT_WRITE | libc::PROT_EXEC,
    )
}

unsafe fn fix_branch_imm(
    inp: &mut *const u32,
    outp: &mut *mut u32,
    ctx: &mut Context,
) -> bool {
    const MASK: u32 = 0xfc000000;
    const RMASK: u32 = 0x03ffffff;
    const OP_B: u32 = 0x14000000;
    const OP_BL: u32 = 0x94000000;

    let ins = **inp;
    let opc = ins & MASK;

    match opc {
        OP_B | OP_BL => {
            let current_idx = ctx.get_and_set_current_index(*inp, *outp);
            let absolute_addr = (*inp as i64) + (((ins << 6) as i32) >> 4) as i64;
            let mut new_pc_offset = (absolute_addr - *outp as i64) >> 2;
            let special_fix_type = ctx.is_in_fixing_range(absolute_addr);

            if !special_fix_type && new_pc_offset.abs() >= (RMASK >> 1) as i64 {
                let b_aligned = (*outp.add(2) as u64 & 7) == 0;
                if opc == OP_B {
                    if !b_aligned {
                        **outp = A64_NOP;
                        *outp = outp.add(1);
                        ctx.reset_current_ins(current_idx, *outp);
                    }
                    ptr::write(*outp, 0x58000051); // LDR X17, #0x8
                    ptr::write(outp.add(1), 0xd61f0220); // BR X17
                    ptr::copy_nonoverlapping(&absolute_addr as *const i64 as *const u32, outp.add(2), 2);
                    *outp = outp.add(4);
                } else {
                    if b_aligned {
                        **outp = A64_NOP;
                        *outp = outp.add(1);
                        ctx.reset_current_ins(current_idx, *outp);
                    }
                    ptr::write(*outp, 0x58000071); // LDR X17, #12
                    ptr::write(outp.add(1), 0x1000009e); // ADR X30, #16
                    ptr::write(outp.add(2), 0xd61f0220); // BR X17
                    ptr::copy_nonoverlapping(&absolute_addr as *const i64 as *const u32, outp.add(3), 2);
                    *outp = outp.add(5);
                }
            } else {
                if special_fix_type {
                    let ref_idx = ctx.get_ref_ins_index(absolute_addr);
                    if ref_idx <= current_idx {
                        new_pc_offset = (ctx.dat[ref_idx as usize].val.ins - *outp as i64) >> 2;
                    } else {
                        ctx.insert_fix_map(ref_idx, *outp, 0, RMASK);
                        new_pc_offset = 0;
                    }
                }

                **outp = opc | ((new_pc_offset as u32) & !MASK);
                *outp = outp.add(1);
            }

            *inp = inp.add(1);
            ctx.process_fix_map(current_idx);
            true
        }
        _ => false,
    }
}

unsafe fn fix_cond_comp_test_branch(
    inp: &mut *const u32,
    outp: &mut *mut u32,
    ctx: &mut Context,
) -> bool {
    const LMASK01: u32 = 0xff00001f;
    const MASK0: u32 = 0xff000010;
    const OP_BC: u32 = 0x54000000;
    const MASK1: u32 = 0x7f000000;
    const OP_CBZ: u32 = 0x34000000;
    const OP_CBNZ: u32 = 0x35000000;
    const LMASK2: u32 = 0xfff8001f;
    const MASK2: u32 = 0x7f000000;
    const OP_TBZ: u32 = 0x36000000;
    const OP_TBNZ: u32 = 0x37000000;
    const LSB: u32 = 5;

    let ins = **inp;
    let mut lmask = LMASK01;

    if (ins & MASK0) != OP_BC {
        let mut opc = ins & MASK1;
        if opc != OP_CBZ && opc != OP_CBNZ {
            opc = ins & MASK2;
            if opc != OP_TBZ && opc != OP_TBNZ {
                return false;
            }
            lmask = LMASK2;
        }
    }

    let msb = (!lmask).leading_zeros();
    let current_idx = ctx.get_and_set_current_index(*inp, *outp);
    let absolute_addr = (*inp as i64) + ((((ins & !lmask) << msb) as i32) >> (LSB - 2 + msb)) as i64;
    let mut new_pc_offset = (absolute_addr - *outp as i64) >> 2;
    let special_fix_type = ctx.is_in_fixing_range(absolute_addr);

    if !special_fix_type && new_pc_offset.abs() >= (!lmask >> (LSB + 1)) as i64 {
        if (*outp.add(4) as u64 & 7) != 0 {
            **outp = A64_NOP;
            *outp = outp.add(1);
            ctx.reset_current_ins(current_idx, *outp);
        }

        **outp = (((8 >> 2) << LSB) & !lmask) | (ins & lmask);
        ptr::write(outp.add(1), 0x14000005);
        ptr::write(outp.add(2), 0x58000051);
        ptr::write(outp.add(3), 0xd61f0220);
        ptr::copy_nonoverlapping(&absolute_addr as *const i64 as *const u32, outp.add(4), 2);
        *outp = outp.add(6);
    } else {
        if special_fix_type {
            let ref_idx = ctx.get_ref_ins_index(absolute_addr);
            if ref_idx <= current_idx {
                new_pc_offset = (ctx.dat[ref_idx as usize].val.ins - *outp as i64) >> 2;
            } else {
                ctx.insert_fix_map(ref_idx, *outp, LSB, !lmask);
                new_pc_offset = 0;
            }
        }

        **outp = ((new_pc_offset as u32) << LSB & !lmask) | (ins & lmask);
        *outp = outp.add(1);
    }

    *inp = inp.add(1);
    ctx.process_fix_map(current_idx);
    true
}

unsafe fn fix_loadlit(
    inp: &mut *const u32,
    outp: &mut *mut u32,
    ctx: &mut Context,
) -> bool {
    let ins = **inp;

    if (ins & 0xff000000) == 0xd8000000 {
        let index = ctx.get_and_set_current_index(*inp, *outp);
        ctx.process_fix_map(index);
        *inp = inp.add(1);
        return true;
    }

    const MSB: u32 = 8;
    const LSB: u32 = 5;
    const MASK_30: u32 = 0x40000000;
    const MASK_31: u32 = 0x80000000;
    const LMASK: u32 = 0xff00001f;
    const MASK_LDR: u32 = 0xbf000000;
    const OP_LDR: u32 = 0x18000000;
    const MASK_LDRV: u32 = 0x3f000000;
    const OP_LDRV: u32 = 0x1c000000;
    const MASK_LDRSW: u32 = 0xff000000;
    const OP_LDRSW: u32 = 0x98000000;

    let mut mask = MASK_LDR;
    let mut faligned = if (ins & MASK_30) != 0 { 7 } else { 3 };

    if (ins & MASK_LDR) != OP_LDR {
        mask = MASK_LDRV;
        if faligned != 7 {
            faligned = if (ins & MASK_31) != 0 { 15 } else { 3 };
        }
        if (ins & MASK_LDRV) != OP_LDRV {
            if (ins & MASK_LDRSW) != OP_LDRSW {
                return false;
            }
            mask = MASK_LDRSW;
            faligned = 7;
        }
    }

    let current_idx = ctx.get_and_set_current_index(*inp, *outp);
    let absolute_addr = (*inp as i64) + ((((ins << MSB) as i32) >> (MSB + LSB - 2)) & !3) as i64;
    let mut new_pc_offset = (absolute_addr - *outp as i64) >> 2;
    let special_fix_type = ctx.is_in_fixing_range(absolute_addr);

    if special_fix_type || (new_pc_offset.abs() + ((faligned + 1 - 4) / 4) as i64) >= (!LMASK >> (LSB + 1)) as i64 {
        while (*outp.add(2) as usize & faligned) != 0 {
            **outp = A64_NOP;
            *outp = outp.add(1);
        }
        ctx.reset_current_ins(current_idx, *outp);

        let ns = ((faligned + 1) / 4) as usize;
        **outp = (((8 >> 2) << LSB) & !mask) | (ins & LMASK);
        ptr::write(outp.add(1), 0x14000001 + ns as u32);
        ptr::copy_nonoverlapping(absolute_addr as *const u32, outp.add(2), ns);
        *outp = outp.add(2 + ns);
    } else {
        let faligned_shift = faligned >> 2;
        while (new_pc_offset & faligned_shift as i64) != 0 {
            **outp = A64_NOP;
            *outp = outp.add(1);
            new_pc_offset = (absolute_addr - *outp as i64) >> 2;
        }
        ctx.reset_current_ins(current_idx, *outp);

        **outp = ((new_pc_offset as u32) << LSB & !mask) | (ins & LMASK);
        *outp = outp.add(1);
    }

    *inp = inp.add(1);
    ctx.process_fix_map(current_idx);
    true
}

unsafe fn fix_pcreladdr(
    inp: &mut *const u32,
    outp: &mut *mut u32,
    ctx: &mut Context,
) -> bool {
    const MSB: u32 = 8;
    const LSB: u32 = 5;
    const MASK: u32 = 0x9f000000;
    const RMASK: u32 = 0x0000001f;
    const LMASK: u32 = 0xff00001f;
    const FMASK: u32 = 0x00ffffff;
    const MAX_VAL: u32 = 0x001fffff;
    const OP_ADR: u32 = 0x10000000;
    const OP_ADRP: u32 = 0x90000000;

    let ins = **inp;

    match ins & MASK {
        OP_ADR => {
            let current_idx = ctx.get_and_set_current_index(*inp, *outp);
            let lsb_bytes = ((ins << 1) >> 30) as i64;
            let absolute_addr = (*inp as i64) + (((((ins << MSB) as i32) >> (MSB + LSB - 2)) & !3) as i64 | lsb_bytes);
            let mut new_pc_offset = absolute_addr - *outp as i64;
            let special_fix_type = ctx.is_in_fixing_range(absolute_addr);

            if !special_fix_type && new_pc_offset.abs() >= (MAX_VAL >> 1) as i64 {
                if (*outp.add(2) as u64 & 7) != 0 {
                    **outp = A64_NOP;
                    *outp = outp.add(1);
                    ctx.reset_current_ins(current_idx, *outp);
                }

                **outp = 0x58000000 | (((8 >> 2) << LSB) & !MASK) | (ins & RMASK);
                ptr::write(outp.add(1), 0x14000003);
                ptr::copy_nonoverlapping(&absolute_addr as *const i64 as *const u32, outp.add(2), 2);
                *outp = outp.add(4);
            } else {
                if special_fix_type {
                    let ref_idx = ctx.get_ref_ins_index(absolute_addr & !3);
                    if ref_idx <= current_idx {
                        new_pc_offset = ctx.dat[ref_idx as usize].val.ins - *outp as i64;
                    } else {
                        ctx.insert_fix_map(ref_idx, *outp, LSB, FMASK);
                        new_pc_offset = 0;
                    }
                }

                **outp = ((new_pc_offset as u32) << (LSB - 2)) & FMASK | (ins & LMASK);
                *outp = outp.add(1);
            }

            *inp = inp.add(1);
            ctx.process_fix_map(current_idx);
            true
        }
        OP_ADRP => {
            let current_idx = ctx.get_and_set_current_index(*inp, *outp);
            let lsb_bytes = ((ins << 1) >> 30) as i32;
            let absolute_addr = ((*inp as i64) & !0xfff) + ((((((ins << MSB) as i32) >> (MSB + LSB - 2)) & !3) | lsb_bytes) as i64) << 12;

            log_info!("ins = 0x{:08X}, pc = {:p}, abs_addr = {:p}", ins, *inp, absolute_addr as *const u8);

			if ctx.is_in_fixing_range(absolute_addr) {
			    log_error!("ADRP pointing to hook region not fully supported!");
			    // Fallback: load absolute address
			    if (*outp.add(2) as u64 & 7) != 0 {
			        **outp = A64_NOP;
			        *outp = outp.add(1);
			        ctx.reset_current_ins(current_idx, *outp);
			    }
			    **outp = 0x58000000 | (((8 >> 2) << LSB) & !MASK) | (ins & RMASK);
			    ptr::write(outp.add(1), 0x14000003);
			    ptr::copy_nonoverlapping(&absolute_addr as *const i64 as *const u32, outp.add(2), 2);
			    *outp = outp.add(4);
			} else {
                if (*outp.add(2) as u64 & 7) != 0 {
                    **outp = A64_NOP;
                    *outp = outp.add(1);
                    ctx.reset_current_ins(current_idx, *outp);
                }

                **outp = 0x58000000 | (((8 >> 2) << LSB) & !MASK) | (ins & RMASK);
                ptr::write(outp.add(1), 0x14000003);
                ptr::copy_nonoverlapping(&absolute_addr as *const i64 as *const u32, outp.add(2), 2);
                *outp = outp.add(4);
            }

            *inp = inp.add(1);
            ctx.process_fix_map(current_idx);
            true
        }
        _ => false,
    }
}

unsafe fn fix_instructions(mut inp: *const u32, count: i32, mut outp: *mut u32) {
    let outp_base = outp;
    let mut ctx = Context::new(inp, count);
    let mut remaining = count;

    while remaining > 0 {
        if fix_branch_imm(&mut inp, &mut outp, &mut ctx) {
            remaining -= 1;
            continue;
        }
        if fix_cond_comp_test_branch(&mut inp, &mut outp, &mut ctx) {
            remaining -= 1;
            continue;
        }
        if fix_loadlit(&mut inp, &mut outp, &mut ctx) {
            remaining -= 1;
            continue;
        }
        if fix_pcreladdr(&mut inp, &mut outp, &mut ctx) {
            remaining -= 1;
            continue;
        }

        let index = ctx.get_and_set_current_index(inp, outp);
        ctx.process_fix_map(index);
        *outp = *inp;
        outp = outp.add(1);
        inp = inp.add(1);
        remaining -= 1;
    }

    const MASK: u64 = 0x03ffffff;
    let callback = inp as i64;
    let pc_offset = (callback - outp as i64) >> 2;

    if pc_offset.abs() >= (MASK >> 1) as i64 {
        if (outp.add(2) as u64 & 7) != 0 {
            *outp = A64_NOP;
            outp = outp.add(1);
        }
        ptr::write(outp, 0x58000051);
        ptr::write(outp.add(1), 0xd61f0220);
        ptr::copy_nonoverlapping(&callback as *const i64 as *const u32, outp.add(2), 2);
        outp = outp.add(4);
    } else {
        *outp = 0x14000000 | ((pc_offset as u32) & MASK as u32);
        outp = outp.add(1);
    }

    let total = (outp as usize - outp_base as usize);
    flush_cache(outp_base, total);
}

fn fast_allocate_trampoline() -> Option<*mut u32> {
    loop {
        let idx = POOL_INDEX.load(Ordering::Acquire);
        if idx >= A64_MAX_BACKUPS as i32 - 1 {
            return None;
        }
        if POOL_INDEX.compare_exchange(
            idx,
            idx + 1,
            Ordering::Release,
            Ordering::Acquire
        ).is_ok() {
            unsafe {
                return Some(INSNS_POOL.data[(idx + 1) as usize].as_mut_ptr());
            }
        }
    }
}

/// Hook a function with custom RWX memory region
pub unsafe fn a64_hook_function_v(
    symbol: *mut u32,
    replace: *const u32,
    rwx: *mut u32,
    rwx_size: usize,
) -> Option<*mut u32> {
    const MASK: u64 = 0x03ffffff;

    let mut trampoline = rwx;
    let original = symbol;

    let pc_offset = (replace as i64 - symbol as i64) >> 2;
    
    if pc_offset.abs() >= (MASK >> 1) as i64 {
        let count = if (original.add(2) as u64 & 7) != 0 { 5 } else { 4 };
        
        if !trampoline.is_null() {
            if rwx_size < count * 10 {
                log_error!("rwx size is too small to hold {} bytes backup instructions!", count * 10);
                return None;
            }
            fix_instructions(original, count as i32, trampoline);
        }

        if make_rwx(original, 5 * 4) == 0 {
            let mut write_ptr = original;
            if count == 5 {
                *write_ptr = A64_NOP;
                write_ptr = write_ptr.add(1);
            }
            ptr::write(write_ptr, 0x58000051);
            ptr::write(write_ptr.add(1), 0xd61f0220);
            ptr::copy_nonoverlapping(&(replace as i64) as *const i64 as *const u32, write_ptr.add(2), 2);
            flush_cache(symbol, 5 * 4);

            log_info!("inline hook {:p}->{:p} successfully! {} bytes overwritten", symbol, replace, 5 * 4);
        } else {
            log_error!("mprotect failed, p = {:p}, size = {}", original, 5 * 4);
            trampoline = ptr::null_mut();
        }
    } else {
        if !trampoline.is_null() {
            if rwx_size < 10 {
                log_error!("rwx size is too small to hold {} bytes backup instructions!", 10);
                return None;
            }
            fix_instructions(original, 1, trampoline);
        }

        if make_rwx(original, 4) == 0 {
            *original = 0x14000000 | ((pc_offset as u32) & MASK as u32);
            flush_cache(symbol, 4);

            log_info!("inline hook {:p}->{:p} successfully! {} bytes overwritten", symbol, replace, 4);
        } else {
            log_error!("mprotect failed, p = {:p}, size = {}", original, 4);
            trampoline = ptr::null_mut();
        }
    }

    if trampoline.is_null() {
        None
    } else {
        Some(trampoline)
    }
}

/// Hook a function and optionally return the trampoline
pub unsafe fn a64_hook_function(
    symbol: *mut u32,
    replace: *const u32,
) -> Option<*mut u32> {
	if (symbol as usize) & 3 != 0 {
        log_error!("symbol pointer not 4-byte aligned!");
        return None;
    }
    let trampoline = fast_allocate_trampoline()?;
    
    // Fix Android 10+ where .text segment is read-only by default
    make_rwx(symbol, 5 * 4);

    a64_hook_function_v(symbol, replace, trampoline, A64_MAX_INSTRUCTIONS * 10)
}

/// Initialize the instruction pool
pub unsafe fn init_hook_pool() {
    make_rwx(
        INSNS_POOL.data.as_mut_ptr() as *mut u32,
        std::mem::size_of::<InsnsPool>(),
    );
    log_info!("insns pool initialized.");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_context_creation() {
        let dummy: [u32; 5] = [0; 5];
        let ctx = Context::new(dummy.as_ptr(), 5);
        assert_eq!(ctx.basep, dummy.as_ptr() as i64);
    }

    #[test]
    fn test_pool_allocation() {
        unsafe {
            init_hook_pool();
            POOL_INDEX.store(-1, Ordering::Release);
            let trampoline1 = fast_allocate_trampoline();
            assert!(trampoline1.is_some());
            
            let trampoline2 = fast_allocate_trampoline();
            assert!(trampoline2.is_some());
            assert_ne!(trampoline1.unwrap(), trampoline2.unwrap());
        }
    }

    #[test]
    fn test_fix_info_default() {
        let fix = FixInfo::default();
        assert!(fix.bp.is_null());
        assert_eq!(fix.ls, 0);
        assert_eq!(fix.ad, 0xffffffff);
    }

    #[test]
    fn test_insns_info_creation() {
        let info = InsnsInfo::new();
        unsafe {
            assert_eq!(info.val.ins, 0);
        }
        assert!(info.fmap[0].bp.is_null());
    }

    #[test]
    fn test_context_range_check() {
        let dummy: [u32; 5] = [0x14000000, 0x14000001, 0x14000002, 0x14000003, 0x14000004];
        let ctx = Context::new(dummy.as_ptr(), 5);
        
        let addr_in_range = dummy.as_ptr() as i64 + 4;
        assert!(ctx.is_in_fixing_range(addr_in_range));
        
        let addr_out_range = dummy.as_ptr() as i64 + 1000;
        assert!(!ctx.is_in_fixing_range(addr_out_range));
    }

    #[test]
    fn test_get_ref_ins_index() {
        let dummy: [u32; 5] = [0; 5];
        let ctx = Context::new(dummy.as_ptr(), 5);
        
        let idx = ctx.get_ref_ins_index(dummy.as_ptr() as i64);
        assert_eq!(idx, 0);
        
        let idx2 = ctx.get_ref_ins_index((dummy.as_ptr() as i64) + 8);
        assert_eq!(idx2, 2);
    }
}

// Export for C FFI compatibility
#[no_mangle]
pub unsafe extern "C" fn A64HookFunction(
    symbol: *mut u32,
    replace: *const u32,
    result: *mut *mut u32,
) {
    if result.is_null() {
        a64_hook_function(symbol, replace);
        return;
    }

    match a64_hook_function(symbol, replace) {
        Some(trampoline) => {
            *result = trampoline;
        }
        None => {
            *result = ptr::null_mut();
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn A64HookFunctionV(
    symbol: *mut u32,
    replace: *const u32,
    rwx: *mut u32,
    rwx_size: usize,
) -> *mut u32 {
    match a64_hook_function_v(symbol, replace, rwx, rwx_size) {
        Some(trampoline) => trampoline,
        None => ptr::null_mut(),
    }
}