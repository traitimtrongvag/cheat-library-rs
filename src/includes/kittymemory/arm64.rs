
pub mod kitty_arm64 {
    pub fn bit_from(insn: u32, pos: u32) -> i32 {
        (((1u32 << pos) & insn) >> pos) as i32
    }

    pub fn bits_from(insn: u32, pos: u32, l: u32) -> i32 {
        ((insn >> pos) & ((1u32 << l) - 1)) as i32
    }

    pub fn is_insn_adr(insn: u32) -> bool {
        (insn & 0x9F00_0000) == 0x10_00_0000
    }

    pub fn is_insn_adrp(insn: u32) -> bool {
        (insn & 0x9F00_0000) == 0x90_00_0000
    }

    pub fn decode_adr_imm(insn: u32, imm: &mut i64) -> bool {
        if is_insn_adr(insn) || is_insn_adrp(insn) {
            let mut imm_val: i64 = (bits_from(insn, 5, 19) as i64) << 2;
            imm_val |= bits_from(insn, 29, 2) as i64;

            if is_insn_adrp(insn) {
                let msbt: u64 = ((imm_val >> 20) & 1) as u64;

                let _v: u64 = (imm_val as u64) << 12;

                const BIT33: u64 = 1u64 << 32; // note: C++ used <<33 then or, we'll follow same outcome
                let top_mask = ((1u64 << 32) - msbt) << 33;
                let imm_val_shifted = ( ( (bits_from(insn, 5, 19) as i64) << 2 ) as u64 ) << 12;
                let result = top_mask | imm_val_shifted;
                *imm = result as i64;
            } else {
                if (imm_val & (1 << (21 - 1))) != 0 {
                    imm_val |= !((1i64 << 21) - 1);
                }
                *imm = imm_val;
            }

            return true;
        }
        false
    }

    /*
     * Decode ADD/SUB (immediate) imm12 with optional shift of 12 (LSL #12)
     *
     * C++:
     * int32_t decode_addsub_imm(uint32_t insn)
     */
    pub fn decode_addsub_imm(insn: u32) -> i32 {
        let mut imm12: i32 = bits_from(insn, 10, 12);
        let shift = bit_from(insn, 22) == 1;
        if shift {
            imm12 <<= 12;
        }
        imm12
    }

    pub fn is_insn_ld(insn: u32) -> bool {
        bit_from(insn, 22) == 1
    }

    pub fn is_insn_ldst(insn: u32) -> bool {
        (insn & 0x0A00_0000) == 0x08_00_0000
    }

    pub fn is_insn_ldst_uimm(insn: u32) -> bool {
        (insn & 0x3B00_0000) == 0x39_00_0000
    }

    pub fn decode_ldrstr_uimm(insn: u32, imm12: &mut i32) -> bool {
        if is_insn_ldst_uimm(insn) {
            *imm12 = bits_from(insn, 10, 12);
            let size = bits_from(insn, 30, 2) as u32;
            *imm12 <<= size;
            return true;
        }
        false
    }

    pub use bit_from as bit_from_u32;
    pub use bits_from as bits_from_u32;
    pub use is_insn_adr as is_adr;
    pub use is_insn_adrp as is_adrp;
}

pub use kitty_arm64::{
    bit_from as bit_from,
    bits_from as bits_from,
    decode_addsub_imm as decode_addsub_imm,
    decode_adr_imm as decode_adr_imm,
    decode_ldrstr_uimm as decode_ldrstr_uimm,
    is_insn_adr as is_insn_adr,
    is_insn_adrp as is_insn_adrp,
    is_insn_ld as is_insn_ld,
    is_insn_ldst as is_insn_ldst,
    is_insn_ldst_uimm as is_insn_ldst_uimm,
};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bit_bits() {
        assert_eq!(bit_from(0b1010, 1), 1); // second bit is 1
        assert_eq!(bits_from(0b1111000, 3, 4), 0b1111);
    }

}