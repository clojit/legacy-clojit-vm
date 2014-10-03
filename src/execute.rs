use vm;
use vm::Vm;
use vm::{Nil, Int, Float, Bool, Str, Key, Func};
use vm::Instr;

use fetch::Fetch;
use decode::Decode;
use decode::from_instr;
use decode::{OpABC, OpAD};

pub trait Execute {
    fn execute(&self, &mut Vm) -> Instr;
}

macro_rules! execute (
    (using $vm:ident with $args:ident
     $($op:pat as $ty:ty => $code:expr),+) => (

        impl Execute for Instr {
            fn execute(&self, $vm:&mut Vm) -> Instr {
                match self.decode() {
                    $($op => { let $args : $ty = from_instr(self); $code }),+
                }
            }
        }
    )
)

execute! {
    using vm with args

    vm::ADDVV as OpABC => {
        let slot1 = vm.slots.load(args.b);
        let slot2 = vm.slots.load(args.c);

        let res = match (slot1, slot2) {
            (Int(val1),   Int(val2))   => Int(val1 + val2),
            (Float(val1), Float(val2)) => Float(val1 + val2),
            (Int(val1),   Float(val2)) => Float(val1 as f64 + val2),
            (Float(val1), Int(val2))   => Float(val1 + val2 as f64),
            _ => fail!("Invalid operand types for ADDVV")
        };

        vm.slots[args.a] = res;

        vm.fetch_next()
    },

    vm::CINT as OpAD => {
        let val = vm.data.cint[args.d as uint];
        vm.slots[args.a] = Int(val);
        vm.fetch_next()
    },

    vm::CFUNC as OpAD => {
        vm.slots[args.a] = Func(args.d as uint);
        vm.fetch_next()
    },

    vm::MOV as OpAD => {
        vm.slots[args.a] = vm.slots.load(args.d);
        vm.fetch_next()
    },

    vm::CALL as OpAD  => {
        let base = args.a as uint;
        let _lit = args.d as uint;

        let offset = vm.slots.base + base;

        let func = match vm.slots.slot[offset + 1] {
            Func(func) => func,
            ref slot => fail!("Tried to execute invalid function: {}", slot)
        };

        vm.push_context();
        vm.slots.base = offset;
        vm.code.fp = func;
        vm.code.ip = 0;

        vm.fetch(0)
    },

    vm::RET as OpAD => {
        vm.slots[0u] = vm.slots.load(args.a);

        vm.pop_context();
        vm.fetch_next()
    },

    _ as Instr => {
        error!("skipping instruction: {}", args);
        vm.fetch_next()
    }
}

