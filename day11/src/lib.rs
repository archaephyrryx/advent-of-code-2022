extern crate combine;
use combine::{Parser};

mod calc {
    #[derive(Copy, Clone, PartialEq, Eq, Hash)]
    pub enum Expr {
        Add(u32),
        Multiply(u32),
        Square,
    }

    impl std::str::FromStr for Expr {
        type Err = <u32 as std::str::FromStr>::Err;

        fn from_str(s: &str) -> Result<Self, Self::Err> {

        }
    }

    impl Expr {
        pub fn compile(self) -> Box<dyn Fn(u32) -> u32> {
            match self {
                Expr::Add(i) => Box::new(move |x| x + i),
                Expr::Multiply(i) => Box::new(move |x| x * i),
                Expr::Square => Box::new(move |x| x * x),
            }
        }
    }
}

pub struct MonkeyBrain {
    id: usize,
    behavior: Box<dyn Fn(u32) -> u32>,
    quotient: u32,
    branch_t: usize,
    branch_f: usize,
}

impl MonkeyBrain {
    pub fn new(id: usize, raw_expr: calc::RawExpr, quotient: u32, branch_t: usize, branch_f: usize) -> Self {
        let behavior =

    }
    pub fn inspect(&self, item: u32) -> (usize, u32) {
        let inspected : u32 = (self.behavior)(item);
        let relaxed : u32 = inspected / 3;
        if relaxed % self.quotient == 0 {
            (self.branch_t, relaxed)
        } else {
            (self.branch_f, relaxed)
        }
    }
}

pub fn parse_monkey(lines: &mut impl Iterator<Item = std::io::Result<String>>) -> Option<(Vec<u32>, MonkeyBrain)> {
    let mut held_items : Vec<u32> = Vec::new();
    let _header =
        lines
            .next()?
            .expect("monkey error")
            ;
    let mb =
    Some()
}