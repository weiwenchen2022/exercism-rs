#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    InvalidInputBase,
    InvalidOutputBase,
    InvalidDigit(u32),
}

///
/// Convert a number between two bases.
///
/// A number is any slice of digits.
/// A digit is any unsigned integer (e.g. u8, u16, u32, u64, or usize).
/// Bases are specified as unsigned integers.
///
/// Return the corresponding Error enum if the conversion is impossible.
///
///
/// You are allowed to change the function signature as long as all test still pass.
///
///
/// Example:
/// Input
///   number: &[4, 2]
///   from_base: 10
///   to_base: 2
/// Result
///   Ok(vec![1, 0, 1, 0, 1, 0])
///
/// The example corresponds to converting the number 42 from decimal
/// which is equivalent to 101010 in binary.
///
///
/// Notes:
///  * The empty slice ( "[]" ) is equal to the number 0.
///  * Never output leading 0 digits, unless the input number is 0, in which the output must be `[0]`.
///    However, your function must be able to process input with leading 0 digits.
///
pub fn convert(number: &[u32], from_base: u32, to_base: u32) -> Result<Vec<u32>, Error> {
    // todo!("Convert {number:?} from base {from_base} to base {to_base}")

    if !(2..=99).contains(&from_base) {
        return Err(Error::InvalidInputBase);
    }

    if !(2..=99).contains(&to_base) {
        return Err(Error::InvalidOutputBase);
    }

    for &digit in number {
        if digit >= from_base {
            return Err(Error::InvalidDigit(digit));
        }
    }

    if number.is_empty() {
        return Ok(vec![0]);
    }

    let mut number_in_decimal = number
        .iter()
        .zip((0..number.len()).rev())
        .map(|(digit, e)| digit * from_base.pow(e as u32))
        .sum::<u32>();

    if number_in_decimal == 0 {
        return Ok(vec![0]);
    }

    eprintln!("{}", number_in_decimal);

    let mut number_to_base = Vec::new();

    while number_in_decimal != 0 {
        let digit = number_in_decimal % to_base;
        number_to_base.push(digit);
        number_in_decimal /= to_base;
    }

    number_to_base.reverse();
    let number_to_base = number_to_base
        .into_iter()
        .skip_while(|&digit| digit == 0)
        .collect();

    Ok(number_to_base)
}
