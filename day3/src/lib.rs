use std::cmp::Ordering;

pub fn priority(c: u8) -> u32 {
    match c {
        b'a'..=b'z' => ((c - b'a') as u32) + 1,
        b'A'..=b'Z' => ((c - b'A') as u32) + 27,
        _ => unreachable!(),
    }
}

pub fn scanlr<'a>(l: &'a mut str, r: &'a mut str) -> u32 {
    let sorted_l = unsafe { let tmp = l.as_bytes_mut(); tmp.sort(); tmp };
    let sorted_r = unsafe { let tmp = r.as_bytes_mut(); tmp.sort(); tmp };

    let mut i : usize = 0;
    let mut j : usize = 0;

    while usize::max(i, j) < sorted_l.len() {
        let lbyte = sorted_l[i];
        let rbyte = sorted_r[j];
        match lbyte.cmp(&rbyte) {
            Ordering::Less => i += 1,
            Ordering::Equal => return priority(lbyte),
            Ordering::Greater => j += 1,
        }
    }

    unreachable!();
}

pub fn scanlmr<'a>(l: &'a mut str, m: &'a mut str, r: &'a mut str) -> u32 {
    let sorted_l = unsafe { let tmp = l.as_bytes_mut(); tmp.sort(); tmp };
    let sorted_m = unsafe { let tmp = m.as_bytes_mut(); tmp.sort(); tmp };
    let sorted_r = unsafe { let tmp = r.as_bytes_mut(); tmp.sort(); tmp };

    let mut i : usize = 0;
    let mut j : usize = 0;
    let mut k : usize = 0;

    while i < sorted_l.len() && j < sorted_m.len() && k < sorted_r.len() {
        let lbyte = sorted_l[i];
        let mbyte = sorted_m[j];
        let rbyte = sorted_r[k];

        let highest = lbyte.max(mbyte.max(rbyte));

        match (lbyte == highest, mbyte == highest, rbyte == highest) {
            (true, true, true) => return priority(lbyte),
            (enough_i, enough_j, enough_k) => {
                if !enough_i {
                    i += 1;
                }
                if !enough_j {
                    j += 1;
                }
                if !enough_k {
                    k += 1;
                }
            },
        }
    }

    unreachable!();
}