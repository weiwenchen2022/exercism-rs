pub fn nth(n: u32) -> u32 {
    // todo!("What is the 0-indexed {n}th prime number?")

    use std::iter;

    let mut prime_numbers: Vec<u32> = Vec::with_capacity(n as usize);
    iter::from_fn(|| {
        if prime_numbers.is_empty() {
            prime_numbers.push(2);
            prime_numbers.last().copied()
        } else {
            let mut next_prime = prime_numbers.last().copied().unwrap();
            next_prime += 1;
            while prime_numbers.iter().any(|prime| next_prime % prime == 0) {
                next_prime += 1;
            }

            prime_numbers.push(next_prime);
            prime_numbers.last().copied()
        }
    })
    .nth(n as usize)
    .unwrap()
}
