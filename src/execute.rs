use vm;
use vm::Vm;
use vm::VmContext;
use vm::{Nil, Int, Float, Bool, Str, Key, Func};
use vm::Instr;

use fetch::Fetch;
use decode::Decode;
use decode::{OpABC, OpAD};

pub trait Execute {
    fn execute(&self, &mut Vm) -> Instr;
}

fn exec_addvv(vm: &mut Vm, args: OpABC) -> Instr {
    let slot1 = args.load_b(vm);
    let slot2 = args.load_c(vm);

    let res = match (slot1, slot2) {
        (Int(val1),   Int(val2))   => Int(val1 + val2),
        (Float(val1), Float(val2)) => Float(val1 + val2),
        (Int(val1),   Float(val2)) => Float(val1 as f64 + val2),
        (Float(val1), Int(val2))   => Float(val1 + val2 as f64),
        _ => fail!("Invalid operand types for ADDVV")
    };

    args.store_a(vm, res);

    vm.fetch_next()
}

fn exec_cint(vm: &mut Vm, args: OpAD) -> Instr {
    let val = vm.data.cint[args.d as uint];
    args.store_a(vm, Int(val));

    vm.fetch_next()
}

fn exec_cfunc(vm: &mut Vm, args: OpAD) -> Instr {
    args.store_a(vm, Func(args.d as uint));

    vm.fetch_next()
}

fn exec_mov(vm: &mut Vm, args: OpAD) -> Instr {
    let val = args.load_d(vm);
    args.store_a(vm, val);

    vm.fetch_next()
}
fn exec_call(vm: &mut Vm, args: OpAD) -> Instr {
    let base = args.a as uint;
    let _lit = args.d as uint;

    let offset = vm.ctx.base + base;

    let func = match vm.slot[offset + 1] {
        Func(func) => func,
        ref slot => fail!("Tried to execute invalid function: {}", slot)
    };

    vm.stack.push(vm.ctx);
    vm.ctx = VmContext { base: offset, func: func, ip: 0 };

    vm.fetch_next()
}

fn exec_ret(vm: &mut Vm, args: OpAD) -> Instr {
    let ret = args.load_a(vm);
    *vm.slot.get_mut(vm.ctx.base) = ret;

    vm.ctx = vm.stack.pop().unwrap();

    vm.fetch_next()
}

fn exec_default(vm: &mut Vm, instr: Instr) -> Instr {
    error!("skipping instruction: {}", instr);
    vm.fetch_next()
}

impl Execute for Instr {
    fn execute(&self, vm:&mut Vm) -> Instr {
        match self.decode() {
            vm::ADDVV       => exec_addvv(vm, self.as_abc()),
            vm::CINT        => exec_cint(vm, self.as_ad()),
            vm::CFUNC       => exec_cfunc(vm, self.as_ad()),
            vm::MOV         => exec_mov(vm, self.as_ad()),
            vm::CALL        => exec_call(vm, self.as_ad()),
            vm::RET         => exec_ret(vm, self.as_ad()),
            _               => exec_default(vm, *self)
        }
    }
}
