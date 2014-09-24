use std::fmt;
use std::fmt::Show;
use std::num::FromPrimitive;

use vm;
use vm::Instr;
use vm::OpCode;

use decode::Decode;
use decode::OpAD;
use decode::OpABC;

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
            vm::TyAD => write!(f, "{}", self.as_ad()),
            vm::TyABC => write!(f, "{}", self.as_abc())
        }
    }
}
