use std::{fs::File, io::{BufReader, BufRead}, fmt::Write};

fn main() -> std::io::Result<()> {
    let f: File = File::open("src/input.txt")?;
    let outcome = process_lines(BufReader::new(f).lines())?;
    println!("{}", outcome);
    Ok(())
}

fn process_lines(lines: std::io::Lines<BufReader<File>>) -> std::io::Result<usize> {
    let mut hier = day7::Hierarchy::new();
    let mut path = String::new();

    for line in lines.flatten() {
        if let Some(cdarg) = line.strip_prefix("$ cd ") {
            match cdarg {
                ".." => {
                    let ix = path.rfind("/").unwrap();
                    path.drain(ix..);
                    ()
                },
                "/" => continue,
                other => { let _ = write!(&mut path, "/{}", other); () }
            };
        } else if let Some(_) = line.strip_prefix("$ ls") {
            continue;
        } else {
            match line.split_once(' ').unwrap() {
                ("dir", _) => continue,
                (rawsize, name) => {
                    hier.add_file(format!("{}/{}", path, name), rawsize.parse().unwrap());
                }
            }
        }
    }

    const MAX_USED : usize = 70000000 - 30000000;
    let amount_used : usize = hier.space_used();
    let must_free = amount_used - MAX_USED;

    Ok(hier.into_iter().filter_map(|(_, v)| {
        match v {
            day7::Node::Directory(n) if n >= must_free => Some(n),
            _ => None,
        }
    }).min().unwrap())
}