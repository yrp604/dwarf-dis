use std::convert::TryInto;

use log::*;
use nano_leb128::{SLEB128, ULEB128};

fn read_u8(pc: &mut usize, bytecode: &[u8]) -> u8 {
    let r = bytecode[*pc];

    *pc += 1;

    r
}

fn read_i8(pc: &mut usize, bytecode: &[u8]) -> i8 {
    let r = bytecode[*pc] as i8;

    *pc += 1;

    r
}

fn read_u16(pc: &mut usize, bytecode: &[u8]) -> u16 {
    let r = u16::from_le_bytes(bytecode[*pc..*pc+2].try_into().unwrap());

    *pc += 2;

    r
}

fn read_i16(pc: &mut usize, bytecode: &[u8]) -> i16 {
    let r = i16::from_le_bytes(bytecode[*pc..*pc+2].try_into().unwrap());

    *pc += 2;

    r
}

fn read_u32(pc: &mut usize, bytecode: &[u8]) -> u32 {
    let r = u32::from_le_bytes(bytecode[*pc..*pc+4].try_into().unwrap());

    *pc += 4;

    r
}

fn read_i32(pc: &mut usize, bytecode: &[u8]) -> i32 {
    let r = i32::from_le_bytes(bytecode[*pc..*pc+4].try_into().unwrap());

    *pc += 4;

    r
}

fn read_u64(pc: &mut usize, bytecode: &[u8]) -> u64 {
    let r = u64::from_le_bytes(bytecode[*pc..*pc+8].try_into().unwrap());

    *pc += 8;

    r
}

fn read_i64(pc: &mut usize, bytecode: &[u8]) -> i64 {
    let r = i64::from_le_bytes(bytecode[*pc..*pc+8].try_into().unwrap());

    *pc += 8;

    r
}

fn read_uleb128(pc: &mut usize, bytecode: &[u8]) -> u64 {
    let (val, sz) = ULEB128::read_from(bytecode)
        .expect(&format!("Could not read uleb128 value from bytecode @ {:x}", pc));

    *pc += sz;

    val.into()
}

fn read_sleb128(pc: &mut usize, bytecode: &[u8]) -> i64 {
    let (val, sz) = SLEB128::read_from(bytecode)
        .expect(&format!("Could not read sleb128 value from bytecode @ {:x}", pc));

    *pc += sz;

    val.into()
}

#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
pub enum Op {
    Addr,
    Deref,
    Const1u(u8),
    Const1s(i8),
    Const2u(u16),
    Const2s(i16),
    Const4u(u32),
    Const4s(i32),
    Const8u(u64),
    Const8s(i64),
    Constu(u64),
    Consts(i64),
    Dup,
    Drop,
    Over,
    Pick(u8),
    Swap,
    Rot,
    Abs,
    And,
    Div,
    Minus,
    Mod,
    Mul,
    Neg,
    Not,
    Or,
    Plus,
    PlusConst(u64),
    Bra(i16),
    Eq,
    Ge,
    Gt,
    Le,
    Lt,
    Ne,
    Shl,
    Shr,
    Shra,
    Xor,
    Skip(i16),
    Lit(u8),
    Reg(u8),
    Breg(u8),
    DerefSize(usize),
    Nop,
}

pub fn dis(bytecode: &[u8]) -> Vec<(usize, Op)> {
    let mut pc = 0;
    let mut disas = vec![];

    loop {
        if pc >= bytecode.len() { break }

        let op = bytecode[pc];
        let orig_pc = pc;
        pc += 1;

        let parsed_op = match op {
            0x03 => Op::Addr,
            0x06 => Op::Deref,
            0x08 => {
                let res = read_u8(&mut pc, &bytecode);
                Op::Const1u(res)
            }
            0x09 => {
                let res = read_i8(&mut pc, &bytecode);
                Op::Const1s(res)
            }
            0x0a => {
                let res = read_u16(&mut pc, &bytecode);
                Op::Const2u(res)
            }
            0x0b => {
                let res = read_i16(&mut pc, &bytecode);
                Op::Const2s(res)
            }
            0x0c => {
                let res = read_u32(&mut pc, &bytecode);
                Op::Const4u(res)
            }
            0x0d => {
                let res = read_i32(&mut pc, &bytecode);
                Op::Const4s(res)
            }
            0x0e => {
                let res = read_u64(&mut pc, &bytecode);
                Op::Const8u(res)
            }
            0x0f => {
                let res = read_i64(&mut pc, &bytecode);
                Op::Const8s(res)
            }
            0x10 => {
                let res = read_uleb128(&mut pc, &bytecode);

                Op::Constu(res)
            }
            0x11 => {
                let res = read_sleb128(&mut pc, &bytecode);

                Op::Consts(res)
            }
            0x12 => Op::Dup,
            0x13 => Op::Drop,
            0x14 => Op::Over,
            0x15 => {
                let off = read_u8(&mut pc, &bytecode);

                Op::Pick(off)
            }
            0x16 => Op::Swap,
            0x17 => Op::Rot,
            // xderef unimplemented by libgcc?
            0x19 => Op::Abs,
            0x1a => Op::And,
            0x1b => Op::Div,
            0x1c => Op::Minus,
            0x1d => Op::Mod,
            0x1e => Op::Mul,
            0x1f => Op::Neg,
            0x20 => Op::Not,
            0x21 => Op::Or,
            0x22 => Op::Plus,
            0x23 => {
                let val = read_uleb128(&mut pc, &bytecode);

                Op::PlusConst(val)
            }
            0x24 => Op::Shl,
            0x25 => Op::Shr,
            0x26 => Op::Shra,
            0x27 => Op::Xor,
            0x28 => {
                let off = read_i16(&mut pc, &bytecode);

                Op::Bra(off)
            }
            0x29 => Op::Eq,
            0x2a => Op::Ge,
            0x2b => Op::Gt,
            0x2c => Op::Le,
            0x2d => Op::Lt,
            0x2e => Op::Ne,
            0x2f => {
                let off = read_i16(&mut pc, &bytecode);

                Op::Skip(off)
            }
            0x30..=0x4f => {
                let lit = op - 0x30;

                Op::Lit(lit)
            }
            0x50..=0x6f => {
                let reg = op - 0x50;

                Op::Reg(reg)
            }
            0x70..=0x8f => {
                let breg = op - 0x70;

                Op::Breg(breg)
            }
            0x94 => {
                let sz = read_u8(&mut pc, &bytecode) as usize;

                assert!([1, 2, 4, 8].contains(&sz));

                Op::DerefSize(sz)
            },
            0x96 => Op::Nop,
            _ => unimplemented!("opcode {:04x}: 0x{:02x}", orig_pc, op),
        };

        debug!("{:04x}: {:x?}", orig_pc, parsed_op);

        disas.push((orig_pc, parsed_op))
    }

    disas
}
