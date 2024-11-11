pub fn find<S: AsRef<[T]>, T: Ord>(array: S, key: T) -> Option<usize> {
    use std::cmp::Ordering;

    let array = array.as_ref();

    let mut left = 0;
    let mut right = array.len();
    while left < right {
        let middle = (left + right) / 2;
        match array[middle].cmp(&key) {
            Ordering::Equal => return Some(middle),
            Ordering::Greater => right = middle,
            Ordering::Less => left = middle + 1,
        }
    }
    None
}
