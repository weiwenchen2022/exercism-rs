/// Determines whether the supplied string is a valid ISBN number
pub fn is_valid_isbn(isbn: &str) -> bool {
    // todo!("Is {isbn:?} a valid ISBN number?");

    if isbn
        .chars()
        .any(|ch| !(ch.is_ascii_digit() || ch == '-' || ch == 'X'))
    {
        return false;
    }

    let digits = isbn
        .chars()
        .filter_map(|digit| {
            if digit.is_ascii_digit() {
                digit.to_digit(10)
            } else if digit == 'X' {
                Some(10)
            } else {
                None
            }
        })
        .collect::<Vec<_>>();

    if digits.len() != 10 {
        return false;
    }

    if matches!(digits.iter().position(|digit| &10 == digit), Some(index) if index != digits.len()-1)
    {
        return false;
    }

    digits
        .iter()
        .zip((1..=10).rev())
        .map(|(d, factor)| d * factor)
        .sum::<u32>()
        % 11
        == 0
}
