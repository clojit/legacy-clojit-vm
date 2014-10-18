#![feature(macro_rules)]
#![feature(phase)]
//#![feature(struct_variant)]

extern crate serialize;
#[phase(plugin, link)] extern crate log;


use std::io;
use std::os;

use serialize::{json, Decoder, Decodable};

use vm::Vm;
use vm::Data;
use vm::DispatchData;
use vm::Code;
use vm::Keyword;
use vm::Instr;
use vm::OpCode;

use decode::ToInstr;

use std::collections::HashMap;

mod vm;
mod diag;
mod decode;
mod fetch;
mod execute;


#[deriving(Decodable, Show, Clone)]
struct JsonInstr {
    op: OpCode,
    a: Option<u8>,
    b: Option<u8>,
    c: Option<u8>,
    d: Option<i32> // either i16 or u16
}

#[deriving(Decodable, Show, Clone)]
enum IntOrType {
    I(uint),
    T(Type)
}

#[allow(non_snake_case)]
#[deriving(Decodable, Show, Clone)]
struct Method {
    func:uint,
    protocol_method_nr:uint,
    doc:Option<String>,
    arglist:Vec<String>,
    name:String,
}

#[allow(non_snake_case)]
#[deriving(Decodable, Show, Clone)]
struct Field {
    offset:uint,
    mutable:Option<bool>,    
    name:String,
    local:String,
    op:String,
    o_tag:String,
    tag:String,
}

#[deriving(Decodable, Show, Clone)]
enum OptionMethod {
    Int(uint),
    S(String),
    M(Method)
}

#[allow(non_snake_case)]
#[deriving(Decodable, Show, Clone)]
struct Type {
    fields:HashMap<String,Field>,
    nr:uint,
    class_name:String,
    interfaces:HashMap<String,HashMap<String,OptionMethod>>,
    name:String,
}

#[deriving(Clone, Show)]
enum Stuff {
    TypeInt(uint),
    TypeString(String)
}


/*
impl<D: Decoder<E>, E> Decodable<D, E> for Stuff {
    fn decode(dec: &mut D) -> Result<Instr, E> {
        let json : JsonInstr = try!(Decodable::decode(dec));
        
        match json.ty() {
            vm::TyAD => {
                Ok(decode::OpAD {
                     op: json.op as u8,
                      a: json.a.unwrap_or_default(),
                      d: json.d.unwrap_or_default() as u16,
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
}*/


#[allow(non_snake_case)]
#[deriving(Decodable, Show, Clone)]
struct JsonBytecode {
    bytecode  : Vec<Instr>,
    CINT   : Vec<i64>,
    CFLOAT : Vec<f64>,
    CSTR   : Vec<String>,
    CKEY   : Vec<Keyword>,
    vtable : HashMap<uint,HashMap<uint, uint>>,
    //types  : HashMap<String,IntOrType>,
    //types : HashMap<String,Stuff>
}
//
//  "vtable":
// {"3":{"0":42, "1":24}, "0":{"0":33}, "1":{"0":36}, "2":{"0":39}},

impl<D: Decoder<E>, E> Decodable<D, E> for Instr {
    fn decode(dec: &mut D) -> Result<Instr, E> {
        let json : JsonInstr = try!(Decodable::decode(dec));
        match json.op.ty() {
            vm::TyAD => {
                Ok(decode::OpAD {
                     op: json.op as u8,
                      a: json.a.unwrap_or_default(),
                      d: json.d.unwrap_or_default() as u16,
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

fn parse_json(path: &Path) -> Result<(Data, Code, DispatchData), json::DecoderError> {
    
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

    let dispatchdata  = DispatchData {
        vtable : bc.vtable
    };

    let code = Code {
        ip : 0,
        func : bc.bytecode
    };

    Ok((data, code, dispatchdata))
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
        Ok((data, code, dispatchdata)) => Vm::new(data, code, dispatchdata).start(),
        Err(err) => println!("{}", err)
    };
}
