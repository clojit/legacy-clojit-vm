use std::mem;
use std::fmt;
use std::fmt::Show;
use std::num::FromPrimitive;

use vm;
use vm::Instr;
use vm::OpCode;

pub trait Decode {
    fn decode(self) -> OpCode;
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

impl OpABC {
    pub fn from_instr(instr: Instr) -> OpABC {
        unsafe { mem::transmute(instr) }
    }

    pub fn as_instr(self) -> Instr {
        unsafe { mem::transmute(self) }
    }
}

impl OpAD {
    pub fn from_instr(instr: Instr) -> OpAD {
        unsafe { mem::transmute(instr) }
    }

    pub fn as_instr(self) -> Instr {
        unsafe { mem::transmute(self) }
    }
}

impl Show for OpABC {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let op: OpCode = FromPrimitive::from_u8(self.op).unwrap();
        write!(f, "{}(a:{},b:{},c:{})", op, self.a, self.b, self.c)
    }
}

impl Show for OpAD {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let op: OpCode = FromPrimitive::from_u8(self.op).unwrap();
        write!(f, "{}(a:{},d:{})", op, self.a, self.d)
    }
}

impl Show for Instr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.decode().ty()  {
            vm::TyAD => write!(f, "{}", OpAD::from_instr(*self)),
            vm::TyABC => write!(f, "{}", OpABC::from_instr(*self))
        }
    }
}

impl Decode for Instr {
    fn decode(self) -> OpCode {
        let opcode = OpABC::from_instr(self).op;
        FromPrimitive::from_u8(opcode).unwrap()
    }
}
