use std::{fs::File, io::{BufReader, BufRead}};

use day10::{Instruction, CPU};

fn main() -> std::io::Result<()> {
    let f: File = File::open("src/input.txt")?;
    let outcome = process_lines(BufReader::new(f).lines())?;
    outcome.iter().for_each(|row| {
        println!("{}", row);
    });
    Ok(())
}

fn process_lines(lines: std::io::Lines<BufReader<File>>) -> std::io::Result<Vec<String>> {
    let program: Vec<Instruction> = lines.flatten().map(|s| {
        let (cmd, rest) = s.split_at(4);
        match cmd {
            "noop" => day10::Instruction::Noop,
            "addx" => day10::Instruction::AddX(rest.trim().parse::<isize>().unwrap()),
            _ => unreachable!(),
        }
    }).collect();
    let cpu = CPU::new(program);
    Ok(cpu.iter_pixel().collect::<Vec<u8>>().chunks(40).map(|pixels| String::from_utf8_lossy(pixels).into_owned()).collect::<Vec<String>>())
}
