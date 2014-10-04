use vm::Vm;
use vm::Instr;

pub trait Fetch {
    fn fetch(&mut self, offset: uint) -> Instr;
    fn fetch_next(&mut self) -> Instr;
}

impl Fetch for Vm {
    fn fetch(&mut self, offset: uint) -> Instr {
        self.code.ip += offset;
        self.code.func[self.code.ip]
    }

    fn fetch_next(&mut self) -> Instr {
        self.fetch(1)
    }
}
