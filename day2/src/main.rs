extern crate std;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::io::Lines;

use day2::{Outcome, Shape};

#[derive(Debug, Clone)]
struct UnexpectedLineError;

impl std::fmt::Display for UnexpectedLineError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Unexpected line found in input")
    }
}

impl std::error::Error for UnexpectedLineError {}

fn process_lines(lines: Lines<BufReader<File>>) -> std::io::Result<(u32, u32)> {
    let mut total_part1 = 0u32;
    let mut total_part2 = 0u32;

    for line in lines {
        if let Some((theirs, mine)) = line?.split_once(' ') {
            let their_shape: Shape = Shape::from_str(theirs);
            let my_shape_part1 = Shape::from_str(mine);
            let my_shape_part2 = Shape::generate(their_shape, Outcome::from_str(mine));
            total_part1 += my_shape_part1.score(their_shape);
            total_part2 += my_shape_part2.score(their_shape);
        } else {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                UnexpectedLineError,
            ));
        }
    }

    Ok((total_part1, total_part2))
}

fn main() -> std::io::Result<()> {
    let file = File::open("/home/peter/projects/advent-of-code-2022/day2/src/input.txt")?;
    let reader = BufReader::new(file);

    let Ok((part1, part2)) = process_lines(reader.lines()) else { unreachable!(); };

    println!("Part One: {}", part1);
    println!("Part Two: {}", part2);
    Ok(())
}
