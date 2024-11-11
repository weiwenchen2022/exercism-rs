pub fn is_armstrong_number(num: u32) -> bool {
    // todo!("true if {num} is an armstrong number")

    let digits = num
        .to_string()
        .as_bytes()
        .iter()
        .map(|&digit| (digit as char).to_digit(10).unwrap())
        .collect::<Vec<_>>();

    num == digits
        .iter()
        .map(|digit| digit.pow(digits.len() as u32))
        .sum::<u32>()
}
