use std::convert::TryInto;
use std::error::Error;
use std::fmt;
use std::mem;

use log::*;
use nano_leb128::{SLEB128, ULEB128};

fn read_u8(bytecode: &[u8]) -> (usize, u8) {
    let ty_sz = mem::size_of::<i8>();

    let r = bytecode[0];

    (ty_sz, r)
}

fn read_i8(bytecode: &[u8]) -> (usize, i8) {
    let ty_sz = mem::size_of::<i8>();

    let r = bytecode[0] as i8;

    (ty_sz, r)
}

fn read_u16(bytecode: &[u8]) -> (usize, u16) {
    let ty_sz = mem::size_of::<u16>();

    let r = u16::from_le_bytes(bytecode[..ty_sz].try_into().unwrap());

    (ty_sz, r)
}

fn read_i16(bytecode: &[u8]) -> (usize, i16) {
    let ty_sz = mem::size_of::<i16>();

    let r = i16::from_le_bytes(bytecode[..ty_sz].try_into().unwrap());

    (ty_sz , r)
}

fn read_u32(bytecode: &[u8]) -> (usize, u32) {
    let ty_sz = mem::size_of::<u32>();

    let r = u32::from_le_bytes(bytecode[..ty_sz].try_into().unwrap());

    (ty_sz, r)
}

fn read_i32(bytecode: &[u8]) -> (usize, i32) {
    let ty_sz = mem::size_of::<i32>();

    let r = i32::from_le_bytes(bytecode[..ty_sz].try_into().unwrap());

    (ty_sz, r)
}

fn read_u64(bytecode: &[u8]) -> (usize, u64) {
    let ty_sz = mem::size_of::<u64>();

    let r = u64::from_le_bytes(bytecode[..ty_sz].try_into().unwrap());

    (ty_sz, r)
}

fn read_i64(bytecode: &[u8]) -> (usize, i64) {
    let ty_sz = mem::size_of::<i64>();

    let r = i64::from_le_bytes(bytecode[..ty_sz].try_into().unwrap());

    (ty_sz, r)
}

fn read_uleb128(bytecode: &[u8]) -> (usize, u64) {
    let (val, sz) = ULEB128::read_from(bytecode).expect(&format!(
        "Could not read uleb128 value from bytecode {:x?}",
        &bytecode[..10]
    ));


    (sz, val.into())
}

fn read_sleb128(bytecode: &[u8]) -> (usize, i64) {
    let (val, sz) = SLEB128::read_from(bytecode).expect(&format!(
        "Could not read sleb128 value from bytecode @ {:x?}",
        &bytecode[..10]
    ));

    (sz, val.into())
}

#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
pub enum DwarfDisError {
    Decode(u8)
}

impl fmt::Display for DwarfDisError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Self::Decode(op) => write!(f, "dwarf-dis: could not decode opcode {:#x}", op),
        }
    }
}

impl Error for DwarfDisError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }
}

#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
pub enum Op {
    Addr(u64),
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
    BReg(u8, isize),
    RegX(u64),
    BRegX(u64, isize),
    DerefSize(usize),
    Nop,
}

impl Op {
    pub fn mnem(&self) -> &str {
        match *self {
            Self::Addr(_) => "addr",
            Self::Deref => "deref",
            Self::Const1u(_) => "const1u",
            Self::Const1s(_) => "const1s",
            Self::Const2u(_) => "const2u",
            Self::Const2s(_) => "const2s",
            Self::Const4u(_) => "const4u",
            Self::Const4s(_) => "const4s",
            Self::Const8u(_) => "const8u",
            Self::Const8s(_) => "const8s",
            Self::Constu(_) => "constu",
            Self::Consts(_) => "consts",
            Self::Dup => "dup",
            Self::Drop => "drop",
            Self::Over => "over",
            Self::Pick(_) => "pick",
            Self::Swap => "swap",
            Self::Rot => "rot",
            Self::Abs => "abs",
            Self::And => "and",
            Self::Div => "div",
            Self::Minus => "minus",
            Self::Mod => "mod",
            Self::Mul => "mul",
            Self::Neg => "neg",
            Self::Not => "not",
            Self::Or => "or",
            Self::Plus => "plus",
            Self::PlusConst(_) => "plus_const",
            Self::Bra(_) => "bra",
            Self::Eq => "eq",
            Self::Ge => "ge",
            Self::Gt => "gt",
            Self::Le => "le",
            Self::Lt => "lt",
            Self::Ne => "ne",
            Self::Shl => "shl",
            Self::Shr => "shr",
            Self::Shra => "shra",
            Self::Xor => "xor",
            Self::Skip(_) => "skip",
            Self::Lit(_) => "lit",
            Self::Reg(_) => "reg",
            Self::BReg(_, _) => "breg",
            Self::RegX(_) => "regx",
            Self::BRegX(_, _) => "bregx",
            Self::DerefSize(_) => "deref_size",
            Self::Nop => "nop",
        }
    }
}

