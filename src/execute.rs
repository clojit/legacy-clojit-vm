use vm;
use vm::Vm;
use vm::Closure;
use vm::{Nil, Int, Float, Bool, Str, Key, Func, VFunc, Obj, CType, SCC, Builtin};
use vm::Instr;
use vm::Context;
use vm::TopLevelBinding;

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

    // ---------------------- Const Ops ----------------------

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

    vm::CTYPE as OpAD => {
        vm.slots[args.a] = CType(args.d as uint);
        vm.fetch_next()
    },

    // -------------------- Global Table Ops ------------------

    vm::NSSETS as OpABC => {

        vm.symbol_table.insert(vm.data.cstr[args.b as uint].clone(),
                                 TopLevelBinding {
                                    val:vm.slots.load(args.a),
                                    dynamic: match args.c as uint {
                                                1u => true,
                                                _ => false
                                             } 
                                 }
                               );

        vm.fetch_next()
    },

    vm::NSGETS as OpAD => {

        let symbol = vm.data.cstr[args.d as uint].clone();

        let value = match vm.symbol_table.find(&symbol) {
            Some(toplvlbinging) => toplvlbinging.val.clone(),
            None => panic!("Symbol not found in symbol_table")
        };

        vm.slots[args.a] = value;

        vm.fetch_next()
    },

    // ---------------------- Math ----------------------

    vm::ADDVV as OpABC => {
        let slot1 = vm.slots.load(args.b);
        let slot2 = vm.slots.load(args.c);

        let res = match (slot1, slot2) {
            (Int(val1),   Int(val2))   => Int(val1 + val2),
            (Float(val1), Float(val2)) => Float(val1 + val2),
            (Int(val1),   Float(val2)) => Float(val1 as f64 + val2),
            (Float(val1), Int(val2))   => Float(val1 + val2 as f64),
            _ => panic!("Invalid operand types for ADDVV")
        };

        vm.slots[args.a] = res;

        vm.fetch_next()
    },

    vm::SUBVV as OpABC => {
        let slot1 = vm.slots.load(args.b);
        let slot2 = vm.slots.load(args.c);

        let res = match (slot1, slot2) {
            (Int(val1),   Int(val2))   => Int(val1 - val2),
            (Float(val1), Float(val2)) => Float(val1 - val2),
            (Int(val1),   Float(val2)) => Float(val1 as f64 - val2),
            (Float(val1), Int(val2))   => Float(val1 - val2 as f64),
            _ => panic!("Invalid operand types for SUBVV")
        };

        vm.slots[args.a] = res;

        vm.fetch_next()
    },

    vm::MULVV as OpABC => {
        let slot1 = vm.slots.load(args.b);
        let slot2 = vm.slots.load(args.c);

        let res = match (slot1, slot2) {
            (Int(val1),   Int(val2))   => Int(val1 * val2),
            (Float(val1), Float(val2)) => Float(val1 * val2),
            (Int(val1),   Float(val2)) => Float(val1 as f64 * val2),
            (Float(val1), Int(val2))   => Float(val1 * val2 as f64),
            _ => panic!("Invalid operand types for MULVV")
        };

        vm.slots[args.a] = res;

        vm.fetch_next()
    },

    vm::DIVVV as OpABC => {
        let slot1 = vm.slots.load(args.b);
        let slot2 = vm.slots.load(args.c);

        let res = match (slot1, slot2) {
            (Int(val1),   Int(val2))   => Float(val1 as f64 / val2 as f64),
            (Float(val1), Float(val2)) => Float(val1 / val2),
            (Int(val1),   Float(val2)) => Float(val1 as f64 / val2),
            (Float(val1), Int(val2))   => Float(val1 / val2 as f64),
            _ => panic!("Invalid operand types for DIVVV")
        };

        vm.slots[args.a] = res;

        vm.fetch_next()
    },

    vm::MODVV as OpABC => {
        let slot1 = vm.slots.load(args.b);
        let slot2 = vm.slots.load(args.c);

        let res = match (slot1, slot2) {
            (Int(val1),   Int(val2))   => Int(val1 % val2),
            (Float(val1), Float(val2)) => Float(val1 % val2),
            (Int(val1),   Float(val2)) => Float(val1 as f64 % val2),
            (Float(val1), Int(val2))   => Float(val1 % val2 as f64),
            _ => panic!("Invalid operand types for MOVVV")
        };

        vm.slots[args.a] = res;

        vm.fetch_next()
    },


    // ---------------------- Comparisent ----------------------

    vm::ISEQ as OpABC => {
        let slot1 = vm.slots.load(args.b);
        let slot2 = vm.slots.load(args.c);

        let res = match (slot1, slot2) {
            (Int(val1),   Int(val2))   => Bool(val1 == val2),
            (Float(val1), Float(val2)) => Bool(val1 == val2),
            (Int(val1),   Float(val2)) => Bool(val1 as f64 == val2),
            (Float(val1), Int(val2))   => Bool(val1 == val2 as f64),
            _ => panic!("Invalid operand types for ISEQ")
        };

        vm.slots[args.a] = res;
        vm.fetch_next()
    },

    vm::ISNEQ as OpABC => {
        let slot1 = vm.slots.load(args.b);
        let slot2 = vm.slots.load(args.c);

        let res = match (slot1, slot2) {
            (Int(val1),   Int(val2))   => Bool(val1 != val2),
            (Float(val1), Float(val2)) => Bool(val1 != val2),
            (Int(val1),   Float(val2)) => Bool(val1 as f64 != val2),
            (Float(val1), Int(val2))   => Bool(val1 != val2 as f64),
            _ => panic!("Invalid operand types for ISEQ")
        };

        vm.slots[args.a] = res;
        vm.fetch_next()
    },

    /*
    vm::ISLT as OpABC => {
        let slot1 = vm.slots.load(args.b);
        let slot2 = vm.slots.load(args.c);

        let res = match (slot1, slot2) {
            (Int(val1),   Int(val2))   => Bool(val1 > val2),
            (Float(val1), Float(val2)) => Bool(val1 > val2),
            (Int(val1),   Float(val2)) => Bool(val1 as f64 > val2),
            (Float(val1), Int(val2))   => Bool(val1 > val2 as f64),
            _ => panic!("Invalid operand types for ISLT")
        };

        vm.slots[args.a] = match res { true => false, false => true };
        vm.fetch_next()
    },*/

    vm::ISGE as OpABC => {
        let slot1 = vm.slots.load(args.b);
        let slot2 = vm.slots.load(args.c);

        let res = match (slot1, slot2) {
            (Int(val1),   Int(val2))   => Bool(val1 >= val2),
            (Float(val1), Float(val2)) => Bool(val1 >= val2),
            (Int(val1),   Float(val2)) => Bool(val1 as f64 >= val2),
            (Float(val1), Int(val2))   => Bool(val1 >= val2 as f64),
            _ => panic!("Invalid operand types for ISGE")
        };

        vm.slots[args.a] = res;
        vm.fetch_next()
    },

    vm::ISLE as OpABC => {
        let slot1 = vm.slots.load(args.b);
        let slot2 = vm.slots.load(args.c);

        let res = match (slot1, slot2) {
            (Int(val1),   Int(val2))   => Bool(val1 <= val2),
            (Float(val1), Float(val2)) => Bool(val1 <= val2),
            (Int(val1),   Float(val2)) => Bool(val1 as f64 <= val2),
            (Float(val1), Int(val2))   => Bool(val1 <= val2 as f64),
            _ => panic!("Invalid operand types for ISLE")
        };

        vm.slots[args.a] = res;
        vm.fetch_next()
    },

    vm::ISGT as OpABC => {
        let slot1 = vm.slots.load(args.b);
        let slot2 = vm.slots.load(args.c);

        let res = match (slot1, slot2) {
            (Int(val1),   Int(val2))   => Bool(val1 > val2),
            (Float(val1), Float(val2)) => Bool(val1 > val2),
            (Int(val1),   Float(val2)) => Bool(val1 as f64 > val2),
            (Float(val1), Int(val2))   => Bool(val1 > val2 as f64),
            _ => panic!("Invalid operand types for ISGT")
        };

        vm.slots[args.a] = res;
        vm.fetch_next()
    },

    // --------- Closures and Free Variables ----------

    vm::FNEW as OpAD => {

        let func_slot = vm.slots.load(1u);
  
        let new_fnew = match func_slot {
                        SCC(clos) => {SCC(Closure{func:args.d as uint,
                                                  freevar:clos.freevar.clone()})
                                     }
                        _ =>  Func(args.d as uint),
                      };

        vm.slots[args.a] = new_fnew;

        vm.fetch_next()
    },

    vm::VFNEW as OpAD => {
        let vfunc = args.d as int;
        vm.slots[args.a] = VFunc(vfunc as uint);
        vm.fetch_next()
    },

    // ---------------------- JUMPs ----------------------

    vm::JUMP as OpAD => {
        let offset = args.d as i16 as int;
        vm.fetch(offset)
    },

    vm::JUMPF as OpAD => {
        let jumpf_bool = vm.slots.load(args.a);

        match jumpf_bool {
            Nil | Bool(false) => { let offset = args.d as i16 as int;
                                   vm.fetch(offset) }
            _ => vm.fetch_next()

        }
    },

    vm::JUMPT as OpAD => {
        let jumpt_bool = vm.slots.load(args.a);

        match jumpt_bool {
            Nil | Bool(false) => vm.fetch_next(),
            _ => { let offset = args.d as i16 as int;
                   vm.fetch(offset) }
        }
    },

    // ---------------------- Unary Ops ----------------------

    vm::MOV as OpAD => {
        vm.slots[args.a] = vm.slots.load(args.d);
        vm.fetch_next()
    },

    vm::NOT as OpAD => {
        let src_slot = vm.slots.load(args.d);

        let dst_val = match src_slot {
            Bool(false) | Nil =>  Bool(true),
            _ => Bool(false)
        };

        vm.slots.store(args.a, dst_val);
        vm.fetch_next()
    },

    vm::NEG as OpAD => {
        let src_slot = vm.slots.load(args.d);

        let dst_val = match src_slot {
            Int(val)   => Int(- val),
            Float(val) => Float(- val),
            _ => panic!("Tried to execute invalid bytecode, NOT")
        };
        vm.slots.store(args.a, dst_val);
        vm.fetch_next()
    },


    // ----------- Tail Recursion and Loops ------------

    vm::BULKMOV as OpABC => {

        for x in range(0, args.c) {
            //vm.slots.mov(args.a+x, args.b+x)
            vm.slots[args.a+x] = vm.slots.load(args.b+x);
        }

        vm.fetch_next()
    },

    vm::LOOP as OpABC => {
        vm.fetch_next()
    },

    // ---------------------- Function Calls ----------------------

    vm::CALL as OpAD  => {
        let base = args.a as uint;
        let lit = args.d as i64;
        
        vm.slots[base] = Int(lit);

        let func = match vm.slots.load(base+1) {
            VFunc(vfunc) => {let type_int = match vm.slots.load(base+2) {
                                Obj(val)  => val.cljtype,
                                ref slot => panic!("Stack not ready, base+2 
                                                   is not of type CType: {}", slot)
                             };
                             vm.dd.vtable[vfunc][type_int] }
            Func(func)   => func,
            SCC(clos)    => clos.func,
            Builtin(_)   => -1,
            ref slot     => panic!("Tried to execute invalid function 2: {}", slot)
        };

        let old = vm.get_context();
        vm.stack.push(old);

        let newbase = vm.slots.base + base;
        vm.set_context(Context {
            base : newbase,
            ip : func,
        });

 
        match vm.slots.load(base+1) {
                Builtin(f) => { f(vm);

                                for i in range(2, args.a as int + 10) {
                                    vm.slots.store(i, Nil);
                                }

                                let caller = vm.stack.pop().unwrap();
                                vm.set_context(caller);

                                vm.fetch_next()
                               }
                _ => vm.fetch(0)
        }
    },

    vm::RET as OpAD => {
        vm.slots[0u] = vm.slots.load(args.a);
        
        //should use tranc
        for i in range(2, args.a as int + 10) {
            vm.slots.store(i, Nil);
        }
    

        let caller = vm.stack.pop().unwrap();
        vm.set_context(caller);

        vm.fetch_next()
    },

    // --------------- Function Def ------------

    vm::FUNCF as OpAD => {
        vm.fetch_next()
    },

    vm::FUNCV as OpAD => {
        vm.fetch_next()
    },

    // ---------------- Types ------------------

//    OP       A       D
//    ALLOC    dst     type
//    ALLOC allocates the empty instance of Type D and puts a reference to into dst.

    vm::ALLOC as OpAD => {
        let index = match vm.slots.load(args.d as uint) {
            CType(index) => index as uint,
            _ => panic!("ALLOC Failed!")
        };

        let t = vm.data.ctype[index].clone();

        vm.slots.store(args.a as uint, Obj(t.alloc()) );

        vm.fetch_next()
    },

//    OP        A    B            C
//    SETFIELD  ref  offset(lit)  var

    vm::SETFIELD as OpABC => {
        let var = vm.slots.load(args.c as uint);

        let offset = args.b as uint;

        let ref_index = args.a as uint;

        let obj = vm.slots.load(ref_index).clone();  

        let mut inside_obj = match obj {
            Obj(sobj) =>  sobj,
            _ => panic!("SETFIELD Failed!")
        };
        
        *inside_obj.fields.get_mut(offset) = var;
        
        let myobj = inside_obj;
        vm.slots.store(ref_index, Obj(myobj) );

        vm.fetch_next()
    },


    //OP        A    B     C
    //GETFIELD  dst  ref   offset(lit)
    
    vm::GETFIELD as OpABC => {
        
        let dst_index = args.a as uint;
        let slot_refr = vm.slots.load(args.b as uint).clone();

        let refr = match slot_refr {
            Obj(sobj) => sobj,
            _ => panic!("GETFIELD Failed!")
        };

        let offset = args.c as uint;
        let dst = refr.fields[offset].clone();
        
        vm.slots.store(dst_index, dst);

        vm.fetch_next()
    },

    // -------------------- Closure -------------------

    vm::UCLO as OpAD => {
        
        let start_slot = args.a as uint;
        let end_slot = args.d as uint;
        let fnew_slot_index = end_slot + 1;

        let new_freevars = vm.slots[start_slot..end_slot+1].to_vec();

        match vm.slots.load(fnew_slot_index) {
            Func(func) => {vm.slots.store(fnew_slot_index, SCC(Closure{func:func,
                                                                       freevar:new_freevars}));
                          }
            SCC(clos) =>  {let mut new_clos = clos.clone();
                           new_clos.freevar.push_all(vm.slots[start_slot..end_slot+1]);
                           vm.slots.store(fnew_slot_index, SCC( new_clos.clone()));
                          }
            _ => panic!("Not Func type on FNEW slot")             
        };
      
        vm.fetch_next()
    },

    //    OP      A       D
    //GETFREEVAR  dst     idx

    vm::GETFREEVAR as OpAD => {

        let idx = args.d as uint;
        let dst_idx = args.a as uint;

        let freevar = match vm.slots.load(1u) {
                        SCC(clos) => clos.freevar[idx].clone(),
                        _ => panic!("Not Closure in Slot") 
                      };


        vm.slots.store(dst_idx, freevar);

        vm.fetch_next()
    },


    // --------------- Run-Time Behavior ------------

    vm::DROP as OpAD => {

        for i in range(args.a as int, 1 + args.d as int) {
            vm.slots.store(i, Nil);
        }

        vm.fetch_next()
    },

    // Currently very very slow
    vm::TRANC as OpAD => {

        for i in range(args.d as uint, vm.slots.slot.len() ) {
            vm.slots.store(i, Nil);
        }

        vm.fetch_next()
    },


    _ as Instr => {
        error!("skipping instruction: {}", args);
        vm.fetch_next()
    }
}

