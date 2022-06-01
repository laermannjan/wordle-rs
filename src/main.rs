const GAMES: &str = include_str!("../data/answers.txt");
// const DICTIONARY: &str = include!("../data/dictionary.txt");

fn main() {
    let w = wordle::Wordle::new();
    for answer in GAMES.split_whitespace() {
        let guesser = wordle::algorithms::Unoptimized::new();
        w.play(answer, guesser);
    }
}
