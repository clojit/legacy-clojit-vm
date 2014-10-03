use vm::Vm;
use vm::Instr;

pub trait Fetch {
    fn fetch(&mut self, offset: uint) -> Instr;
    fn fetch_next(&mut self) -> Instr;
}

impl Fetch for Vm {
    fn fetch(&mut self, offset: uint) -> Instr {
        let code = &mut self.code;
        let func = &code.func[code.fp];

        code.ip += offset;

        func[code.ip]
    }

    fn fetch_next(&mut self) -> Instr {
        self.fetch(1)
    }
}
