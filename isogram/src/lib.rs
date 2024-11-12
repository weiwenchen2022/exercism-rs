pub fn check(candidate: &str) -> bool {
    if candidate.is_empty() {
        return true;
    }

    use std::collections::HashSet;

    let mut set = HashSet::new();
    candidate
        .bytes()
        .filter(u8::is_ascii_alphabetic)
        .all(|ch| set.insert(ch.to_ascii_lowercase()))
}
