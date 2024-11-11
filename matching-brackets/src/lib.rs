pub fn brackets_are_balanced(string: &str) -> bool {
    let mut brackets = Vec::new();
    for ch in string.chars() {
        match ch {
            '[' | '{' | '(' => brackets.push(ch),
            ']' if !matches!(brackets.pop(), Some('[')) => return false,
            '}' if !matches!(brackets.pop(), Some('{')) => return false,
            ')' if !matches!(brackets.pop(), Some('(')) => return false,
            _ => {}
        }
    }
    brackets.is_empty()
}
