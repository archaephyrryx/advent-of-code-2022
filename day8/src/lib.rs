use std::{cell::RefCell, convert::Infallible, str::FromStr};

#[derive(Clone, Debug, Default)]
pub struct TreeStats {
    visible: bool,
    vantage: [u8; 4],
}

pub enum ScanDirection {
    Forward = 0,
    Backward = 1,
    Upward = 2,
    Downward = 3,
}

impl TreeStats {
    pub fn new() -> Self {
        Self {
            visible: false,
            vantage: [0; 4],
        }
    }

    pub fn set_vantage(&mut self, distance: u8, direction: ScanDirection) {
        self.vantage[direction as usize] = distance;
    }

    pub fn scenic_score(&self) -> u32 {
        self.vantage.iter().map(|&x| x as u32).product()
    }
}

#[derive(Clone, Debug, Default)]
pub struct Tree {
    height: u8,
    stats: RefCell<TreeStats>,
}

impl PartialOrd for Tree {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.height.partial_cmp(&other.height)
    }
}

impl Ord for Tree {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.height.cmp(&other.height)
    }
}

impl PartialEq for Tree {
    fn eq(&self, other: &Self) -> bool {
        self.height == other.height
    }
}

impl Eq for Tree {}

impl Tree {
    pub fn new(height: u8) -> Self {
        Self {
            height: height + 1,
            stats: RefCell::new(TreeStats::new()),
        }
    }

    pub fn count_flag(&self) -> usize {
        if self.stats.borrow().visible {
            1
        } else {
            0
        }
    }

    pub fn set_flag(&self) -> usize {
        let mut stats = self.stats.borrow_mut();
        let tmp = stats.visible;
        stats.visible = true;
        if tmp {
            1
        } else {
            0
        }
    }

    pub fn mark_distance<'a>(&self, dir: ScanDirection, acc: std::slice::Iter<'a, u8>) {
        let dist = acc
            .rev()
            .enumerate()
            .find(|(_, &t)| t >= self.height)
            .map_or_else(|| 0, |(ix, _)| ix + 1);
        self.stats.borrow_mut().set_vantage(dist as u8, dir);
    }

    pub fn apply_cover<'a>(&'a self, other: &'a Self) -> (usize, &'a Tree) {
        if self < other {
            (other.set_flag(), other)
        } else {
            (0, self)
        }
    }

    pub fn sweep_vis(storage: &[Tree]) -> usize {
        let (tmp, _) = storage
            .iter()
            .fold((0, &Self::default()), |(n, tallest), x| {
                let (delta, tallest) = tallest.apply_cover(x);
                (n + delta, tallest)
            });
        let (ret, _) = storage
            .iter()
            .rev()
            .fold((tmp, &Self::default()), |(n, tallest), x| {
                let (delta, tallest) = tallest.apply_cover(x);
                (n + delta, tallest)
            });
        ret
    }

    pub fn sweep(storage: &[Tree]) {
        let _: usize = storage
            .iter()
            .scan(vec![], |acc, x| {
                x.mark_distance(ScanDirection::Forward, acc.iter());
                acc.push(if acc.len() == 0 { u8::MAX } else { x.height });
                Some(1)
            })
            .sum();
        let _: usize = storage
            .iter()
            .rev()
            .scan(vec![], |acc, x| {
                x.mark_distance(ScanDirection::Backward, acc.iter());
                acc.push(if acc.len() == 0 { u8::MAX } else { x.height });
                Some(1)
            })
            .sum();
    }

    pub fn swing<'a>(storage: Vec<&'a Tree>) {
        let _: usize = storage
            .iter()
            .scan(vec![], |acc, x| {
                x.mark_distance(ScanDirection::Downward, acc.iter());
                acc.push(if acc.len() == 0 { u8::MAX } else { x.height });
                Some(1)
            })
            .sum();
        let _ : usize = storage
            .iter()
            .rev()
            .scan(vec![], |acc, x| {
                x.mark_distance(ScanDirection::Upward, acc.iter());
                acc.push(if acc.len() == 0 { u8::MAX } else { x.height });
                Some(1)
            }).sum();

    }
}

impl FromStr for Tree {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let h_char = s.bytes().next().unwrap();
        Ok(Self::new(h_char - b'0'))
    }
}

#[derive(Clone, Debug)]
pub struct Forest {
    _storage: Vec<Tree>,
    columns: usize,
}

impl Forest {
    pub fn from_rows(iter: impl Iterator<Item = String>) -> Self {
        let mut rows: Vec<Vec<Tree>> = iter
            .map(|s| {
                s.matches(char::is_numeric)
                    .map(|d| d.parse::<Tree>().unwrap())
                    .collect::<Vec<Tree>>()
            })
            .collect();
        let columns = rows[0].len();
        let _storage = rows.drain(..).flatten().collect();
        Self { _storage, columns }
    }

    pub fn max_scenic(&mut self) -> u32 {
        let ncols = self.columns;
        let nrows = self._storage.len() / ncols;

        (0..nrows).for_each(|i| Tree::sweep(&mut self._storage[i * ncols..(i + 1) * ncols]));
        (0..ncols)
            .for_each(|i| {
                Tree::swing(
                    self._storage
                        .iter()
                        .skip(i)
                        .step_by(ncols)
                        .collect::<Vec<_>>(),
                )
            });
        self._storage.iter().map(|x| x.stats.borrow().scenic_score()).max().unwrap()
    }
}
