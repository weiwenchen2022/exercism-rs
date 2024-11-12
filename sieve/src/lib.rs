pub fn primes_up_to(upper_bound: u64) -> Vec<u64> {
    // todo!("Construct a vector of all primes up to {upper_bound}");

    use std::collections::BTreeSet;

    let mut possible_primes = (2..=upper_bound).collect::<BTreeSet<_>>();
    let mut primes = Vec::new();

    while let Some(first) = possible_primes.pop_first() {
        primes.push(first);

        (1..upper_bound / 2).for_each(|x| {
            possible_primes.remove(&(x * first));
        });
    }

    primes
}
