use std::{fs::File, io::{BufReader, BufRead}};

fn main() -> std::io::Result<()> {
    let f: File = File::open("src/input.txt")?;
    let outcome = process_lines(BufReader::new(f).lines())?;
    Ok(())
}

fn process_lines(lines: std::io::Lines<BufReader<File>>) -> std::io::Result<Vec<String>> {

}

