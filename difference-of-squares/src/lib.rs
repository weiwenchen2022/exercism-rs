pub fn square_of_sum(n: u32) -> u32 {
    // todo!("square of sum of 1...{n}")
    (1..=n).sum::<u32>().pow(2)
}

pub fn sum_of_squares(n: u32) -> u32 {
    // todo!("sum of squares of 1...{n}")
    (1..=n).map(|n| n * n).sum()
}

pub fn difference(n: u32) -> u32 {
    // todo!("difference between square of sum of 1...{n} and sum of squares of 1...{n}")
    square_of_sum(n) - sum_of_squares(n)
}
