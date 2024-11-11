pub fn factors(mut n: u64) -> Vec<u64> {
    // todo!("This should calculate the prime factors of {n}")

    if n < 2 {
        return Vec::new();
    }

    let mut primes = Vec::new();
    let mut x = 2;
    while n != 1 {
        if n % x == 0 {
            n /= x;
            primes.push(x);
            continue;
        }

        x += 1;
    }

    primes
}
