use std::mem;

use vm;
use vm::Vm;
use vm::VmSlots;

use vm::Slot;
use vm::Instr;
use vm::OpCode;

pub trait Decode {
    fn decode(self) -> OpCode;
    fn as_abc(self) -> OpABC;
    fn as_ad(self) -> OpAD;
}

pub trait Encode {
    fn as_instr(self) -> Instr;
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

impl Encode for OpABC {
    fn as_instr(self) -> Instr {
        unsafe { mem::transmute(self) }
    }
}

impl Encode for OpAD {
    fn as_instr(self) -> Instr {
        unsafe { mem::transmute(self) }
    }
}

impl Decode for Instr {
    fn decode(self) -> OpCode {
        let opcode = self.as_abc().op;
        match FromPrimitive::from_u8(opcode) {
            Some(op) => op,
            None => fail!("invalid opcode: {}", opcode)
        }
    }

    fn as_abc(self) -> OpABC {
        unsafe { mem::transmute(self) }
    }

    fn as_ad(self) -> OpAD {
        unsafe { mem::transmute(self) }
    }
}

macro_rules! emit_loadstore(
    ($ty:ident, $field:ident, $load:ident, $store:ident) => (
        impl $ty {
            pub fn $load(self, vm: &mut Vm) -> Slot {
                let slots = vm.slot[vm.ctx.base..];
                slots[self.$field as uint].clone()
            }

            pub fn $store(self, vm: &mut Vm, slot: Slot) {
                *vm.slot.get_mut(vm.ctx.base + self.$field as uint) = slot;
            }
        }
    );
)

emit_loadstore!(OpABC, a, load_a, store_a)
emit_loadstore!(OpABC, b, load_b, store_b)
emit_loadstore!(OpABC, c, load_c, store_c)

emit_loadstore!(OpAD, a, load_a, store_a)
emit_loadstore!(OpAD, d, load_d, store_d)
