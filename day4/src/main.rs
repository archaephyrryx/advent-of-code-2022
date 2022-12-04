extern crate std;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::io::Lines;

fn bisect<'a>(s: &'a str) -> (&'a str, &'a str) {
    s.split_once(',').unwrap()
}

fn process_lines(lines: Lines<BufReader<File>>) -> std::io::Result<(usize, usize)> {
    let mut total_one = 0usize;
    let mut total_two = 0usize;

    for line in lines {
        let Ok(s) = line else { return line.map(|_| unreachable!() ); };
        let (left, right) = bisect(s.as_str());
        if day4::is_partial_overlap(left, right) {
            if day4::is_total_overlap(left, right) {
                total_one += 1;
            }
            total_two += 1;
        }
    }

    Ok((total_one, total_two))
}

fn main() -> std::io::Result<()> {
    let file = File::open("/home/peter/projects/advent-of-code-2022/day4/src/input.txt")?;
    let reader = BufReader::new(file);

    let Ok((part1, part2)) = process_lines(reader.lines()) else { unreachable!(); };

    println!("Part One: {}", part1);
    println!("Part Two: {}", part2);
    Ok(())
}
