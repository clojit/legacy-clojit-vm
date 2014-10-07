use vm;
use vm::Vm;
use vm::{Nil, Int, Float, Bool, Str, Key, Func};
use vm::Instr;
use vm::Context;

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

    vm::CSTR as OpAD => {
        let string = vm.data.cstr[args.d as uint].clone();
        vm.slots[args.a] = Str(string);
        vm.fetch_next()
    },

    vm::CKEY as OpAD => {
        let keyword = vm.data.ckey[args.d as uint].clone();
        vm.slots[args.a] = Key(keyword);
        vm.fetch_next()
    },

    vm::CSHORT as OpAD => {
        let integer = args.d as i64;
        vm.slots[args.a] = Int(integer);
        vm.fetch_next()
    },

    vm::CINT as OpAD => {
        let integer = vm.data.cint[args.d as uint];
        vm.slots[args.a] = Int(integer);
        vm.fetch_next()
    },

    vm::CFLOAT as OpAD => {
        let float = vm.data.cfloat[args.d as uint];
        vm.slots[args.a] = Float(float);
        vm.fetch_next()
    },

    vm::CBOOL as OpAD => {
        vm.slots[args.a] = Bool(args.d == 1);
        vm.fetch_next()
    },

    vm::CNIL as OpAD => {
        vm.slots[args.a] = Nil;
        vm.fetch_next()
    },

    vm::FNEW as OpAD => {
        let func = vm.code.ip as int + args.d as i16 as int;
        vm.slots[args.a] = Func(func as uint);
        vm.fetch_next()
    },

    vm::JUMP as OpAD => {
        let offset = args.d as i16 as int;
        vm.fetch(offset)
    },

    vm::JUMPF as OpAD => {
        let jumpf_bool = vm.slots[args.a].clone();
  
        match jumpf_bool {
            Bool(true) =>  vm.fetch_next(),
            Bool(false) => { let offset = args.d as i16 as int;
                             vm.fetch(offset) }
            _ => fail!("Invoked on non bool slot")
        }
    },

    vm::JUMPT as OpAD => {
        let jumpt_bool = vm.slots[args.a].clone();
  
        match jumpt_bool {
            Bool(false) =>  vm.fetch_next(),
            Bool(true) => { let offset = args.d as i16 as int;
                            vm.fetch(offset) }
            _ => fail!("Invoked on non bool slot")
        }
    },

    vm::MOV as OpAD => {
        vm.slots[args.a] = vm.slots.load(args.d);
        vm.fetch_next()
    },

    vm::CALL as OpAD  => {
        let base = args.a as uint;
        let lit = args.d as i64;

        vm.slots[base] = Int(lit);
        let func = match vm.slots.load(base+1) {
            Func(func) => func,
            ref slot => fail!("Tried to execute invalid function: {}", slot)
        };

        let old = vm.get_context();
        vm.stack.push(old);

        let newbase = vm.slots.base + base;
        vm.set_context(Context {
            base : newbase,
            ip : func,
        });

        vm.fetch(0)
    },

    vm::RET as OpAD => {
        vm.slots[0u] = vm.slots.load(args.a);

        let caller = vm.stack.pop().unwrap();
        vm.set_context(caller);
        vm.fetch_next()
    },

    _ as Instr => {
        error!("skipping instruction: {}", args);
        vm.fetch_next()
    }
}

