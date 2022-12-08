use std::collections::VecDeque;

pub struct Stacks {
    _stacks: Vec<VecDeque<char>>,
}

impl Stacks {
    pub fn new() -> Self {
        Stacks { _stacks: Vec::new() }
    }

    pub fn push_stratum(&mut self, stratum: String) {
        let n_stacks = self._stacks.len();
        for _ in n_stacks..stratum.len() {
            self._stacks.push(VecDeque::new());
        }

        self._stacks.iter_mut().zip(stratum.chars()).for_each(
            |(stack, c)| {
            if c != ' ' {
                stack.push_back(c);
            } else {
                debug_assert_eq!(stack.len(), 0);
            }
        } );
    }

    pub fn swap(&mut self, amount: usize, source_ix: usize, target_ix: usize) {
        assert_ne!(source_ix, target_ix);
        let src = &mut self._stacks[source_ix];
        let tmp: Vec<char> = src.drain(0..amount).rev().collect();
        let tgt = &mut self._stacks[target_ix];
        tmp.into_iter().for_each(|x| tgt.push_front(x));
    }

    pub fn skim(&self) -> String {
        self._stacks.iter().map(|stack| stack.front()).flatten().collect::<String>()
    }
}
