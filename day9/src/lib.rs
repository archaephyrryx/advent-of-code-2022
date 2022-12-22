use arrayvec::ArrayVec;
use std::{collections::HashSet};

#[derive(Debug, Hash, Default, PartialEq, PartialOrd, Eq, Ord, Clone, Copy)]
pub struct Point(pub isize, pub isize);

impl Point {
    pub fn new() -> Self {
        Self(0, 0)
    }

    pub fn follow(&self, locus: Self) -> Option<Self> {
        let delta = locus - *self;
        if delta.is_unit() {
            None
        } else {
            let ret = *self + delta.normalize();
            Some(ret)
        }
    }
}

#[derive(Debug, Hash, Default, PartialEq, PartialOrd, Eq, Ord, Clone, Copy)]
pub struct Offset(pub isize, pub isize);

impl Offset {
    fn is_unit(&self) -> bool {
        self.0.abs() <= 1 && self.1.abs() <= 1
    }

    fn normalize(&self) -> Self {
        Self(self.0.signum(), self.1.signum())
    }
}

impl std::ops::Sub<Point> for Point {
    type Output = Offset;

    fn sub(self, rhs: Point) -> Self::Output {
        Offset(self.0 - rhs.0, self.1 - rhs.1)
    }
}

impl std::ops::Add<Offset> for Point {
    type Output = Point;

    fn add(self, rhs: Offset) -> Self::Output {
        Self(self.0 + rhs.0, self.1 + rhs.1)
    }
}

#[derive(Debug, Clone)]
pub struct Chain<const N: usize>(ArrayVec<Point, N>);

impl<const N: usize> Chain<N> {
    pub fn new() -> Self {
        Self(ArrayVec::from_iter([Point::default(); N].into_iter()))
    }

    pub fn step(self, dir: &str, dist: usize, canvas: &mut Canvas) -> Self {
        if dist == 0 {
            return self;
        }

        let offs = match dir {
            "U" => Offset(0, 1),
            "D" => Offset(0, -1),
            "L" => Offset(-1, 0),
            "R" => Offset(1, 0),
            _ => unreachable!(),
        };

        let mut ret = self;

        for _ in 0..dist {
            ret = ret.shift(offs, canvas);
        }

        ret
    }

    fn shift(self, offs: Offset, canvas: &mut Canvas) -> Chain<N> {
        let init = self.0[0] + offs;
        Self(
            self.0
                .into_iter()
                .enumerate()
                .scan(Some(init), |next, (ix, link)| match next {
                    Some(locus) => {
                        let newlink = if ix == 0 {
                            Some(*locus)
                        } else {
                            link.follow(*locus)
                        };
                        match newlink {
                            Some(x) => {
                                if ix == N - 1 {
                                    canvas.register(x);
                                }
                                next.replace(x)
                            }
                            None => next.take(),
                        };
                        newlink.or(Some(link))
                    }
                    None => Some(link),
                })
                .collect(),
        )
    }
}

pub struct Canvas {
    _visited: HashSet<Point>,
    distinct: usize,
}

impl Canvas {
    pub fn new() -> Self {
        let mut _visited = HashSet::new();
        _visited.insert(Point::default());
        Self {
            _visited,
            distinct: 1,
        }
    }

    pub fn get_distinct(&self) -> usize {
        self.distinct
    }

    pub fn register(&mut self, x: Point) {
        if self._visited.insert(x) {
            self.distinct += 1;
        }
    }
}
