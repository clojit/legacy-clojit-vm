use std::collections::HashMap;

pub type Keyword = String;

pub struct Instr(pub u32);

#[deriving(Show, FromPrimitive, Decodable)]
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
    FUNCF, FUNCV
}

pub enum Slot {
    Nil,
    Integer(i64),
    Float(f64),
    Boolean(bool),
    String(String),
    Keyword(Keyword)
}

struct Frame {
    ret  : InstrPtr,
    base : uint,
}

type InstrPtr = (uint, uint);

type CFunc  = HashMap<uint, Vec<Instr>>;
type CInt   = Vec<i64>;
type CFloat = Vec<f64>;
type CStr   = Vec<String>;
type CKey   = Vec<Keyword>;

pub struct Vm {
    ip : InstrPtr,
    slots  : Vec<Slot>,
    frames : Vec<Frame>,

    cfunc  : CFunc,
    cint   : CInt,
    cfloat : CFloat,
    cstr   : CStr,
    ckey   : CKey
}


impl Vm {
    pub fn new(cfunc: CFunc, cint: CInt, cfloat:CFloat, cstr:CStr, ckey:CKey) -> Vm {
        Vm {
            ip     : (0, 0),
            slots  : vec!(),
            frames : vec!(),
            cfunc  : cfunc,
            cint   : cint,
            cfloat : cfloat,
            cstr   : cstr,
            ckey   : ckey
        }
    }
}

pub enum InstrType {
    TyABC,
    TyAD
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
            FUNCF|FUNCV
                => TyAD,
        }
    }
}
