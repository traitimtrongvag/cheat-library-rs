
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
            if is_insn_adrp(insn) {
                // ADRP: imm = sign_extend(immhi:immlo, 21) << 12
                let immlo = bits_from(insn, 29, 2) as i64;
                let immhi = bits_from(insn, 5, 19) as i64;
                let imm21 = (immhi << 2) | immlo;
                // sign-extend from bit 20
                let imm21_sx = if imm21 & (1 << 20) != 0 {
                    imm21 | !((1i64 << 21) - 1)
                } else {
                    imm21
                };
                *imm = imm21_sx << 12;
            } else {
                // ADR: imm = sign_extend(immhi:immlo, 21)
                let immlo = bits_from(insn, 29, 2) as i64;
                let immhi = bits_from(insn, 5, 19) as i64;
                let imm21 = (immhi << 2) | immlo;
                *imm = if imm21 & (1 << 20) != 0 {
                    imm21 | !((1i64 << 21) - 1)
                } else {
                    imm21
                };
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
    bit_from,
    bits_from,
    decode_addsub_imm,
    decode_adr_imm,
    decode_ldrstr_uimm,
    is_insn_adr,
    is_insn_adrp,
    is_insn_ld,
    is_insn_ldst,
    is_insn_ldst_uimm,
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