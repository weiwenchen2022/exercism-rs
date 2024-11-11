pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    // todo!("Sum the multiples of all of {factors:?} which are less than {limit}")

    use std::collections::HashSet;
    use std::iter;

    factors
        .iter()
        .flat_map(|&factor| -> Box<dyn Iterator<Item = _>> {
            if factor > 0 {
                Box::new((1..).map(move |n| factor * n).take_while(|n| n < &limit))
            } else {
                Box::new(iter::empty())
            }
        })
        .collect::<HashSet<_>>()
        .iter()
        .sum()
}
