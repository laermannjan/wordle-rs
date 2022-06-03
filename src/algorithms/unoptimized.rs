use std::collections::HashMap;

use crate::{Guess, Guesser, DICTIONARY};

pub struct Unoptimized {
    remaining: HashMap<&'static str, usize>,
}

impl Unoptimized {
    pub fn new() -> Self {
        Unoptimized {
            remaining: HashMap::from_iter(DICTIONARY.lines().map(|line| {
                let (word, count) = line.split_once(' ')
                    .expect("every line is word + space + frequency");
                let count: usize = count.parse().expect("every count is a number");
                (word, count)
            })), 
        }
    }
}

impl Guesser for Unoptimized {
    fn guess(&mut self, _history: &[Guess]) -> String {
        todo!();
    }
}
