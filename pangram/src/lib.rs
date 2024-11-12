/// Determine whether a sentence is a pangram.
pub fn is_pangram(sentence: &str) -> bool {
    use std::collections::HashSet;

    26 == sentence
        .bytes()
        .filter_map(|b| {
            if b.is_ascii_alphabetic() {
                Some(b.to_ascii_lowercase())
            } else {
                None
            }
        })
        .collect::<HashSet<_>>()
        .len()
}
