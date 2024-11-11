/// Check a Luhn checksum.
pub fn is_valid(code: &str) -> bool {
    // todo!("Is the Luhn checksum for {code} valid?");

    if code.len() <= 1 {
        return false;
    }

    if !code
        .bytes()
        .all(|ch| ch.is_ascii_whitespace() || ch.is_ascii_digit())
    {
        return false;
    }

    let digits = code
        .bytes()
        .filter_map(|ch| {
            if (ch as char).is_ascii_digit() {
                (ch as char).to_digit(10)
            } else {
                None
            }
        })
        .collect::<Vec<_>>();
    if digits.len() <= 1 {
        return false;
    }

    digits
        .into_iter()
        .rev()
        .enumerate()
        .map(|(i, mut digit)| {
            if (i + 1) % 2 == 0 {
                digit *= 2;
                if digit > 9 {
                    digit -= 9;
                }
            }
            digit
        })
        .sum::<u32>()
        % 10
        == 0
}
