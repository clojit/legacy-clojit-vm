use std::mem;

use vm::Instr;
use vm::OpCode;

pub trait Decode {
    fn decode(self) -> OpCode;
}

pub trait FromInstr {
    fn from_instr(Instr) -> Self;
}

pub trait ToInstr {
    fn to_instr(self) -> Instr;
}

#[repr(packed)]
pub struct OpABC {
    pub b: u8, pub c:  u8,
    pub a: u8, pub op: u8
}

#[repr(packed)]
pub struct OpAD {
    pub d: u16,
    pub a: u8, pub op: u8
}

pub fn from_instr<A: FromInstr>(i: &Instr) -> A {
    FromInstr::from_instr(*i)
}

impl FromInstr for Instr {
    fn from_instr(instr: Instr) -> Instr {
        instr
    }
}

impl ToInstr for OpABC {
    fn to_instr(self) -> Instr {
        unsafe { mem::transmute(self) }
    }
}

impl FromInstr for OpABC {
    fn from_instr(instr: Instr) -> OpABC {
        unsafe { mem::transmute(instr) }
    }
}

impl ToInstr for OpAD {
    fn to_instr(self) -> Instr {
        unsafe { mem::transmute(self) }
    }
}

impl FromInstr for OpAD {
    fn from_instr(instr: Instr) -> OpAD {
        unsafe { mem::transmute(instr) }
    }
}

impl Decode for Instr {
    fn decode(self) -> OpCode {
        let opcode = from_instr::<OpABC>(&self).op;
        match FromPrimitive::from_u8(opcode) {
            Some(op) => op,
            None => panic!("invalid opcode: {}", opcode)
        }
    }
}
