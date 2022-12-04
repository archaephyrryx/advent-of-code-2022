extern crate std;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::io::Lines;

// fn bisect<'a>(s: &'a mut str) -> (&'a mut str, &'a mut str) {
//     let mid = s.len() / 2;
//     s.split_at_mut(mid)
// }

// fn part_one(lines: Lines<BufReader<File>>) -> std::io::Result<u32> {
//     let mut total = 0u32;

//     for line in lines {
//         let Ok(mut s) = line else { return line.map(|_| unreachable!() ); };
//         let (left, right) = bisect(s.as_mut_str());
//         total += day3::scanlr(left, right);
//     }

//     Ok(total)
// }

fn part_two(lines: Lines<BufReader<File>>) -> std::io::Result<u32> {
    let mut total: u32 = 0u32;

    let mut all_lines: Vec<String> = lines.flatten().collect();
    let groups = all_lines.chunks_exact_mut(3);

    for group in groups {
        let [ref mut a, ref mut b, ref mut c] = group else { unreachable!(); };
        total += day3::scanlmr(a.as_mut_str(), b.as_mut_str(), c.as_mut_str());
    }

    Ok(total)
}

fn main() -> std::io::Result<()> {
    let file = File::open("/home/peter/projects/advent-of-code-2022/day3/src/input.txt")?;
    let reader = BufReader::new(file);

    let Ok(part2) = part_two(reader.lines()) else { unreachable!(); };

    // println!("Part One: {}", part1);
    println!("Part Two: {}", part2);
    Ok(())
}
