/// Compute the Scrabble score for a word.
pub fn score(word: &str) -> u64 {
    use once_cell::sync::Lazy;
    use std::collections::HashMap;

    static SCORES: Lazy<HashMap<char, u64>> = Lazy::new(|| {
        use std::iter::{empty, repeat};

        let scores = empty()
            .chain(
                ['A', 'E', 'I', 'O', 'U', 'L', 'N', 'R', 'S', 'T']
                    .into_iter()
                    .zip(repeat(1)),
            )
            .chain(['D', 'G'].into_iter().zip(repeat(2)))
            .chain(['B', 'C', 'M', 'P'].into_iter().zip(repeat(3)))
            .chain(['F', 'H', 'V', 'W', 'Y'].into_iter().zip(repeat(4)))
            .chain(['K'].into_iter().zip(repeat(5)))
            .chain(['J', 'X'].into_iter().zip(repeat(8)))
            .chain(['Q', 'Z'].into_iter().zip(repeat(10)))
            .collect::<HashMap<_, _>>();
        assert_eq!(26, scores.len());
        scores
    });

    word.chars()
        .filter(|ch| ch.is_ascii_alphabetic())
        .map(|ch| SCORES.get(&ch.to_ascii_uppercase()).unwrap())
        .sum()
}
