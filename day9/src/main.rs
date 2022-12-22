use std::{fs::File, io::{BufReader, BufRead}};

fn main() -> std::io::Result<()> {
    let f: File = File::open("src/input.txt")?;
    let outcome = process_lines(BufReader::new(f).lines())?;
    println!("{}", outcome);
    Ok(())
}

fn process_lines(lines: std::io::Lines<BufReader<File>>) -> std::io::Result<usize> {
    let mut canvas = day9::Canvas::new();
    lines.flatten().fold(day9::Chain::<10>::new(), |chain, s| {
        let Some((dir, dist)) = s.split_once(' ') else { panic!() };
        chain.step(dir, dist.parse::<usize>().unwrap(), &mut canvas)
    });
    Ok(canvas.get_distinct())
}