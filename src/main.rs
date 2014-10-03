#![feature(macro_rules)]
#![feature(phase)]
//#![feature(struct_variant)]

extern crate serialize;
#[phase(plugin, link)] extern crate log;

use std::io;
use std::os;

use std::collections::HashMap;

use serialize::{json, Decoder, Decodable};

use vm::Vm;
use vm::Data;
use vm::Code;
use vm::Keyword;
use vm::Instr;
use vm::OpCode;

use decode::ToInstr;

mod vm;
mod diag;
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
    CKEY   : Vec<Keyword>
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
                    }.to_instr())
            },
            vm::TyABC => {
                Ok(decode::OpABC {
                     op: json.op as u8,
                      a: json.a.unwrap_or_default(),
                      b: json.b.unwrap_or_default(),
                      c: json.c.unwrap_or_default(),
                    }.to_instr())
            },
        }
    }
}

fn parse_json(path: &Path) -> Result<(Data, Code), json::DecoderError> {
    let mut reader = match io::File::open(path) {
        Ok(reader) => reader,
        Err(err) => return Err(json::ParseError(
                        json::IoError(err.kind, err.desc)))
    };

    let mut decoder = match json::from_reader(&mut reader) {
        Ok(json) => json::Decoder::new(json),
        Err(err) => return Err(json::ParseError(err))
    };

    let bc : JsonBytecode = try!(Decodable::decode(&mut decoder));

    let data = Data {
        cint   : bc.CINT,
        cfloat : bc.CFLOAT,
        cstr   : bc.CSTR,
        ckey   : bc.CKEY
    };

    let code = Code {
        ip : 0,
        fp : 0,
        func : bc.CFUNC
    };

    Ok((data, code))
}


fn main() {
    let args = os::args();
    if args.len() < 2 {
        println!("usage: {} input.json", args[0]);
        os::set_exit_status(1);
        return;
    }

    let path = Path::new(args[1].as_slice());

    match parse_json(&path) {
        Ok((data, code)) => Vm::new(data, code).start(),
        Err(err) => println!("{}", err)
    };
}
