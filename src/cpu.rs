use crate::{mmu::Mmu, ppu::Ppu};

pub const PREFIX: u8 = 0xcb;

pub const INSTRUCTIONS: [(fn(&mut Cpu), u8, &'static str); 256] = [
    // 0x0 opcodes
    (nop, 4, "NOP"),
    (ld_bc_u16, 12, "LD BC, u16"),
    (ld_mbc_a, 8, "LD [BC], A"),
    (inc_bc, 8, "INC BC"),
    (inc_b, 4, "INC B"),
    (dec_b, 4, "DEC B"),
    (ld_b_u8, 8, "LD B, u8"),
    (rlca, 4, "RLCA"),
    (ld_mu16_sp, 20, "LD [u16], SP"),
    (add_hl_bc, 8, "ADD HL, BC"),
    (noimpl, 4, "! UNIMP !"),
    (noimpl, 4, "! UNIMP !"),
    (inc_c, 4, "INC C"),
    (dec_c, 4, "DEC C"),
    (ld_c_u8, 8, "LD C, u8"),
    (noimpl, 4, "! UNIMP !"),
    // 0x1 opcodes
    (noimpl, 4, "! UNIMP !"),
    (ld_de_u16, 12, "LD DE, u16"),
    (noimpl, 4, "! UNIMP !"),
    (inc_de, 8, "INC DE"),
    (noimpl, 4, "! UNIMP !"),
    (dec_d, 4, "DEC D"),
    (noimpl, 4, "! UNIMP !"),
    (rla, 4, "RLA"),
    (jr_i8, 12, "JR i8"),
    (noimpl, 4, "! UNIMP !"),
    (ld_a_mde, 8, "LD A, [DE]"),
    (noimpl, 4, "! UNIMP !"),
    (noimpl, 4, "! UNIMP !"),
    (dec_e, 4, "DEC E"),
    (ld_e_u8, 8, "LD E, u8"),
    (noimpl, 4, "! UNIMP !"),
    // 0x2 opcodes
    (jr_nz_i8, 8, "JR NZ, i8"),
    (ld_hl_u16, 12, "LD HL, u16"),
    (ld_mhli_a, 8, "LD [HL+], A"),
    (inc_hl, 8, "INC HL"),
    (inc_h, 4, "INC H"),
    (noimpl, 4, "! UNIMP !"),
    (noimpl, 4, "! UNIMP !"),
    (noimpl, 4, "! UNIMP !"),
    (jr_z_i8, 8, "JR Z, i8"),
    (noimpl, 4, "! UNIMP !"),
    (noimpl, 4, "! UNIMP !"),
    (noimpl, 4, "! UNIMP !"),
    (noimpl, 4, "! UNIMP !"),
    (noimpl, 4, "! UNIMP !"),
    (ld_l_u8, 8, "LD L, u8"),
    (noimpl, 4, "! UNIMP !"),
    // 0x3 opcodes
    (noimpl, 4, "! UNIMP !"),
    (ld_sp_u16, 12, "LD SP, u16"),
    (ld_mhld_a, 8, "LD [HL-], A"),
    (noimpl, 4, "! UNIMP !"),
    (noimpl, 4, "! UNIMP !"),
    (noimpl, 4, "! UNIMP !"),
    (noimpl, 4, "! UNIMP !"),
    (noimpl, 4, "! UNIMP !"),
    (noimpl, 4, "! UNIMP !"),
    (noimpl, 4, "! UNIMP !"),
    (noimpl, 4, "! UNIMP !"),
    (noimpl, 4, "! UNIMP !"),
    (noimpl, 4, "! UNIMP !"),
    (dec_a, 4, "DEC A"),
    (ld_a_u8, 8, "LD A, u8"),
    (noimpl, 4, "! UNIMP !"),
    // 0x4 opcodes
    (noimpl, 4, "! UNIMP !"),
    (noimpl, 4, "! UNIMP !"),
    (noimpl, 4, "! UNIMP !"),
    (noimpl, 4, "! UNIMP !"),
    (ld_b_h, 4, "LD B, H"),
    (noimpl, 4, "! UNIMP !"),
    (noimpl, 4, "! UNIMP !"),
    (noimpl, 4, "! UNIMP !"),
    (noimpl, 4, "! UNIMP !"),
    (noimpl, 4, "! UNIMP !"),
    (noimpl, 4, "! UNIMP !"),
    (noimpl, 4, "! UNIMP !"),
    (noimpl, 4, "! UNIMP !"),
    (noimpl, 4, "! UNIMP !"),
    (noimpl, 4, "! UNIMP !"),
    (ld_c_a, 4, "LD C, A"),
    // 0x5 opcodes
    (noimpl, 4, "! UNIMP !"),
    (noimpl, 4, "! UNIMP !"),
    (noimpl, 4, "! UNIMP !"),
    (noimpl, 4, "! UNIMP !"),
    (noimpl, 4, "! UNIMP !"),
    (noimpl, 4, "! UNIMP !"),
    (noimpl, 4, "! UNIMP !"),
    (ld_d_a, 4, "LD D, A"),
    (noimpl, 4, "! UNIMP !"),
    (noimpl, 4, "! UNIMP !"),
    (noimpl, 4, "! UNIMP !"),
    (noimpl, 4, "! UNIMP !"),
    (noimpl, 4, "! UNIMP !"),
    (noimpl, 4, "! UNIMP !"),
    (noimpl, 4, "! UNIMP !"),
    (noimpl, 4, "! UNIMP !"),
    // 0x6 opcodes
    (noimpl, 4, "! UNIMP !"),
    (noimpl, 4, "! UNIMP !"),
    (noimpl, 4, "! UNIMP !"),
    (noimpl, 4, "! UNIMP !"),
    (noimpl, 4, "! UNIMP !"),
    (noimpl, 4, "! UNIMP !"),
    (noimpl, 4, "! UNIMP !"),
    (ld_h_a, 4, "LD H, A"),
    (noimpl, 4, "! UNIMP !"),
    (noimpl, 4, "! UNIMP !"),
    (noimpl, 4, "! UNIMP !"),
    (noimpl, 4, "! UNIMP !"),
    (noimpl, 4, "! UNIMP !"),
    (noimpl, 4, "! UNIMP !"),
    (noimpl, 4, "! UNIMP !"),
    (noimpl, 4, "! UNIMP !"),
    // 0x7 opcodes
    (noimpl, 4, "! UNIMP !"),
    (noimpl, 4, "! UNIMP !"),
    (noimpl, 4, "! UNIMP !"),
    (noimpl, 4, "! UNIMP !"),
    (noimpl, 4, "! UNIMP !"),
    (noimpl, 4, "! UNIMP !"),
    (noimpl, 4, "! UNIMP !"),
    (ld_mhl_a, 8, "LD [HL], A"),
    (noimpl, 4, "! UNIMP !"),
    (noimpl, 4, "! UNIMP !"),
    (noimpl, 4, "! UNIMP !"),
    (ld_a_e, 4, "LD A, E"),
    (ld_a_h, 4, "LD A, H"),
    (noimpl, 4, "! UNIMP !"),
    (noimpl, 4, "! UNIMP !"),
    (noimpl, 4, "! UNIMP !"),
    // 0x8 opcodes
    (noimpl, 4, "! UNIMP !"),
    (noimpl, 4, "! UNIMP !"),
    (noimpl, 4, "! UNIMP !"),
    (noimpl, 4, "! UNIMP !"),
    (noimpl, 4, "! UNIMP !"),
    (noimpl, 4, "! UNIMP !"),
    (noimpl, 4, "! UNIMP !"),
    (noimpl, 4, "! UNIMP !"),
    (noimpl, 4, "! UNIMP !"),
    (noimpl, 4, "! UNIMP !"),
    (noimpl, 4, "! UNIMP !"),
    (noimpl, 4, "! UNIMP !"),
    (noimpl, 4, "! UNIMP !"),
    (noimpl, 4, "! UNIMP !"),
    (noimpl, 4, "! UNIMP !"),
    (noimpl, 4, "! UNIMP !"),
    // 0x9 opcodes
    (sub_a_b, 4, "SUB A, B"),
    (noimpl, 4, "! UNIMP !"),
    (noimpl, 4, "! UNIMP !"),
    (noimpl, 4, "! UNIMP !"),
    (noimpl, 4, "! UNIMP !"),
    (noimpl, 4, "! UNIMP !"),
    (noimpl, 4, "! UNIMP !"),
    (noimpl, 4, "! UNIMP !"),
    (noimpl, 4, "! UNIMP !"),
    (noimpl, 4, "! UNIMP !"),
    (noimpl, 4, "! UNIMP !"),
    (noimpl, 4, "! UNIMP !"),
    (noimpl, 4, "! UNIMP !"),
    (noimpl, 4, "! UNIMP !"),
    (noimpl, 4, "! UNIMP !"),
    (noimpl, 4, "! UNIMP !"),
    // 0xa opcodes
    (noimpl, 4, "! UNIMP !"),
    (noimpl, 4, "! UNIMP !"),
    (noimpl, 4, "! UNIMP !"),
    (noimpl, 4, "! UNIMP !"),
    (noimpl, 4, "! UNIMP !"),
    (noimpl, 4, "! UNIMP !"),
    (noimpl, 4, "! UNIMP !"),
    (noimpl, 4, "! UNIMP !"),
    (noimpl, 4, "! UNIMP !"),
    (noimpl, 4, "! UNIMP !"),
    (noimpl, 4, "! UNIMP !"),
    (noimpl, 4, "! UNIMP !"),
    (noimpl, 4, "! UNIMP !"),
    (noimpl, 4, "! UNIMP !"),
    (noimpl, 4, "! UNIMP !"),
    (xor_a_a, 4, "XOR A, A"),
    // 0xb opcodes
    (noimpl, 4, "! UNIMP !"),
    (noimpl, 4, "! UNIMP !"),
    (noimpl, 4, "! UNIMP !"),
    (noimpl, 4, "! UNIMP !"),
    (noimpl, 4, "! UNIMP !"),
    (noimpl, 4, "! UNIMP !"),
    (noimpl, 4, "! UNIMP !"),
    (noimpl, 4, "! UNIMP !"),
    (noimpl, 4, "! UNIMP !"),
    (noimpl, 4, "! UNIMP !"),
    (noimpl, 4, "! UNIMP !"),
    (noimpl, 4, "! UNIMP !"),
    (noimpl, 4, "! UNIMP !"),
    (noimpl, 4, "! UNIMP !"),
    (noimpl, 4, "! UNIMP !"),
    (noimpl, 4, "! UNIMP !"),
    // 0xc opcodes
    (ret_nz, 8, "RET NZ"),
    (pop_bc, 12, "POP BC"),
    (noimpl, 4, "! UNIMP !"),
    (noimpl, 4, "! UNIMP !"),
    (noimpl, 4, "! UNIMP !"),
    (push_bc, 16, "PUSH BC"),
    (noimpl, 4, "! UNIMP !"),
    (noimpl, 4, "! UNIMP !"),
    (noimpl, 4, "! UNIMP !"),
    (ret, 16, "RET"),
    (noimpl, 4, "! UNIMP !"),
    (noimpl, 4, "! UNIMP !"),
    (noimpl, 4, "! UNIMP !"),
    (call_u16, 24, "CALL u16"),
    (noimpl, 4, "! UNIMP !"),
    (noimpl, 4, "! UNIMP !"),
    // 0xd opcodes
    (noimpl, 4, "! UNIMP !"),
    (noimpl, 4, "! UNIMP !"),
    (noimpl, 4, "! UNIMP !"),
    (noimpl, 4, "! UNIMP !"),
    (noimpl, 4, "! UNIMP !"),
    (noimpl, 4, "! UNIMP !"),
    (noimpl, 4, "! UNIMP !"),
    (noimpl, 4, "! UNIMP !"),
    (noimpl, 4, "! UNIMP !"),
    (noimpl, 4, "! UNIMP !"),
    (noimpl, 4, "! UNIMP !"),
    (noimpl, 4, "! UNIMP !"),
    (noimpl, 4, "! UNIMP !"),
    (noimpl, 4, "! UNIMP !"),
    (noimpl, 4, "! UNIMP !"),
    (noimpl, 4, "! UNIMP !"),
    // 0xe opcodes
    (ld_mff00u8_a, 12, "LD [FF00+u8], A"),
    (noimpl, 4, "! UNIMP !"),
    (ld_mff00c_a, 8, "LD [FF00+C], A"),
    (noimpl, 4, "! UNIMP !"),
    (noimpl, 4, "! UNIMP !"),
    (noimpl, 4, "! UNIMP !"),
    (noimpl, 4, "! UNIMP !"),
    (noimpl, 4, "! UNIMP !"),
    (noimpl, 4, "! UNIMP !"),
    (noimpl, 4, "! UNIMP !"),
    (ld_mu16_a, 16, "LD [u16], A"),
    (noimpl, 4, "! UNIMP !"),
    (noimpl, 4, "! UNIMP !"),
    (noimpl, 4, "! UNIMP !"),
    (noimpl, 4, "! UNIMP !"),
    (noimpl, 4, "! UNIMP !"),
    // 0xf opcodes
    (ld_a_mff00u8, 12, "LD A, [FF00+u8]"),
    (noimpl, 4, "! UNIMP !"),
    (noimpl, 4, "! UNIMP !"),
    (noimpl, 4, "! UNIMP !"),
    (noimpl, 4, "! UNIMP !"),
    (noimpl, 4, "! UNIMP !"),
    (noimpl, 4, "! UNIMP !"),
    (noimpl, 4, "! UNIMP !"),
    (noimpl, 4, "! UNIMP !"),
    (noimpl, 4, "! UNIMP !"),
    (noimpl, 4, "! UNIMP !"),
    (noimpl, 4, "! UNIMP !"),
    (noimpl, 4, "! UNIMP !"),
    (noimpl, 4, "! UNIMP !"),
    (cp_a_u8, 8, "CP A, u8"),
    (noimpl, 4, "! UNIMP !"),
];

pub const BITWISE: [(fn(&mut Cpu), u8, &'static str); 176] = [
    // 0x0 opcodes
    (noimpl, 4, "! UNIMP !"),
    (noimpl, 4, "! UNIMP !"),
    (noimpl, 4, "! UNIMP !"),
    (noimpl, 4, "! UNIMP !"),
    (noimpl, 4, "! UNIMP !"),
    (noimpl, 4, "! UNIMP !"),
    (noimpl, 4, "! UNIMP !"),
    (noimpl, 4, "! UNIMP !"),
    (noimpl, 4, "! UNIMP !"),
    (noimpl, 4, "! UNIMP !"),
    (noimpl, 4, "! UNIMP !"),
    (noimpl, 4, "! UNIMP !"),
    (noimpl, 4, "! UNIMP !"),
    (noimpl, 4, "! UNIMP !"),
    (noimpl, 4, "! UNIMP !"),
    (noimpl, 4, "! UNIMP !"),
    // 0x1 opcodes
    (noimpl, 4, "! UNIMP !"),
    (rl_c, 8, "RL C"),
    (noimpl, 4, "! UNIMP !"),
    (noimpl, 4, "! UNIMP !"),
    (noimpl, 4, "! UNIMP !"),
    (noimpl, 4, "! UNIMP !"),
    (noimpl, 4, "! UNIMP !"),
    (noimpl, 4, "! UNIMP !"),
    (noimpl, 4, "! UNIMP !"),
    (noimpl, 4, "! UNIMP !"),
    (noimpl, 4, "! UNIMP !"),
    (noimpl, 4, "! UNIMP !"),
    (noimpl, 4, "! UNIMP !"),
    (noimpl, 4, "! UNIMP !"),
    (noimpl, 4, "! UNIMP !"),
    (noimpl, 4, "! UNIMP !"),
    // 0x2 opcodes
    (noimpl, 4, "! UNIMP !"),
    (noimpl, 4, "! UNIMP !"),
    (noimpl, 4, "! UNIMP !"),
    (noimpl, 4, "! UNIMP !"),
    (noimpl, 4, "! UNIMP !"),
    (noimpl, 4, "! UNIMP !"),
    (noimpl, 4, "! UNIMP !"),
    (noimpl, 4, "! UNIMP !"),
    (noimpl, 4, "! UNIMP !"),
    (noimpl, 4, "! UNIMP !"),
    (noimpl, 4, "! UNIMP !"),
    (noimpl, 4, "! UNIMP !"),
    (noimpl, 4, "! UNIMP !"),
    (noimpl, 4, "! UNIMP !"),
    (noimpl, 4, "! UNIMP !"),
    (noimpl, 4, "! UNIMP !"),
    // 0x3 opcodes
    (noimpl, 4, "! UNIMP !"),
    (noimpl, 4, "! UNIMP !"),
    (noimpl, 4, "! UNIMP !"),
    (noimpl, 4, "! UNIMP !"),
    (noimpl, 4, "! UNIMP !"),
    (noimpl, 4, "! UNIMP !"),
    (noimpl, 4, "! UNIMP !"),
    (noimpl, 4, "! UNIMP !"),
    (noimpl, 4, "! UNIMP !"),
    (noimpl, 4, "! UNIMP !"),
    (noimpl, 4, "! UNIMP !"),
    (noimpl, 4, "! UNIMP !"),
    (noimpl, 4, "! UNIMP !"),
    (noimpl, 4, "! UNIMP !"),
    (noimpl, 4, "! UNIMP !"),
    (noimpl, 4, "! UNIMP !"),
    // 0x4 opcodes
    (noimpl, 4, "! UNIMP !"),
    (noimpl, 4, "! UNIMP !"),
    (noimpl, 4, "! UNIMP !"),
    (noimpl, 4, "! UNIMP !"),
    (noimpl, 4, "! UNIMP !"),
    (noimpl, 4, "! UNIMP !"),
    (noimpl, 4, "! UNIMP !"),
    (noimpl, 4, "! UNIMP !"),
    (noimpl, 4, "! UNIMP !"),
    (noimpl, 4, "! UNIMP !"),
    (noimpl, 4, "! UNIMP !"),
    (noimpl, 4, "! UNIMP !"),
    (noimpl, 4, "! UNIMP !"),
    (noimpl, 4, "! UNIMP !"),
    (noimpl, 4, "! UNIMP !"),
    (noimpl, 4, "! UNIMP !"),
    // 0x5 opcodes
    (noimpl, 4, "! UNIMP !"),
    (noimpl, 4, "! UNIMP !"),
    (noimpl, 4, "! UNIMP !"),
    (noimpl, 4, "! UNIMP !"),
    (noimpl, 4, "! UNIMP !"),
    (noimpl, 4, "! UNIMP !"),
    (noimpl, 4, "! UNIMP !"),
    (noimpl, 4, "! UNIMP !"),
    (noimpl, 4, "! UNIMP !"),
    (noimpl, 4, "! UNIMP !"),
    (noimpl, 4, "! UNIMP !"),
    (noimpl, 4, "! UNIMP !"),
    (noimpl, 4, "! UNIMP !"),
    (noimpl, 4, "! UNIMP !"),
    (noimpl, 4, "! UNIMP !"),
    (noimpl, 4, "! UNIMP !"),
    // 0x6 opcodes
    (noimpl, 4, "! UNIMP !"),
    (noimpl, 4, "! UNIMP !"),
    (noimpl, 4, "! UNIMP !"),
    (noimpl, 4, "! UNIMP !"),
    (noimpl, 4, "! UNIMP !"),
    (noimpl, 4, "! UNIMP !"),
    (noimpl, 4, "! UNIMP !"),
    (noimpl, 4, "! UNIMP !"),
    (noimpl, 4, "! UNIMP !"),
    (noimpl, 4, "! UNIMP !"),
    (noimpl, 4, "! UNIMP !"),
    (noimpl, 4, "! UNIMP !"),
    (noimpl, 4, "! UNIMP !"),
    (noimpl, 4, "! UNIMP !"),
    (noimpl, 4, "! UNIMP !"),
    (noimpl, 4, "! UNIMP !"),
    // 0x7 opcodes
    (noimpl, 4, "! UNIMP !"),
    (noimpl, 4, "! UNIMP !"),
    (noimpl, 4, "! UNIMP !"),
    (noimpl, 4, "! UNIMP !"),
    (noimpl, 4, "! UNIMP !"),
    (noimpl, 4, "! UNIMP !"),
    (noimpl, 4, "! UNIMP !"),
    (noimpl, 4, "! UNIMP !"),
    (noimpl, 4, "! UNIMP !"),
    (noimpl, 4, "! UNIMP !"),
    (noimpl, 4, "! UNIMP !"),
    (noimpl, 4, "! UNIMP !"),
    (bit_7_h, 8, "BIT 7, H"),
    (noimpl, 4, "! UNIMP !"),
    (noimpl, 4, "! UNIMP !"),
    (noimpl, 4, "! UNIMP !"),
    // 0x8 opcodes
    (noimpl, 4, "! UNIMP !"),
    (noimpl, 4, "! UNIMP !"),
    (noimpl, 4, "! UNIMP !"),
    (noimpl, 4, "! UNIMP !"),
    (noimpl, 4, "! UNIMP !"),
    (noimpl, 4, "! UNIMP !"),
    (noimpl, 4, "! UNIMP !"),
    (noimpl, 4, "! UNIMP !"),
    (noimpl, 4, "! UNIMP !"),
    (noimpl, 4, "! UNIMP !"),
    (noimpl, 4, "! UNIMP !"),
    (noimpl, 4, "! UNIMP !"),
    (noimpl, 4, "! UNIMP !"),
    (noimpl, 4, "! UNIMP !"),
    (noimpl, 4, "! UNIMP !"),
    (noimpl, 4, "! UNIMP !"),
    // 0x9 opcodes
    (noimpl, 4, "! UNIMP !"),
    (noimpl, 4, "! UNIMP !"),
    (noimpl, 4, "! UNIMP !"),
    (noimpl, 4, "! UNIMP !"),
    (noimpl, 4, "! UNIMP !"),
    (noimpl, 4, "! UNIMP !"),
    (noimpl, 4, "! UNIMP !"),
    (noimpl, 4, "! UNIMP !"),
    (noimpl, 4, "! UNIMP !"),
    (noimpl, 4, "! UNIMP !"),
    (noimpl, 4, "! UNIMP !"),
    (noimpl, 4, "! UNIMP !"),
    (noimpl, 4, "! UNIMP !"),
    (noimpl, 4, "! UNIMP !"),
    (noimpl, 4, "! UNIMP !"),
    (noimpl, 4, "! UNIMP !"),
    // 0xa opcodes
    (noimpl, 4, "! UNIMP !"),
    (noimpl, 4, "! UNIMP !"),
    (noimpl, 4, "! UNIMP !"),
    (noimpl, 4, "! UNIMP !"),
    (noimpl, 4, "! UNIMP !"),
    (noimpl, 4, "! UNIMP !"),
    (noimpl, 4, "! UNIMP !"),
    (noimpl, 4, "! UNIMP !"),
    (noimpl, 4, "! UNIMP !"),
    (noimpl, 4, "! UNIMP !"),
    (noimpl, 4, "! UNIMP !"),
    (noimpl, 4, "! UNIMP !"),
    (noimpl, 4, "! UNIMP !"),
    (noimpl, 4, "! UNIMP !"),
    (noimpl, 4, "! UNIMP !"),
    (noimpl, 4, "! UNIMP !"),
];

pub struct Cpu {
    pc: u16,
    sp: u16,
    a: u8,
    b: u8,
    c: u8,
    d: u8,
    e: u8,
    h: u8,
    l: u8,
    zero: bool,
    sub: bool,
    half_carry: bool,
    carry: bool,
    mmu: Mmu,
    ticks: u32,
}

impl Cpu {
    pub fn new(mmu: Mmu) -> Cpu {
        Cpu {
            pc: 0x0,
            sp: 0x0,
            a: 0x0,
            b: 0x0,
            c: 0x0,
            d: 0x0,
            e: 0x0,
            h: 0x0,
            l: 0x0,
            zero: false,
            sub: false,
            half_carry: false,
            carry: false,
            mmu: mmu,
            ticks: 0,
        }
    }

    pub fn clock(&mut self) -> u8 {
        let pc = self.pc;

        // fetches the current instruction and increments
        // the PC (program counter) accordingly
        let mut opcode = self.mmu.read(self.pc);
        self.pc = self.pc.wrapping_add(1);

        let is_prefix = opcode == PREFIX;
        let instruction: &(fn(&mut Cpu), u8, &str);

        if is_prefix {
            opcode = self.mmu.read(self.pc);
            self.pc = self.pc.wrapping_add(1);
            instruction = &BITWISE[opcode as usize];
        } else {
            instruction = &INSTRUCTIONS[opcode as usize];
        }

        let (instruction_fn, instruction_time, instruction_str) = instruction;

        if *instruction_str == "! UNIMP !" {
            println!(
                "{}\t(0x{:02x})\t${:04x} {}",
                instruction_str, opcode, pc, is_prefix
            );
        }

        if pc == 0x0080 {
            println!("GOING TO PLAY BOOT SOUND");
        }

        if pc == 0x008f {
            println!("GOING TO PLAY BOOT 0x00e0");
        }

        // calls the current instruction and increments the number of
        // cycles executed by the instruction time of the instruction
        // that has just been executed
        instruction_fn(self);
        self.ticks = self.ticks.wrapping_add(*instruction_time as u32);

        // returns the number of cycles that the operation
        // that has been executed has taken
        *instruction_time
    }

    #[inline(always)]
    pub fn mmu(&mut self) -> &mut Mmu {
        &mut self.mmu
    }

    #[inline(always)]
    pub fn ppu(&mut self) -> &mut Ppu {
        self.mmu().ppu()
    }

    #[inline(always)]
    pub fn ticks(&self) -> u32 {
        self.ticks
    }

    #[inline(always)]
    pub fn pc(&self) -> u16 {
        self.pc
    }

    #[inline(always)]
    pub fn sp(&self) -> u16 {
        self.sp
    }

    #[inline(always)]
    fn af(&self) -> u16 {
        (self.a as u16) << 8 | self.f() as u16
    }

    #[inline(always)]
    fn bc(&self) -> u16 {
        (self.b as u16) << 8 | self.c as u16
    }

    #[inline(always)]
    fn f(&self) -> u8 {
        let mut f = 0x0u8;
        if self.zero {
            f |= 0x80;
        }
        if self.sub {
            f |= 0x40;
        }
        if self.half_carry {
            f |= 0x20;
        }
        if self.carry {
            f |= 0x10;
        }
        f
    }

    #[inline(always)]
    fn set_bc(&mut self, value: u16) {
        self.b = (value >> 8) as u8;
        self.c = value as u8;
    }

    #[inline(always)]
    fn de(&self) -> u16 {
        (self.d as u16) << 8 | self.e as u16
    }

    #[inline(always)]
    fn set_de(&mut self, value: u16) {
        self.d = (value >> 8) as u8;
        self.e = value as u8;
    }

    #[inline(always)]
    fn hl(&self) -> u16 {
        (self.h as u16) << 8 | self.l as u16
    }

    #[inline(always)]
    fn set_hl(&mut self, value: u16) {
        self.h = (value >> 8) as u8;
        self.l = value as u8;
    }

    #[inline(always)]
    fn read_u8(&mut self) -> u8 {
        let byte = self.mmu.read(self.pc);
        self.pc = self.pc.wrapping_add(1);
        byte
    }

    #[inline(always)]
    fn read_u16(&mut self) -> u16 {
        let byte1 = self.read_u8();
        let byte2 = self.read_u8();
        let word = byte1 as u16 | ((byte2 as u16) << 8);
        word
    }

    #[inline(always)]
    fn push_byte(&mut self, byte: u8) {
        self.sp = self.sp.wrapping_sub(1);
        self.mmu.write(self.sp, byte);
    }

    #[inline(always)]
    fn push_word(&mut self, word: u16) {
        self.push_byte((word >> 8) as u8);
        self.push_byte(word as u8);
    }

    #[inline(always)]
    fn pop_byte(&mut self) -> u8 {
        let byte = self.mmu.read(self.sp);
        self.sp = self.sp.wrapping_add(1);
        byte
    }

    #[inline(always)]
    fn pop_word(&mut self) -> u16 {
        let word = self.pop_byte() as u16 | ((self.pop_byte() as u16) << 8);
        word
    }

    #[inline(always)]
    fn get_zero(&self) -> bool {
        self.zero
    }

    #[inline(always)]
    fn set_zero(&mut self, value: bool) {
        self.zero = value
    }

    #[inline(always)]
    fn get_sub(&self) -> bool {
        self.sub
    }

    #[inline(always)]
    fn set_sub(&mut self, value: bool) {
        self.sub = value;
    }

    #[inline(always)]
    fn get_half_carry(&self) -> bool {
        self.half_carry
    }

    #[inline(always)]
    fn set_half_carry(&mut self, value: bool) {
        self.half_carry = value
    }

    #[inline(always)]
    fn get_carry(&self) -> bool {
        self.carry
    }

    #[inline(always)]
    fn set_carry(&mut self, value: bool) {
        self.carry = value;
    }
}

fn nop(_cpu: &mut Cpu) {}

fn noimpl(_cpu: &mut Cpu) {
    todo!("Instruction not implemented");
}

fn ld_bc_u16(cpu: &mut Cpu) {
    let word = cpu.read_u16();
    cpu.set_bc(word);
}

fn ld_mbc_a(cpu: &mut Cpu) {
    cpu.mmu.write(cpu.bc(), cpu.a);
}

fn inc_bc(cpu: &mut Cpu) {
    cpu.set_bc(cpu.bc().wrapping_add(1));
}

fn inc_b(cpu: &mut Cpu) {
    let value = cpu.b.wrapping_add(1);

    cpu.set_sub(false);
    cpu.set_zero(value == 0);
    cpu.set_half_carry((value & 0xf) == 0xf);

    cpu.b = value;
}

fn dec_b(cpu: &mut Cpu) {
    let b = cpu.b;
    let value = b.wrapping_sub(1);

    cpu.set_sub(true);
    cpu.set_zero(value == 0);
    cpu.set_half_carry((b & 0xf) == 0xf);

    cpu.b = value;
}

fn ld_b_u8(cpu: &mut Cpu) {
    let byte = cpu.read_u8();
    cpu.b = byte;
}

fn rlca(cpu: &mut Cpu) {
    let carry = cpu.a >> 7;

    cpu.a = cpu.a << 1 | carry;

    cpu.set_sub(false);
    cpu.set_zero(false);
    cpu.set_half_carry(false);
    cpu.set_carry(carry == 1);
}

fn ld_mu16_sp(cpu: &mut Cpu) {
    let word = cpu.read_u16();
    cpu.mmu.write(word, cpu.sp as u8);
    cpu.mmu.write(word + 1, (cpu.sp >> 8) as u8);
}

fn add_hl_bc(cpu: &mut Cpu) {
    let value = add_u16_u16(cpu, cpu.hl(), cpu.bc());
    cpu.set_hl(value);
}

fn inc_c(cpu: &mut Cpu) {
    let value = cpu.c.wrapping_add(1);

    cpu.set_sub(false);
    cpu.set_zero(value == 0);
    cpu.set_half_carry((value & 0xf) == 0xf);

    cpu.c = value;
}

fn dec_c(cpu: &mut Cpu) {
    let c = cpu.c;
    let value = c.wrapping_sub(1);

    cpu.set_sub(true);
    cpu.set_zero(value == 0);
    cpu.set_half_carry((c & 0xf) == 0xf);

    cpu.c = value;
}

fn ld_c_u8(cpu: &mut Cpu) {
    let byte = cpu.read_u8();
    cpu.c = byte;
}

fn ld_de_u16(cpu: &mut Cpu) {
    let word = cpu.read_u16();
    cpu.set_de(word);
}

fn inc_de(cpu: &mut Cpu) {
    cpu.set_de(cpu.de().wrapping_add(1));
}

fn dec_d(cpu: &mut Cpu) {
    let d = cpu.d;
    let value = d.wrapping_sub(1);

    cpu.set_sub(true);
    cpu.set_zero(value == 0);
    cpu.set_half_carry((d & 0xf) == 0xf);

    cpu.d = value;
}

fn rla(cpu: &mut Cpu) {
    let carry = cpu.get_carry();

    cpu.set_carry(cpu.a & 0x80 == 0x80);

    cpu.a = cpu.a << 1 | carry as u8;

    cpu.set_sub(false);
    cpu.set_zero(false);
    cpu.set_half_carry(false);
}

fn jr_i8(cpu: &mut Cpu) {
    let byte = cpu.read_u8() as i8;
    cpu.pc = (cpu.pc as i16).wrapping_add(byte as i16) as u16;
}

fn ld_a_mde(cpu: &mut Cpu) {
    let byte = cpu.mmu.read(cpu.de());
    cpu.a = byte;
}

fn dec_e(cpu: &mut Cpu) {
    let e = cpu.e;
    let value = e.wrapping_sub(1);

    cpu.set_sub(true);
    cpu.set_zero(value == 0);
    cpu.set_half_carry((e & 0xf) == 0xf);

    cpu.e = value;
}

fn ld_e_u8(cpu: &mut Cpu) {
    let byte = cpu.read_u8();
    cpu.e = byte;
}

fn jr_nz_i8(cpu: &mut Cpu) {
    let byte = cpu.read_u8() as i8;

    if cpu.get_zero() {
        return;
    }

    cpu.pc = (cpu.pc as i16).wrapping_add(byte as i16) as u16;
    cpu.ticks = cpu.ticks.wrapping_add(4);
}

fn ld_hl_u16(cpu: &mut Cpu) {
    let word = cpu.read_u16();
    cpu.set_hl(word);
}

fn ld_mhli_a(cpu: &mut Cpu) {
    cpu.mmu.write(cpu.hl(), cpu.a);
    cpu.set_hl(cpu.hl().wrapping_add(1));
}

fn inc_hl(cpu: &mut Cpu) {
    cpu.set_hl(cpu.hl().wrapping_add(1));
}

fn inc_h(cpu: &mut Cpu) {
    let value = cpu.h.wrapping_add(1);

    cpu.set_sub(false);
    cpu.set_zero(value == 0);
    cpu.set_half_carry((value & 0xf) == 0xf);

    cpu.h = value;
}

fn jr_z_i8(cpu: &mut Cpu) {
    let byte = cpu.read_u8() as i8;

    if !cpu.get_zero() {
        return;
    }

    cpu.pc = (cpu.pc as i16).wrapping_add(byte as i16) as u16;
    cpu.ticks = cpu.ticks.wrapping_add(4);
}

fn ld_l_u8(cpu: &mut Cpu) {
    let byte = cpu.read_u8();
    cpu.l = byte;
}

fn ld_sp_u16(cpu: &mut Cpu) {
    cpu.sp = cpu.read_u16();
}

fn ld_mhld_a(cpu: &mut Cpu) {
    cpu.mmu.write(cpu.hl(), cpu.a);
    cpu.set_hl(cpu.hl().wrapping_sub(1));
}

fn dec_a(cpu: &mut Cpu) {
    let a = cpu.a;
    let value = a.wrapping_sub(1);

    cpu.set_sub(true);
    cpu.set_zero(value == 0);
    cpu.set_half_carry((a & 0xf) == 0xf);

    cpu.a = value;
}

fn ld_a_u8(cpu: &mut Cpu) {
    let byte = cpu.read_u8();
    cpu.a = byte;
}

fn ld_b_h(cpu: &mut Cpu) {
    cpu.b = cpu.h;
}

fn ld_c_a(cpu: &mut Cpu) {
    cpu.c = cpu.a;
}

fn ld_d_a(cpu: &mut Cpu) {
    cpu.d = cpu.a;
}

fn ld_h_a(cpu: &mut Cpu) {
    cpu.h = cpu.a;
}

fn ld_mhl_a(cpu: &mut Cpu) {
    cpu.mmu.write(cpu.hl(), cpu.a);
}

fn ld_a_e(cpu: &mut Cpu) {
    cpu.a = cpu.e;
}

fn ld_a_h(cpu: &mut Cpu) {
    cpu.a = cpu.h;
}

fn sub_a_b(cpu: &mut Cpu) {
    cpu.a = sub_set_flags(cpu, cpu.a, cpu.b);
}

fn xor_a_a(cpu: &mut Cpu) {
    cpu.a ^= cpu.a;

    cpu.set_sub(false);
    cpu.set_zero(cpu.a == 0);
    cpu.set_half_carry(false);
    cpu.set_carry(false);
}

fn ret_nz(cpu: &mut Cpu) {
    if cpu.get_zero() {
        return;
    }

    cpu.pc = cpu.pop_word();
    cpu.ticks = cpu.ticks.wrapping_add(12);
}

fn pop_bc(cpu: &mut Cpu) {
    let word = cpu.pop_word();
    cpu.set_bc(word);
}

fn push_bc(cpu: &mut Cpu) {
    cpu.push_word(cpu.bc());
}

fn ret(cpu: &mut Cpu) {
    cpu.pc = cpu.pop_word();
}

fn call_u16(cpu: &mut Cpu) {
    let word = cpu.read_u16();
    cpu.push_word(cpu.pc);
    cpu.pc = word;
}

fn ld_mff00u8_a(cpu: &mut Cpu) {
    let byte = cpu.read_u8();
    cpu.mmu.write(0xff00 + byte as u16, cpu.a);
}

fn ld_mff00c_a(cpu: &mut Cpu) {
    cpu.mmu.write(0xff00 + cpu.c as u16, cpu.a);
}

fn ld_mu16_a(cpu: &mut Cpu) {
    let word = cpu.read_u16();
    cpu.mmu.write(word, cpu.a);
}

fn ld_a_mff00u8(cpu: &mut Cpu) {
    let byte = cpu.read_u8();
    cpu.a = cpu.mmu.read(0xff00 + byte as u16);
}

fn cp_a_u8(cpu: &mut Cpu) {
    let byte = cpu.read_u8();
    sub_set_flags(cpu, cpu.a, byte);
}

fn rl_c(cpu: &mut Cpu) {
    cpu.c = rl(cpu, cpu.c);
}

fn bit_7_h(cpu: &mut Cpu) {
    bit_h(cpu, 7);
}

/// Helper function that rotates (shifts) the given
/// byte (probably from a register) and updates the
/// proper flag registers.
fn rl(cpu: &mut Cpu, byte: u8) -> u8 {
    let carry = cpu.get_carry();

    cpu.set_carry(byte & 0x80 == 0x80);

    let result = (byte << 1) | carry as u8;

    cpu.set_sub(false);
    cpu.set_zero(result == 0);
    cpu.set_half_carry(false);

    result
}

/// Helper function to test one bit in a u8.
/// Returns true if bit is 0.
fn bit_zero(val: u8, bit: u8) -> bool {
    (val & (1u8 << (bit as usize))) == 0
}

fn bit_h(cpu: &mut Cpu, bit: u8) {
    cpu.set_sub(false);
    cpu.set_zero(bit_zero(cpu.h, bit));
    cpu.set_half_carry(true);
}

fn sub_set_flags(cpu: &mut Cpu, x: u8, y: u8) -> u8 {
    // checks for borrow using 32bit arithmetics
    let x = x as u32;
    let y = y as u32;

    let value = x.wrapping_sub(y);
    let value_b = value as u8;

    cpu.set_sub(true);
    cpu.set_carry(value & 0x100 == 0x100);
    cpu.set_zero(value_b == 0);
    cpu.set_half_carry((x ^ y ^ value) & 0x10 == 0x10);

    value_b
}

fn add_u16_u16(cpu: &mut Cpu, first: u16, second: u16) -> u16 {
    let first = first as u32;
    let second = second as u32;
    let value = first.wrapping_add(second);

    cpu.set_sub(false);
    cpu.set_carry(value & 0x10000 == 0x10000);
    cpu.set_half_carry((first ^ second ^ value) & 0x1000 == 0x1000);

    value as u16
}
