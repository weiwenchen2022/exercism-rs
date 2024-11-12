use std::collections::HashMap;

/// Count occurrences of words.
pub fn word_count(words: &str) -> HashMap<String, u32> {
    // todo!("Count of occurrences of words in {words:?}");

    let mut word_count = HashMap::new();
    for word in words.split_terminator(|ch: char| {
        (ch.is_ascii_punctuation() && ch != '\'') || ch.is_ascii_whitespace()
    }) {
        let word = word.trim_matches('\'');
        if word.is_empty() {
            continue;
        }

        *word_count.entry(word.to_ascii_lowercase()).or_default() += 1;
    }

    word_count
}
