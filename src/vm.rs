use std::default::Default;
use std::collections::HashMap;

use fetch::Fetch;
use decode::Decode;
use execute::Execute;

#[deriving(Clone)]
pub struct Instr(pub u32);

#[deriving(Show, PartialEq, FromPrimitive, Decodable, Clone)]
pub enum OpCode {
    CSTR, CKEY, CINT, CSHORT, CFLOAT, CBOOL, CNIL,CTYPE,
    NSSETS, NSGETS,
    ADDVV, SUBVV, MULVV, DIVVV, MODVV, POWVV,
    ISLT, ISGE, ISLE, ISGT, ISEQ, ISNEQ,
    MOV, NOT, NEG,
    JUMP, JUMPF, JUMPT,
    CALL, RET,
    APPLY,
    FNEW, VFNEW,
    DROP, UCLO,
    SETFREEVAR, GETFREEVAR,
    LOOP, BULKMOV,
    NEWARRAY, GETARRAY, SETARRAY,
    ALLOC, SETFIELD, GETFIELD,
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
    VFunc(uint),
    Obj(CljObject),
    CType(uint)
}

type CFunc  = Vec<Instr>;
type CInt   = Vec<i64>;
type CFloat = Vec<f64>;
type CStr   = Vec<String>;
type CKey   = Vec<Keyword>;
type VTable = HashMap<uint,HashMap<uint,uint>>;
type Types  = Vec<CljType>;
type Fields = Vec<CljField>;
type RawSlots = Vec<Slot>;

pub type InstrPtr = uint;
pub type FuncPtr = uint;
pub type BasePtr = uint;

pub struct Data {
    pub cint   : CInt,
    pub cfloat : CFloat,
    pub cstr   : CStr,
    pub ckey   : CKey,
    pub ctype  : Types
}

#[deriving(Decodable, Show, Clone)]
pub struct CljType {
    name:String,
    nr:uint,
    size:uint,
    fields:Fields
}

#[deriving(Show, Clone)]
struct CljObject {
    pub cljtype:uint,
    pub fields:RawSlots
}

#[deriving(Decodable, Show, Clone)]
pub struct CljField {
    pub name:String,
    pub offset:uint,
    pub mutable:bool 
}

#[deriving(Show)]
pub struct DispatchData {
    pub vtable : VTable
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
    pub code  : Code,
    pub dd    : DispatchData,
    pub symbol_table : HashMap<String, Slot>
}

static VM_MAX_SLOTS : uint = 64000u;

impl CljType {
    pub fn alloc(&self) -> CljObject {

        let mut obj = CljObject { cljtype: self.nr, fields: vec![] };

        for n in range(0, self.size) {
            obj.fields.push(Nil);
        }
        obj
    }
}

impl Slots {
    pub fn new() -> Slots {
        Slots {
            base : 0,
            slot : Vec::from_fn(VM_MAX_SLOTS, |_| Nil),
        }
    }
}

impl Vm {
    pub fn new(data: Data, code: Code, dd : DispatchData) -> Vm
    {
        Vm {
            stack : vec![],
            slots : Slots::new(),
            code  : code,
            data  : data,
            dd    : dd,
            symbol_table : HashMap::new()
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

    //pub fn mov<I:ToPrimitive>(&mut self, dst_index: I, src_index: I) {
    //    let src_slot = self.load(src_index);
    //    self.store(dst_index, src_slot);
    //}
}

impl OpCode {
    pub fn ty(self) -> InstrType {
        match self {
            ADDVV|SUBVV|MULVV|DIVVV|MODVV|POWVV|
            ISLT|ISGE|ISLE|ISGT|ISEQ|ISNEQ|
            APPLY|
            SETFREEVAR|GETFREEVAR|
            GETFIELD|SETFIELD|
            LOOP|BULKMOV|
            NEWARRAY|GETARRAY|SETARRAY
                => TyABC,
            CSTR|CKEY|CINT|CFLOAT|CSHORT|CBOOL|CNIL|CTYPE|
            NSSETS|NSGETS|
            MOV|NOT|NEG|
            JUMP|JUMPF|JUMPT|
            CALL|RET|
            FNEW| VFNEW|
            DROP|UCLO|
            FUNCF|FUNCV|
            ALLOC|
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
