pub fn translate(input: &str) -> String {
    // todo!("Using the Pig Latin text transformation rules, convert the given input '{input}'");

    let vowels = ['a', 'e', 'i', 'o', 'u'];
    let is_consonant = |ch: char| ch.is_ascii_alphabetic() && !vowels.contains(&ch);

    input
        .split_whitespace()
        .map(|word| {
            let mut word = word.to_string();

            if word.starts_with(vowels) || word.starts_with("xr") || word.starts_with("yt") {
                // rule1
                word.push_str("ay");
            } else if matches!(word.find("qu"), Some(index) if word[..index].chars().all(is_consonant)) {
                // rule3
                let index = word.find("qu").unwrap();
                word = word[index + 2..].to_string() + &word[..index + 2];
                word += "ay";
            } else if matches!(word.find('y'), Some(index) if index > 0 && word[..index].chars().all(is_consonant)) {
                // rule4
                let index = word.find('y').unwrap();
                word = word[index..].to_string() + &word[..index];
                word += "ay";
            } else if word.starts_with(is_consonant) {
                // rule2
                let first_non_consonant = word.chars()
                    .position(|ch| !is_consonant(ch))
                    .unwrap_or(word.len());

                word = word[first_non_consonant..].to_string() + &word[..first_non_consonant];
                word += "ay";
            }

            word
        })
        .collect::<Vec<_>>()
        .join(" ")
}
