fn parse_range(x: &str) -> Option<(u32,u32)> {
    let (min, max) = x.split_once('-')?;
    let minval : u32 = min.parse().ok()?;
    let maxval : u32 = max.parse().ok()?;

    Some((minval,maxval))
}

pub fn is_total_overlap(x: &str, y: &str) -> bool {
    use std::cmp::Ordering::Equal;

    let (m, n) = parse_range(x).unwrap();
    let (p, q) = parse_range(y).unwrap();

    match (m.cmp(&p), n.cmp(&q)) {
        (Equal, Equal) => true,
        (i, j) => i != j,
    }
}

pub fn is_partial_overlap(x: &str, y: &str) -> bool {
    let (m, n) = parse_range(x).unwrap();
    let (p, q) = parse_range(y).unwrap();

    !(q < m || p > n)
}