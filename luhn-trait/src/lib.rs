pub trait Luhn {
    fn valid_luhn(&self) -> bool;
}

/// Here is the example of how to implement custom Luhn trait
/// for the &str type. Naturally, you can implement this trait
/// by hand for the every other type presented in the test suite,
/// but your solution will fail if a new type is presented.
/// Perhaps there exists a better solution for this problem?
impl<T: ToString> Luhn for T {
    fn valid_luhn(&self) -> bool {
        let number = self.to_string();
        if number.len() <= 1 {
            return false;
        }

        if !number
            .bytes()
            .all(|ch| ch.is_ascii_whitespace() || ch.is_ascii_digit())
        {
            return false;
        }

        let digits = number
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
            .iter()
            .rev()
            .enumerate()
            .map(|(i, &digit)| {
                let mut digit = digit;
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
}
