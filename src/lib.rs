use std::collections::HashSet;

pub mod algorithms;

const DICTIONARY: &str = include_str!("../data/dictionary.txt");

pub struct Wordle {
    dictionary: HashSet<&'static str>,
}

impl Wordle {
    pub fn new() -> Self {
        Self {
            dictionary: HashSet::from_iter(DICTIONARY.lines().map(|line| {
                line.split_once(' ')
                    .expect("every line is word + space + frequency")
                    .0
            })),
        }
    }

    pub fn play<G: Guesser>(&self, answer: &'static str, mut guesser: G) -> Option<usize> {
        let mut history = Vec::new();
        for i in 1..=32 {
            let guess = guesser.guess(&history);
            if guess == answer {
                return Some(i);
            }
            assert!(self.dictionary.contains(&*guess));
            let correctness = Correctness::compute(answer, &guess);
            history.push(Guess {
                word: guess,
                mask: correctness,
            });
        }
        None
    }
}
pub trait Guesser {
    fn guess(&mut self, history: &[Guess]) -> String;
}

pub struct Guess {
    word: String,
    mask: [Correctness; 5],
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Correctness {
    Correct,
    Misplaced,
    Wrong,
}

impl Correctness {
    fn compute(answer: &str, guess: &str) -> [Self; 5] {
        assert_eq!(answer.len(), 5);
        assert_eq!(guess.len(), 5);

        let mut c = [Correctness::Wrong; 5];
        let mut used = [false; 5];

        // mark all correct guesses first
        for (i, (a, g)) in answer.chars().zip(guess.chars()).enumerate() {
            if a == g {
                c[i] = Correctness::Correct;
                used[i] = true;
            }
        }

        // mark all misplaced guesses
        for (i, g) in guess.chars().enumerate() {
            if c[i] == Correctness::Correct {
                continue;
            }

            if answer.chars().enumerate().any(|(j, a)| {
                if a == g && !used[j] {
                    used[j] = true;
                    return true;
                }
                false
            }) {
                c[i] = Correctness::Misplaced;
            }
        }
        c
    }
}

impl Guesser for fn(history: &[Guess]) -> String {
    fn guess(&mut self, history: &[Guess]) -> String {
        (*self)(history)
    }
}

#[cfg(test)]
macro_rules! guesser {
    (|$history:ident| $impl:block) => {{
        struct G;
        impl $crate::Guesser for G {
            fn guess(&mut self, $history: &[Guess]) -> String {
                $impl
            }
        }
        G
    }};
}

#[cfg(test)]
mod tests {
    mod game {
        use crate::{Guess, Wordle};

        #[test]
        fn genius() {
            let w = Wordle::new();
            let guesser = guesser!(|_history| {
                if _history.len() == 0 {
                    return "right".to_string();
                }
                return "wrong".to_string();
            });
            assert_eq!(w.play("right", guesser), Some(1));
        }
        #[test]
        fn magnificent() {
            let w = Wordle::new();
            let guesser = guesser!(|_history| {
                if _history.len() == 1 {
                    return "right".to_string();
                }
                return "wrong".to_string();
            });
            assert_eq!(w.play("right", guesser), Some(2));
        }
        #[test]
        fn impressive() {
            let w = Wordle::new();
            let guesser = guesser!(|_history| {
                if _history.len() == 2 {
                    return "right".to_string();
                }
                return "wrong".to_string();
            });
            assert_eq!(w.play("right", guesser), Some(3));
        }
        #[test]
        fn splendid() {
            let w = Wordle::new();
            let guesser = guesser!(|_history| {
                if _history.len() == 3 {
                    return "right".to_string();
                }
                return "wrong".to_string();
            });
            assert_eq!(w.play("right", guesser), Some(4));
        }
        #[test]
        fn great() {
            let w = Wordle::new();
            let guesser = guesser!(|_history| {
                if _history.len() == 4 {
                    return "right".to_string();
                }
                return "wrong".to_string();
            });
            assert_eq!(w.play("right", guesser), Some(5));
        }
        #[test]
        fn phew() {
            let w = Wordle::new();
            let guesser = guesser!(|_history| {
                if _history.len() == 5 {
                    return "right".to_string();
                }
                return "wrong".to_string();
            });
            assert_eq!(w.play("right", guesser), Some(6));
        }

        #[test]
        fn oops() {
            let w = Wordle::new();
            let guesser = guesser!(|_history| { "wrong".to_string() });
            assert_eq!(w.play("right", guesser), None);
        }
    }
    mod compute {
        use crate::Correctness;

        macro_rules! mask {
            (C) => {Correctness::Correct};
            (M) => {Correctness::Misplaced};
            (W) => {Correctness::Wrong};
            ($($c:tt)+) => {[$(mask!($c)),+]}
        }

        #[test]
        fn all_green() {
            assert_eq!(Correctness::compute("abcde", "abcde"), mask![C C C C C]);
        }

        #[test]
        fn all_yellow() {
            assert_eq!(Correctness::compute("abcde", "bcdea"), mask![M M M M M]);
        }

        #[test]
        fn all_grey() {
            assert_eq!(Correctness::compute("abcde", "zzzzz"), mask![W W W W W]);
        }

        #[test]
        fn repeat_green() {
            assert_eq!(Correctness::compute("aabbb", "aaccc"), mask![C C W W W]);
        }
        #[test]
        fn repeat_yellow() {
            assert_eq!(Correctness::compute("aabbb", "ccaac"), mask![W W M M W]);
        }

        #[test]
        fn same_green_yellow() {
            assert_eq!(Correctness::compute("aabbb", "caacc"), mask![W C M W W]);
        }

        #[test]
        fn same_green_yellow_but_one_too_many() {
            assert_eq!(Correctness::compute("aabbb", "caaac"), mask![W C M W W]);
        }

        #[test]
        fn only_one_green() {
            assert_eq!(Correctness::compute("abcde", "aacde"), mask![C W C C C]);
        }
    }
}
