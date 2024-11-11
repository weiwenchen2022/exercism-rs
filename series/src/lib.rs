pub fn series(digits: &str, len: usize) -> Vec<String> {
    if digits.len() < len {
        return Vec::new();
    }

    if len == 0 {
        return vec!["".to_string(); digits.len() + 1];
    }

    (0..=digits.len() - len)
        .map(|i| digits[i..i + len].to_string())
        .collect()
}
