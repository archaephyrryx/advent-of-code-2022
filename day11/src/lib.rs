use std::collections::HashMap;

mod calc {
    #[derive(Copy, Clone, PartialEq, Eq, Hash)]
    pub enum Expr {
        Add(u64),
        Multiply(u64),
        Square,
    }

    impl std::str::FromStr for Expr {
        type Err = <u64 as std::str::FromStr>::Err;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            match s.split_once(' ').unwrap() {
                ("*", "old") => Ok(Self::Square),
                ("*", scalar) => Ok(Self::Multiply(scalar.parse::<u64>()?)),
                ("+", constant) => Ok(Self::Add(constant.parse::<u64>()?)),
                _ => unreachable!(),
            }
        }
    }

    impl Expr {
        pub fn compile(self) -> Box<dyn Fn(u64) -> u64> {
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
    behavior: Box<dyn Fn(u64) -> u64>,
    quotient: u64,
    branch_t: usize,
    branch_f: usize,
}

impl std::fmt::Debug for MonkeyBrain {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MonkeyBrain")
            .field("id", &self.id)
            .field("behavior", &"<closure>")
            .field("quotient", &self.quotient)
            .field("branch_t", &self.branch_t)
            .field("branch_f", &self.branch_f)
            .finish()
    }
}

impl MonkeyBrain {
    pub fn new(
        id: usize,
        expr: calc::Expr,
        quotient: u64,
        branch_t: usize,
        branch_f: usize,
    ) -> Self {
        let behavior = expr.compile();
        Self {
            id,
            behavior,
            quotient,
            branch_t,
            branch_f,
        }
    }

    pub fn get_id(&self) -> usize {
        self.id
    }

    pub fn get_modulus(&self) -> u64 {
        self.quotient
    }

    pub fn get_trio(&self) -> [usize; 3] {
        [self.id, self.branch_t, self.branch_f]
    }

    pub fn inspect(&self, item: u64) -> (usize, u64) {
        let inspected = (self.behavior)(item);
        let relaxed = inspected / 3;
        if relaxed % self.quotient == 0 {
            (self.branch_t, relaxed)
        } else {
            (self.branch_f, relaxed)
        }
    }

    pub fn inspect_mod(&self, item: u64, modulus: u64) -> (usize, u64) {
        let inspected = (self.behavior)(item);
        let rescaled = inspected % modulus;
        if rescaled % self.quotient == 0 {
            (self.branch_t, rescaled)
        } else {
            (self.branch_f, rescaled)
        }
    }

}

pub fn parse_monkey(
    lines: &mut impl Iterator<Item = std::io::Result<String>>,
) -> Option<(Vec<u64>, MonkeyBrain)> {
    let mut held_items: Vec<u64> = Vec::new();
    let id = lines
        .next()?
        .expect("monkey read error")
        .strip_prefix("Monkey ")?
        .strip_suffix(":")?
        .parse::<usize>()
        .expect("monkey parse error");
    lines
        .next()?
        .expect("starting items error")
        .trim_start()
        .strip_prefix("Starting items: ")?
        .split(", ")
        .for_each(|item| held_items.push(item.parse::<u64>().unwrap()));
    let expr = lines
        .next()?
        .expect("operation read error")
        .trim_start()
        .strip_prefix("Operation: new = old ")?
        .parse::<calc::Expr>()
        .expect("operation parse error");
    let quotient = lines
        .next()?
        .expect("quotient read error")
        .trim_start()
        .strip_prefix("Test: divisible by ")?
        .parse::<u64>()
        .expect("quotient parse error");
    let branch_t = lines
        .next()?
        .expect("branch_t read error")
        .trim_start()
        .strip_prefix("If true: throw to monkey ")?
        .parse::<usize>()
        .expect("branch_t parse error");
    let branch_f = lines
        .next()?
        .expect("branch_f read error")
        .trim_start()
        .strip_prefix("If false: throw to monkey ")?
        .parse::<usize>()
        .expect("branch_f parse error");
    let mb = MonkeyBrain::new(id, expr, quotient, branch_t, branch_f);
    Some((held_items, mb))
}

pub fn parse_monkeys(
    lines: &mut impl Iterator<Item = std::io::Result<String>>,
) -> std::io::Result<(HashMap<usize, Vec<u64>>, Vec<MonkeyBrain>)> {
    let mut parcels = HashMap::new();
    let mut monkeys = Vec::new();

    loop {
        let Some((parcel, monkey)) = parse_monkey(lines) else { panic!("could not parse monkey")};
        parcels.insert(monkey.get_id(), parcel);
        monkeys.push(monkey);
        if let None = lines.next() {
            break;
        }
    }

    Ok((parcels, monkeys))
}
