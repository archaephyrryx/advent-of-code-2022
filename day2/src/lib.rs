#[derive(Copy, Clone, Eq, PartialEq, PartialOrd)]
pub enum Outcome {
    Loss = 0,
    Draw = 3,
    Win = 6,
}

impl Outcome {
    pub fn from_str(s: &str) -> Outcome {
        match s {
            "X" => Self::Loss,
            "Y" => Self::Draw,
            "Z" => Self::Win,
            _ => unreachable!(),
        }
    }
}

#[derive(Copy, Clone, Eq, PartialEq, PartialOrd)]
#[repr(u8)]
pub enum Shape {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

impl Shape {
    pub fn from_str(s: &str) -> Shape {
        match s {
            "A" | "X" => Self::Rock,
            "B" | "Y" => Self::Paper,
            "C" | "Z" => Self::Scissors,
            _ => unreachable!(),
        }
    }

    pub fn from_u8(value: u8) -> Self {
        match value {
            1 => Self::Rock,
            2 => Self::Paper,
            3 => Self::Scissors,
            _ => panic!("Invalid value for from_u8: {}", value),
        }
    }

    pub fn cycle_add(self, increment: i8) -> Self {
        let Ok(min_increment): Result<u8, std::num::TryFromIntError> = increment.rem_euclid(3).try_into() else { unreachable!() } ;
        match self as u8 + min_increment {
            x @ (1..=3) => Shape::from_u8(x),
            y @ 4..=5 => Shape::from_u8(y - 3),
            _ => unreachable!(),
        }
    }

    pub fn generate(theirs: Shape, target: Outcome) -> Self {
        let off : i8 = match target {
            Outcome::Draw => 0,
            Outcome::Loss => -1,
            Outcome::Win => 1,
        };

        theirs.cycle_add(off)
    }

    pub fn outcome(self, other: Shape) -> Outcome {
        match self as i8 - other as i8 {
            1 | -2 => Outcome::Win,
            0 => Outcome::Draw,
            -1 | 2 => Outcome::Loss,
            _ => unreachable!(),
        }
    }

    pub fn score(self, other: Self) -> u32 {
        (self as u32) + (self.outcome(other) as u32)
    }
}