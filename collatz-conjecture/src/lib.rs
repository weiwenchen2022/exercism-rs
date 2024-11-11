pub fn collatz(n: u64) -> Option<u64> {
    // todo!("return Some(x) where x is the number of steps required to reach 1 starting with {n}")

    if n == 0 {
        return None;
    }

    use std::iter;

    Some(
        iter::successors(Some(n), |&n| {
            if n == 1 {
                return None;
            }

            if n & 0x1 == 0 {
                Some(n / 2)
            } else {
                Some(n * 3 + 1)
            }
        })
        .count() as u64
            - 1,
    )
}
