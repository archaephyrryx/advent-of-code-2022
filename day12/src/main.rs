use std::{io::{BufReader, BufRead}, fs::File};

fn main() -> std::io::Result<()> {
    let f = File::open("src/input.txt")?;
    let outcome = process_lines(BufReader::new(f).lines())?;
    outcome.iter().for_each(|row| {
        println!("{}", row);
    });
    Ok(())
}

fn process_lines(lines: std::io::Lines<BufReader<File>>) -> std::io::Result<usize> {
    let matrix = parse_matrix(lines);
}