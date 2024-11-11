use std::collections::HashSet;

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &'a [&'a str]) -> HashSet<&'a str> {
    // todo!("For the '{word}' word find anagrams among the following words: {possible_anagrams:?}");

    let lowercase_word = word.to_lowercase();
    let mut anagrams = HashSet::with_capacity(possible_anagrams.len());

    for &possible_anagram in possible_anagrams {
        let lowercase_anagram = possible_anagram.to_lowercase();

        if lowercase_word == lowercase_anagram {
            continue;
        }

        if lowercase_word.len() != lowercase_anagram.len() {
            continue;
        }

        use std::collections::HashMap;

        let mut lowercase_letters: HashMap<char, usize> = HashMap::new();
        lowercase_anagram
            .chars()
            .for_each(|ch| *lowercase_letters.entry(ch).or_default() += 1);

        if lowercase_word.chars().all(|ch| {
            let c = lowercase_letters.entry(ch).or_default();
            if *c == 0 {
                false
            } else {
                *c -= 1;
                true
            }
        }) {
            anagrams.insert(possible_anagram);
        }
    }

    anagrams
}
