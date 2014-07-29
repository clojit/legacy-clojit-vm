extern crate serialize;

use serialize::{json, Decodable, Decoder};
use serialize::json::{DecoderError, ExpectedError};

#[deriving(Show, Decodable)]
enum OpCode {
    CINT, CSTR,
    ADDVV, SUBVV, MULVV, DIVV, MODVV
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

    fn unwrap(self) -> Instr {
        match self {
            OptionalInstr { op: Some(op), a: Some(a), b: Some(b), c: Some(c), d: None }
                => OpABC(op, a, b, c),
            OptionalInstr { op: Some(op), a: Some(a), b: None, c: None, d: Some(d) }
                => OpAD(op, a, d),
            _
                => fail!("Failed to unwrap instruction")
        }
    }
}

impl Decodable<json::Decoder, DecoderError> for Instr {
    fn decode(dec: &mut json::Decoder) -> Result<Instr, DecoderError> {   
        dec.read_map(|dec, len| {
            let mut instr = OptionalInstr::new();

            for i in range(0u, len) {
                let key : String = try!(Decodable::decode(dec));
                match key.as_slice() {
                    "op" => instr.op = Decodable::decode(dec).ok(),
                    "a"  => instr.a  = Decodable::decode(dec).ok(),
                    "b"  => instr.b  = Decodable::decode(dec).ok(),
                    "c"  => instr.c  = Decodable::decode(dec).ok(),
                    "d"  => instr.d  = Decodable::decode(dec).ok(),
                     _   => return Err(ExpectedError("op|a|b|c|d".to_string(), key))
                }
            }

            Ok(instr.unwrap())
        })
    }
}

fn main() {
    let input_str = r#"
        [{"op":"CINT","a":0,"d":0},
         {"op":"CINT","a":1,"d":1},
         {"op":"ADDVV","a":0,"b":0,"c":1},
         {"op":"CINT","a":1,"d":2},
         {"op":"ADDVV","a":0,"b":0,"c":1}]
    "#;

    let instr : Vec<Instr> = json::decode(input_str).unwrap();
    println!("{}", instr);
}
