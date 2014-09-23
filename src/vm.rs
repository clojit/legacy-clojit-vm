use std::collections::HashMap;
use std::default::Default;

use fetch::Fetch;
use decode::Decode;
use execute::Execute;

pub struct Instr(pub u32);

#[deriving(Show, PartialEq, FromPrimitive, Decodable)]
pub enum OpCode {
    CSTR, CKEY, CINT, CFLOAT, CBOOL, CNIL, CFUNC,
    NSSETS, NSGETS,
    ADDVV, SUBVV, MULVV, DIVVV, MODVV, POWVV,
    ISLT, ISGE, ISLE, ISGT, ISEQ, ISNEQ,
    MOV, NOT, NEG,
    JUMP, JUMPF, JUMPT,
    CALL, RET,
    APPLY,
    FNEW,
    SETFREEVAR, GETFREEVAR,
    LOOP, BULKMOV,
    NEWARRAY, GETARRAY, SETARRAY,
    FUNCF, FUNCV,
    EXIT
}

pub enum InstrType {
    TyABC,
    TyAD
}

pub type Keyword = String;

#[deriving(Show, Clone)]
pub enum Slot {
    Nil,
    Int(i64),
    Float(f64),
    Bool(bool),
    Str(String),
    Key(Keyword),
    Func(uint),
}

type CFunc  = HashMap<uint, Vec<Instr>>;
type CInt   = Vec<i64>;
type CFloat = Vec<f64>;
type CStr   = Vec<String>;
type CKey   = Vec<Keyword>;

pub struct VmContext {
    pub ip : uint,
    pub func : uint,
    pub base : uint,
}

pub struct VmData {
    pub cfunc  : CFunc,
    pub cint   : CInt,
    pub cfloat : CFloat,
    pub cstr   : CStr,
    pub ckey   : CKey
}

pub type VmSlots = Vec<Slot>;
pub type VmStack = Vec<VmContext>;

pub struct Vm {
    pub ctx   : VmContext,
    pub stack : VmStack,
    pub slot  : VmSlots,
    pub data  : VmData,
}

static VM_MAX_SLOTS : uint = 64000u;

impl Vm {
    pub fn new(data: VmData) -> Vm
    {
        Vm {
            ctx   : VmContext { ip : 0, func : 0, base : 0 },
            slot  : Vec::from_fn(VM_MAX_SLOTS, |i| Nil),
            stack : vec![],
            data  : data
        }
    }

    pub fn start(&mut self) {
        let mut instr = self.fetch(0);

        while instr.decode() != EXIT {
            let next = instr.execute(self);
            println!("{}: {}", instr, self.slot[..10]);

            instr = next;
        }
    }
}

impl OpCode {
    pub fn ty(self) -> InstrType {
        match self {
            ADDVV|SUBVV|MULVV|DIVVV|MODVV|POWVV|
            ISLT|ISGE|ISLE|ISGT|ISEQ|ISNEQ|
            APPLY|
            SETFREEVAR|GETFREEVAR|
            LOOP|BULKMOV|
            NEWARRAY|GETARRAY|SETARRAY
                => TyABC,
            CSTR|CKEY|CINT|CFLOAT|CBOOL|CNIL|CFUNC|
            NSSETS|NSGETS|
            MOV|NOT|NEG|
            JUMP|JUMPF|JUMPT|
            CALL|RET|
            FNEW|
            FUNCF|FUNCV|
            EXIT
                => TyAD,
        }
    }
}

impl Default for Slot {
    fn default() -> Slot {
        Nil
    }
}
