extern crate serialize;

use std::io::File;
use std::os;

use std::collections::HashMap;

use serialize::{json, Decoder, Decodable};

use vm::Instr;
use vm::OpCode;

mod vm;
mod decode;
mod fetch;
mod execute;

#[deriving(Decodable, Show)]
struct JsonInstr {
    op: OpCode,
    a: Option<u8>,
    b: Option<u8>,
    c: Option<u8>,
    d: Option<u16>
}

#[allow(non_snake_case)]
#[deriving(Decodable, Show)]
struct JsonBytecode {
    CFUNC  : HashMap<uint, Vec<Instr>>,
    CINT   : Vec<i64>,
    CFLOAT : Vec<f64>,
    CSTR   : Vec<String>,
    CKEY   : Vec<String>
}

impl<D: Decoder<E>, E> Decodable<D, E> for Instr {
    fn decode(dec: &mut D) -> Result<Instr, E> {
        let json : JsonInstr = try!(Decodable::decode(dec));
        match json.op.ty() {
            vm::TyAD => {
                Ok(decode::OpAD {
                     op: json.op as u8,
                      a: json.a.unwrap_or_default(),
                      d: json.d.unwrap_or_default(),
                    }.as_instr())
            },
            vm::TyABC => {
                Ok(decode::OpABC {
                     op: json.op as u8,
                      a: json.a.unwrap_or_default(),
                      b: json.b.unwrap_or_default(),
                      c: json.c.unwrap_or_default(),
                    }.as_instr())
            },
        }
    }
}

fn main() {
    let mut reader = match os::args().slice_from(1) {
        [ref arg, ..] => File::open(&Path::new(arg.clone())),
        [] => {
            println!("usage: {} input.json", os::args()[0]);
            os::set_exit_status(1);
            return;
        }
    };

    let mut decoder = json::Decoder::new(json::from_reader(&mut reader).unwrap());
    let json : JsonBytecode = Decodable::decode(&mut decoder).unwrap();

    println!("{}", json);
}
