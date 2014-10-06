use std::default::Default;

use fetch::Fetch;
use decode::Decode;
use execute::Execute;

pub struct Instr(pub u32);

#[deriving(Show, PartialEq, FromPrimitive, Decodable)]
pub enum OpCode {
    CSTR, CKEY, CINT, CSHORT, CFLOAT, CBOOL, CNIL,
    NSSETS, NSGETS,
    ADDVV, SUBVV, MULVV, DIVVV, MODVV, POWVV,
    ISLT, ISGE, ISLE, ISGT, ISEQ, ISNEQ,
    MOV, NOT, NEG,
    JUMP, JUMPF, JUMPT,
    CALL, RET,
    APPLY,
    FNEW,
    DROP, UCLO,
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

type CFunc  = Vec<Instr>;
type CInt   = Vec<i64>;
type CFloat = Vec<f64>;
type CStr   = Vec<String>;
type CKey   = Vec<Keyword>;

pub type InstrPtr = uint;
pub type FuncPtr = uint;
pub type BasePtr = uint;

pub struct Data {
    pub cint   : CInt,
    pub cfloat : CFloat,
    pub cstr   : CStr,
    pub ckey   : CKey
}

pub struct Code {
    pub ip : InstrPtr,
    pub func : CFunc,
}

pub struct Slots {
    pub base : BasePtr,
    pub slot : Vec<Slot>,
}

pub struct Context {
    pub base : BasePtr,
    pub ip : InstrPtr,
}

type Stack = Vec<Context>;

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

    pub fn get_context(&self) -> Context {
        Context {
            base : self.slots.base,
            ip : self.code.ip,
        }
    }

    pub fn set_context(&mut self, ctx: Context) {
        self.slots.base = ctx.base;
        self.code.ip = ctx.ip;
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
            CSTR|CKEY|CINT|CFLOAT|CSHORT|CBOOL|CNIL|
            NSSETS|NSGETS|
            MOV|NOT|NEG|
            JUMP|JUMPF|JUMPT|
            CALL|RET|
            FNEW|
            DROP|UCLO|
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
