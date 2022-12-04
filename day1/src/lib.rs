use core::cmp::Reverse;
use std::collections::BinaryHeap;

pub struct TernaryHeap {
    raw: BinaryHeap<Reverse<u32>>,
}

impl TernaryHeap {
    pub fn new() -> TernaryHeap {
        TernaryHeap { raw: BinaryHeap::with_capacity(4) }
    }

    pub fn push(&mut self, value: u32) {
        self.raw.push(Reverse(value));
        if self.raw.len() > 3 {
            let _ =self.raw.pop();
        }
    }

    pub fn into_sum(self) -> u32 {
        return self.raw.into_iter().map(|Reverse(x)| x).sum();
    }
}