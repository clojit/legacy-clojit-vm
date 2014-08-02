extern crate serialize;

use std::io::File;
use std::os;

use serialize::{json, Decoder, Decodable};

struct VmState {
    pc     : uint,
    slot   : SlotFrame,
    instr  : Vec<Instr>,
    cint   : Vec<i64>,
    cfloat : Vec<f64>,
}

#[deriving(Show, Decodable)]
enum OpCode {
    CSTR, CINT, CFLOAT, CKEY, CBOOL,
    ADDVV, SUBVV, MULVV, DIVV, MODVV, POWVV,
    ISLT, ISGE, ISLE, ISGT, ISEQ, ISNEQ,
    JUMP, JUMPF, JUMPT,
    MOV
}

type ArgA = u8;
type ArgB = u8;
type ArgC = u8;
type ArgD = u16;

#[deriving(Show)]
enum Instr {
    OpAD(OpCode, ArgA, ArgD),
    OpABC(OpCode, ArgA, ArgB, ArgC)
}

#[deriving(Default)]
struct OptionalInstr {
    op: Option<OpCode>,
    a: Option<ArgA>,
    b: Option<ArgB>,
    c: Option<ArgC>,
    d: Option<ArgD>
}

impl OptionalInstr {
    fn new() -> OptionalInstr {
        std::default::Default::default()
    }

    fn as_instr(self) -> Option<Instr> {
        match self {
            OptionalInstr {
                op: Some(op), a: Some(a), b: Some(b), c: Some(c), d: None
            } => Some(OpABC(op, a, b, c)),
            OptionalInstr {
                op: Some(op), a: Some(a), d: Some(d), b: None, c: None
            } => Some(OpAD(op, a, d)),
            _ => None
        }
    }
}

impl<D: Decoder<E>, E> Decodable<D, E> for Instr {
    fn decode(dec: &mut D) -> Result<Instr, E> {
        dec.read_map(|dec, len| {
            let mut opt_instr = OptionalInstr::new();
            for i in range(0u, len) {
                let key : String = try!(Decodable::decode(dec));
                match key.as_slice() {
                    "op" => opt_instr.op = Decodable::decode(dec).ok(),
                    "a"  => opt_instr.a  = Decodable::decode(dec).ok(),
                    "b"  => opt_instr.b  = Decodable::decode(dec).ok(),
                    "c"  => opt_instr.c  = Decodable::decode(dec).ok(),
                    "d"  => opt_instr.d  = Decodable::decode(dec).ok(),
                     _   => return Err(dec.error("Unexpected key"))
                }
            }

            match opt_instr.as_instr() {
                Some(instr) => Ok(instr),
                None => Err(dec.error("Invalid format"))
            }
        })
    }
}

#[deriving(Show, Clone)]
enum Slot {
    Nil,
    Integer(i64),
    Float(f64),
    Boolean(bool)
}

struct SlotFrame {
    offsets : Vec<uint>,
    slots   : Vec<Slot>,
}

impl SlotFrame {
    fn new() -> SlotFrame {
        SlotFrame { offsets: vec!(0), slots: Vec::new() }
    }

    fn get_offset(&self) -> uint {
        *self.offsets.last().unwrap()
    }
}

impl Index<uint, Slot> for SlotFrame {
    fn index(&self, index : &uint) -> &Slot {
        let abs_index = self.get_offset() + *index;
        if abs_index >= self.slots.len() {
            static nil: Slot = Nil;
            &nil
        } else {
            &self.slots[abs_index]
        }
    }
}

impl IndexMut<uint, Slot> for SlotFrame {
    fn index_mut(&mut self, index : &uint) -> &mut Slot {
        let abs_index = self.get_offset() + *index;
        let len = self.slots.len();
        if abs_index >= len {
            self.slots.grow(index - len + 1u, &Nil);
        }
        self.slots.get_mut(abs_index)
    }
}

#[inline]
fn eval_addvv(vm : &mut VmState, dst: ArgA, op1: ArgB, op2: ArgC) {
    let slot1 = vm.slot[op1 as uint];
    let slot2 = vm.slot[op2 as uint];

    let res = match (slot1, slot2) {
        (Integer(val1), Integer(val2)) => Integer(val1 + val2),
        (Float(val1), Float(val2))     => Float(val1 + val2),
        (Integer(val1), Float(val2))   => Float(val1 as f64 + val2),
        (Float(val1), Integer(val2))   => Float(val1 + val2 as f64),
        _ => fail!("Invalid operand types for ADDVV")
    };

    vm.slot[dst as uint] = res;
    vm.pc += 1;
}

#[inline]
fn eval_cint(vm : &mut VmState, dst: ArgA, idx: ArgD) {
    let val = Integer(vm.cint[idx as uint]);
    vm.slot[dst as uint] = val;
    vm.pc += 1;
}

#[inline]
fn eval_cfloat(vm : &mut VmState, dst: ArgA, idx: ArgD) {
    let val = Float(vm.cfloat[idx as uint]);
    vm.slot[dst as uint] = val;
    vm.pc += 1;
}

#[inline]
fn eval_mov(vm : &mut VmState, dst: ArgA, src: ArgD) {
    vm.slot[dst as uint] = vm.slot[src as uint];
    vm.pc += 1;
}

fn dispatch(vm : &mut VmState, instr : Instr) {
    match instr {
        OpAD(CINT, a, d)        => eval_cint(vm, a, d),
        OpAD(CFLOAT, a, d)      => eval_cfloat(vm, a, d),

        OpABC(ADDVV, a, b, c)   => eval_addvv(vm, a, b, c),

        OpAD(MOV, a, d)         => eval_mov(vm, a, d),

        _ => unimplemented!()
    }
}

fn main() {
    let mut reader = match os::args().slice_from(1) {
        [ref arg, ..] => File::open(&Path::new(arg.clone())),
        [] => {
            println!("usage: {} input.json", os::args()[0]);
            return;
        }
    };

    #[deriving(Decodable)]
    struct JsonTemplate {
        bytecode : Vec<Instr>,
        cint : Vec<i64>,
        cfloat : Vec<f64>,
    }

    let mut decoder = json::Decoder::new(json::from_reader(&mut reader).unwrap());
    let json : JsonTemplate = Decodable::decode(&mut decoder).unwrap();

    let mut vm = VmState {
        pc     : 0,
        slot   : SlotFrame::new(),
        instr  : json.bytecode,
        cint   : json.cint,
        cfloat : json.cfloat,
    };

    while vm.pc < vm.instr.len() {
        let instr = vm.instr[vm.pc];
        dispatch(&mut vm, instr);
    }

    println!("{}", vm.slot[0]);
}
