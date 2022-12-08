use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn main() -> std::io::Result<()> {
    let f: File = std::fs::File::open("src/input.txt")?;
    let outcome = process_lines(BufReader::new(f).lines())?;
    println!("Part One: {}", outcome);
    Ok(())
}

fn process_lines(mut lines: std::io::Lines<BufReader<File>>) -> std::io::Result<String> {
    let mut stacks = day5::Stacks::new();

    lines
        .by_ref()
        .take_while(|line| {
            if let Ok(s) = line {
                &s[0..=1] != " 1"
            } else {
                false
            }
        })
        .for_each(|line| {
            let tmp = line
                .unwrap()
                .chars()
                .enumerate()
                .filter_map(|(ix, c)| if ix % 4 == 1 { Some(c) } else { None })
                .collect::<String>();
            stacks.push_stratum(tmp);
        });

    for instruction in lines
        .skip_while(|l| if let Ok(s) = l { !s.is_empty() } else { false })
        .skip(1)
    {
        let tmp = instruction.unwrap();
        let args = tmp
            .split(' ')
            .into_iter()
            .enumerate()
            .filter_map(|(ix, frag)| {
                if ix % 2 == 0 {
                    None
                } else {
                    frag.parse::<usize>().ok()
                }
            })
            .collect::<Vec<usize>>();
        stacks.swap(args[0], args[1] - 1, args[2] - 1);
    }

    Ok(stacks.skim())
}
