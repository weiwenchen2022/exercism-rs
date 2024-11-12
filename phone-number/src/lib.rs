pub fn number(user_number: &str) -> Option<String> {
    // todo!(
    //     "Given the number entered by user '{user_number}', convert it into SMS-friendly format. If the entered number is not a valid NANP number, return None."
    // );

    let user_number = if user_number.starts_with("1") {
        user_number.trim_start_matches("1")
    } else if user_number.starts_with("+1") {
        user_number.trim_start_matches("+1")
    } else {
        user_number
    };

    let user_number = user_number
        .chars()
        .filter(|ch| ch.is_ascii_digit())
        .enumerate()
        .map(|(index, digit)| {
            if (index == 0 || index == 3) && !matches!(digit, '2'..='9') {
                None
            } else {
                Some(digit)
            }
        })
        .collect::<Option<String>>()?;

    if user_number.len() == 10 {
        Some(user_number)
    } else {
        None
    }
}
