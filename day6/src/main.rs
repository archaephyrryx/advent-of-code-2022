#![feature(array_windows)]

use std::{
    fs::File,
    io::{BufRead, BufReader}, collections::BTreeSet,
};

fn main() -> std::io::Result<()> {
    let f: File = File::open("src/input.txt")?;
    let mut signal = String::new();
    BufReader::new(f).read_line(&mut signal)?;
    let ret = signal
        .into_bytes()
        .array_windows::<14>()
        .enumerate()
        .find_map(|(ix, chars)| if distinct_all(chars) { Some(ix) } else { None })
        .unwrap();
    println!("Part Two: {}", ret + 14);
    Ok(())
}

fn distinct_all(chars: &[u8; 14]) -> bool {
    let tmp: BTreeSet<u8> = chars.iter().copied().collect();
    tmp.len() == 14
}

// fn distinct(a: u8, b: u8, c: u8, d: u8) -> bool {
//     let x = a as i64;
//     let y = b as i64;
//     let z = c as i64;
//     let t = d as i64;

//     (x - y) * (y - z) * (z - t) * (t - x) * (x - z) * (y - t) != 0
// }
