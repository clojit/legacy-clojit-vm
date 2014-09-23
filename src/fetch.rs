use vm::Vm;
use vm::Instr;

pub trait Fetch {
    fn fetch(&mut self, offset: uint) -> Instr;
    fn fetch_next(&mut self) -> Instr;
}

impl Fetch for Vm {
    fn fetch(&mut self, offset: uint) -> Instr {
        let ctx = &mut self.ctx;
        let func = &self.data.cfunc[ctx.func];

        ctx.ip += offset;

        func[ctx.ip]
    }

    fn fetch_next(&mut self) -> Instr {
        self.fetch(1)
    }
}
