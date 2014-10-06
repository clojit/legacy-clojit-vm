use vm::Vm;
use vm::Instr;

pub trait Fetch {
    fn fetch(&mut self, offset: int) -> Instr;
    fn fetch_next(&mut self) -> Instr;
}

impl Fetch for Vm {
    fn fetch(&mut self, offset: int) -> Instr {
        let new_ip = self.code.ip as int + offset;
        self.code.ip = new_ip as uint;
        self.code.func[self.code.ip]
    }

    fn fetch_next(&mut self) -> Instr {
        self.fetch(1)
    }
}
