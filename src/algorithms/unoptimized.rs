use crate::{Guess, Guesser};

pub struct Unoptimized;

impl Unoptimized {
    pub fn new() -> Self {
        Unoptimized
    }
}

impl Guesser for Unoptimized {
    fn guess(&mut self, _history: &[Guess]) -> String {
        todo!();
    }
}
