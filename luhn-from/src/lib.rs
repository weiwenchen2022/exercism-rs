pub struct Luhn {
    digits: Vec<u32>,
}

impl Luhn {
    pub fn is_valid(&self) -> bool {
        if self.digits.len() <= 1 {
            return false;
        }
        self.digits.iter().sum::<u32>() % 10 == 0
    }
}

/// Here is the example of how the From trait could be implemented
/// for the &str type. Naturally, you can implement this trait
/// by hand for the every other type presented in the test suite,
/// but your solution will fail if a new type is presented.
/// Perhaps there exists a better solution for this problem?
impl<T: ToString> From<T> for Luhn {
    fn from(input: T) -> Self {
        let input = input.to_string();
        if !input
            .bytes()
            .all(|ch| ch.is_ascii_whitespace() || ch.is_ascii_digit())
        {
            return Self { digits: Vec::new() };
        }

        let mut digits = input
            .bytes()
            .filter_map(|ch| {
                if (ch as char).is_ascii_digit() {
                    (ch as char).to_digit(10)
                    // Some((ch - b'0') as u32)
                } else {
                    None
                }
            })
            .collect::<Vec<_>>();

        digits.iter_mut().rev().enumerate().for_each(|(i, digit)| {
            if (i + 1) % 2 == 0 {
                *digit *= 2;
                if *digit > 9 {
                    *digit -= 9;
                }
            }
        });

        Self { digits }
    }
}
