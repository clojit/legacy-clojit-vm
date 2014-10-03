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

type InstrPtr = uint;
type FuncPtr = uint;
type BasePtr = uint;

pub struct Data {
    pub cint   : CInt,
    pub cfloat : CFloat,
    pub cstr   : CStr,
    pub ckey   : CKey
}

pub struct Code {
    pub fp : FuncPtr,
    pub ip : InstrPtr,
    pub func : CFunc,
}

pub struct Slots {
    pub base : BasePtr,
    pub slot : Vec<Slot>,
}

type Stack = Vec<(BasePtr, InstrPtr, BasePtr)>;

pub struct Vm {
    pub stack : Stack,
    pub slots : Slots,
    pub data  : Data,
    pub code  : Code
}

static VM_MAX_SLOTS : uint = 64000u;

impl Slots {
    pub fn new() -> Slots {
        Slots {
            base : 0,
            slot : Vec::from_fn(VM_MAX_SLOTS, |_| Nil),
        }
    }
}

impl Vm {
    pub fn new(data: Data, code: Code) -> Vm
    {
        Vm {
            stack : vec![],
            slots : Slots::new(),
            code  : code,
            data  : data
        }
    }

    pub fn start(&mut self) {
        let mut instr = self.fetch(0);

        while instr.decode() != EXIT {
            let next = instr.execute(self);
            println!("{}: {}", instr, self.slots.slot[..10]);

            instr = next;
        }
    }

    pub fn push_context(&mut self) {
        self.stack.push((self.slots.base, self.code.ip, self.code.fp));
    }

    pub fn pop_context(&mut self) {
        let (base, ip, fp) = self.stack.pop().unwrap();

        self.slots.base = base;
        self.code.ip = ip;
        self.code.fp = fp;
    }
}

impl<I:ToPrimitive> IndexMut<I, Slot> for Slots {
    fn index_mut(&mut self, index: &I) -> &mut Slot {
        let index = index.to_uint().unwrap();
        self.slot.get_mut(self.base + index)
    }
}

impl<I:ToPrimitive> Index<I, Slot> for Slots {
    fn index(&self, index: &I) -> &Slot {
        let index = index.to_uint().unwrap();
        &self.slot[self.base + index]
    }
}

impl Slots {
    pub fn load<I:ToPrimitive>(&self, index: I) -> Slot {
        self[index].clone()
    }

    pub fn store<I:ToPrimitive>(&mut self, index: I, val: Slot) {
        self[index] = val
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
