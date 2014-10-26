
use vm::Vm;
use vm::{Nil};



// Built in function have there own context. They have to set the base slot as the return value.

pub fn println(vm: &mut Vm) {
     println!("{}", vm.slots.load(2u).clone());
     vm.slots.store(0u, Nil);
}
