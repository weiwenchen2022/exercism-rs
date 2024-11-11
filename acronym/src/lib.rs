pub fn abbreviate(phrase: &str) -> String {
    // todo!("Given the phrase '{phrase}', return its acronym");

    use std::iter;

    phrase
        .split(|ch: char| ch.is_whitespace() || '-' == ch)
        .flat_map(|word| -> Box<dyn Iterator<Item = char>> {
            let word = word.trim_matches(|ch: char| ch.is_ascii_punctuation());
            if word.is_empty() {
                Box::new(iter::empty())
            } else if word.chars().all(|ch| ch.is_uppercase()) {
                Box::new(iter::once(word.chars().next().unwrap()))
            } else {
                Box::new(word.chars().enumerate().flat_map(
                    |(i, ch)| -> Box<dyn Iterator<Item = char>> {
                        if i == 0 || ch.is_uppercase() {
                            Box::new(ch.to_uppercase())
                        } else {
                            Box::new(iter::empty())
                        }
                    },
                ))
            }
        })
        .collect()
}
