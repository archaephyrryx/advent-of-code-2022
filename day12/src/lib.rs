extern crate pheap;
use std::collections::{HashMap};

#[derive(Eq, PartialEq, PartialOrd, Ord, Clone, Copy, Hash)]
pub enum Elevation {
    Start,
    Height(u8),
    End,
}

impl Elevation {
    pub fn is_reachable(&self, other: &Self) -> bool {

    }
}


impl Default for Elevation {
    fn default() -> Self {
        Self::Height(0)
    }
}

struct TopoGraph {
    pub edges: HashMap<usize, Vec<usize>>,
    pub startid: Option<usize>,
    pub destid: Option<usize>,
}


pub struct Matrix {
    storage: Vec<Elevation>,
    n_columns: usize,
}

impl Into<TopoGraph> for Matrix {
    fn into(self) -> TopoGraph {
        let mut edges = HashMap::new();
        let mut startid = None;
        let mut destid = None;
        self.storage.drain(..).enumerate().for_each(|(ix, el)| {
            match el {
                Elevation::Start => {
                    let None = startid.replace(ix) else { panic!("more than one start") };
                },
                Elevation::Height(h) => (),
                Elevation::End => {
                    let None = destid.replace(ix) else { panic!("more than one end") };
                },
            };
        });
        TopoGraph { edges, startid, destid }
    }
}




pub fn parse_matrix(lines: &mut impl Iterator<Item = String>) -> Matrix {

}