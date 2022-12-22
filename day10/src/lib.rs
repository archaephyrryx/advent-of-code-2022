use std::{iter::once};

#[derive(Clone, Copy, Hash, Debug, PartialEq, PartialOrd, Eq, Ord)]
pub struct Register {
    _x: isize,
}

impl Default for Register {
    fn default() -> Self {
        Self { _x: 1isize }
    }
}

#[derive(Clone, Copy, Hash, Debug, Default, PartialEq, PartialOrd, Eq, Ord)]
pub enum CycleOp {
    #[default]
    Sleep,
    Delta(isize),
}

impl CycleOp {
    pub fn apply(&self, memory: &mut Register) {
        match self {
            CycleOp::Sleep => (),
            CycleOp::Delta(delta) => memory._x += *delta,
        }
    }
}

#[derive(Clone, Copy, Hash, Debug, Default, PartialEq, PartialOrd, Eq, Ord)]
pub enum Instruction {
    #[default]
    Noop,
    AddX(isize),
}

impl Instruction {
    pub fn expand(&self) -> Box<dyn Iterator<Item = CycleOp>> {
        match self {
            Self::Noop => Box::new(once(CycleOp::Sleep)),
            Self::AddX(delta) => Box::new(once(CycleOp::Sleep).chain(once(CycleOp::Delta(*delta)))),
        }
    }
}

#[derive(Clone, Debug)]
pub struct CPU {
    program: Vec<Instruction>,
}

impl CPU {
    pub fn new(program: Vec<Instruction>) -> Self {
        Self {
            program,
        }
    }

    pub fn iter_signal<'a>(&'a self) -> impl 'a + Iterator<Item = (usize, isize)> {
        self.program.iter().flat_map(Instruction::expand).enumerate().scan(
            Register::default(),
            |mem, (cycles_elapsed, op)| {
                let cycle = cycles_elapsed + 1;
                let ret = Some((cycle, (<usize as TryInto<isize>>::try_into(cycle).unwrap()) * mem._x));
                op.apply(mem);
                ret
            }
        )
    }

    pub fn iter_pixel<'a>(&'a self) -> impl 'a + Iterator<Item = u8> {
        self.program.iter().flat_map(Instruction::expand).enumerate().scan(
            Register::default(),
            |mem, (hpos, op)| {
                let (h, t) = (mem._x - 1, mem._x + 1);
                let pix = (hpos % 40) as isize;
                let pixel = if pix >= h && pix <= t {
                    b'#'
                } else {
                    b'.'
                };
                op.apply(mem);
                Some(pixel)
            }
        )
    }
}