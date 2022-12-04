extern crate std;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::io::BufReader;
use std::io::Lines;

use day1::TernaryHeap;

type GenericError = std::boxed::Box<dyn std::error::Error + Send + Sync>;

fn accumulate_lines(lines: Lines<BufReader<File>>) -> Result<u32, GenericError> {
    let mut elf_heap = TernaryHeap::new();
    let mut current_elf: u32 = 0;

    for line in lines {
        match line? {
            str if str.is_empty() => {
                elf_heap.push(current_elf);
                current_elf = 0;
                continue;
            }
            str => {
                let calories: u32 = str.parse::<u32>()?;
                current_elf += calories;
            }
        }
    }

    elf_heap.push(current_elf);

    let total: u32 = elf_heap.into_sum();
    Ok(total)
}

fn main() -> io::Result<()> {
    let file = File::open("/home/peter/projects/advent-of-code-2022/day1/src/input.txt")?;
    let reader = BufReader::new(file);

    let result = accumulate_lines(reader.lines());
    println!("{}", result.unwrap());
    Ok(())
}
