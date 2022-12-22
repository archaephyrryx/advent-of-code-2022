use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn main() -> std::io::Result<()> {
    let f: File = File::open("src/input.txt~")?;
    let outcome = process_lines(BufReader::new(f).lines())?;
    println!("{}", outcome);
    Ok(())
}

fn process_lines(mut lines: std::io::Lines<BufReader<File>>) -> std::io::Result<usize> {
    let Ok((mut parcels, monkeys))
        = day11::parse_monkeys(&mut lines) else { panic!("process_lines") };
    let mut levels: Vec<usize> = monkeys.iter().map(|_| 0usize).collect();
    let modulus : u64 = monkeys.iter().map(day11::MonkeyBrain::get_modulus).product();
    (0..10_000).for_each(|_| {
        monkeys.iter().for_each(|monkey| {
            let id = monkey.get_id();
            if let Some(mut mine) = parcels.remove(&id) {
                mine.drain(..).for_each(|item| {
                    let (ix, value) = monkey.inspect_mod(item, modulus);
                    levels[id] += 1;
                    parcels.entry(ix).or_default().push(value);
                })
            }
        })
    });
    levels.sort_by(|a, b| b.cmp(a));
    Ok(levels[0] * levels[1])
}