pub fn decode(bytecode: &[u8]) -> Result<(usize, Op), DwarfDisError> {
    let op = bytecode[0];
    let mut sz = 1;

    let parsed_op = match op {
        0x03 => {
            let (data_sz, res) = read_u64(&bytecode[sz..]);
            sz += data_sz;
            Op::Addr(res)
        }
        0x06 => Op::Deref,
        0x08 => {
            let (data_sz, res) = read_u8(&bytecode[sz..]);
            sz += data_sz;
            Op::Const1u(res)
        }
        0x09 => {
            let (data_sz, res) = read_i8(&bytecode[sz..]);
            sz += data_sz;
            Op::Const1s(res)
        }
        0x0a => {
            let (data_sz, res) = read_u16(&bytecode[sz..]);
            sz += data_sz;
            Op::Const2u(res)
        }
        0x0b => {
            let (data_sz, res) = read_i16(&bytecode[sz..]);
            sz += data_sz;
            Op::Const2s(res)
        }
        0x0c => {
            let (data_sz, res) = read_u32(&bytecode[sz..]);
            sz += data_sz;
            Op::Const4u(res)
        }
        0x0d => {
            let (data_sz, res) = read_i32(&bytecode[sz..]);
            sz += data_sz;
            Op::Const4s(res)
        }
        0x0e => {
            let (data_sz, res) = read_u64(&bytecode[sz..]);
            sz += data_sz;
            Op::Const8u(res)
        }
        0x0f => {
            let (data_sz, res) = read_i64(&bytecode[sz..]);
            sz += data_sz;
            Op::Const8s(res)
        }
        0x10 => {
            let (data_sz, res) = read_uleb128(&bytecode[sz..]);
            sz += data_sz;

            Op::Constu(res)
        }
        0x11 => {
            let (data_sz, res) = read_sleb128(&bytecode[sz..]);
            sz += data_sz;

            Op::Consts(res)
        }
        0x12 => Op::Dup,
        0x13 => Op::Drop,
        0x14 => Op::Over,
        0x15 => {
            let (data_sz, off) = read_u8(&bytecode[sz..]);
            sz += data_sz;

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
            let (data_sz, val) = read_uleb128(&bytecode[sz..]);
            sz += data_sz;

            Op::PlusConst(val)
        }
        0x24 => Op::Shl,
        0x25 => Op::Shr,
        0x26 => Op::Shra,
        0x27 => Op::Xor,
        0x28 => {
            let (data_sz, off) = read_i16(&bytecode[sz..]);
            sz += data_sz;

            Op::Bra(off)
        }
        0x29 => Op::Eq,
        0x2a => Op::Ge,
        0x2b => Op::Gt,
        0x2c => Op::Le,
        0x2d => Op::Lt,
        0x2e => Op::Ne,
        0x2f => {
            let (data_sz, off) = read_i16(&bytecode[sz..]);
            sz += data_sz;

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

            let (data_sz, off) = read_sleb128(&bytecode[sz..]);
            sz += data_sz;

            Op::BReg(breg, off as isize)
        }
        0x90 => {
            let (data_sz, val) = read_uleb128(&bytecode[sz..]);
            sz += data_sz;

            Op::RegX(val)
        }
        0x92 => {
            let (data_sz, reg) = read_uleb128(&bytecode[sz..]);
            sz += data_sz;

            let (data_sz, off) = read_sleb128(&bytecode[sz..]);
            sz += data_sz;

            Op::BRegX(reg, off as isize)
        }
        0x94 => {
            let (data_sz, deref_sz) = read_u8(&bytecode[sz..]);
            sz += data_sz;

            assert!([1, 2, 4, 8].contains(&deref_sz));

            Op::DerefSize(deref_sz.into())
        }
        0x96 => Op::Nop,
        _ => {
            error!("unimplemented opcode: 0x{:02x}", op);

            return Err(DwarfDisError::Decode(op));
        }
    };

    debug!("{:x?}", parsed_op);

    Ok((sz, parsed_op))
}
