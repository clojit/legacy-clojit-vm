use super::{VmState};
use super::{Integer, Float, Boolean};
use super::{JumpFlag, NextInstr, Jump};
use super::{ArgA, ArgB, ArgC, ArgD};

#[inline]
pub fn cint(vm : &mut VmState, dst: ArgA, idx: ArgD) -> JumpFlag {
    let val = Integer(vm.cint[idx as uint]);
    vm.slot[dst as uint] = val;
    NextInstr
}

#[inline]
pub fn cfloat(vm : &mut VmState, dst: ArgA, idx: ArgD) -> JumpFlag {
    let val = Float(vm.cfloat[idx as uint]);
    vm.slot[dst as uint] = val;
    NextInstr
}

#[inline]
pub fn cbool(vm : &mut VmState, dst: ArgA, val: ArgD) -> JumpFlag {
    vm.slot[dst as uint] = Boolean(val != 0);
    NextInstr
}

#[inline]
pub fn addvv(vm : &mut VmState, dst: ArgA, op1: ArgB, op2: ArgC) -> JumpFlag {
    let slot1 = vm.slot[op1 as uint];
    let slot2 = vm.slot[op2 as uint];

    let res = match (slot1, slot2) {
        (Integer(val1), Integer(val2)) => Integer(val1 + val2),
        (Float(val1),   Float(val2))   => Float(val1 + val2),
        (Integer(val1), Float(val2))   => Float(val1 as f64 + val2),
        (Float(val1),   Integer(val2)) => Float(val1 + val2 as f64),
        _ => fail!("Invalid operand types for ADDVV")
    };

    vm.slot[dst as uint] = res;
    NextInstr
}

#[inline]
pub fn mov(vm : &mut VmState, dst: ArgA, src: ArgD) -> JumpFlag {
    vm.slot[dst as uint] = vm.slot[src as uint];
    NextInstr
}

#[inline]
pub fn jump(_vm : &mut VmState, addr: ArgD) -> JumpFlag {
    Jump(addr as uint)
}

#[inline]
pub fn jumpf(vm : &mut VmState, var: ArgA, addr: ArgD) -> JumpFlag {
    let addr = addr as uint;
    match vm.slot[var as uint] {
        Boolean(false) => Jump(addr),
        Integer(0)     => Jump(addr),
        Float(0.0)     => Jump(addr),
        _ => NextInstr,
    }
}

#[inline]
pub fn jumpt(vm : &mut VmState, var: ArgA, addr: ArgD) -> JumpFlag {
    let addr = addr as uint;
    match vm.slot[var as uint] {
        Boolean(true)  => Jump(addr),
        Integer(1)     => Jump(addr),
        Float(1.0)     => Jump(addr),
        _ => NextInstr,
    }
}
