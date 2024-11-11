use std::collections::BTreeMap;

pub fn transform(h: &BTreeMap<i32, Vec<char>>) -> BTreeMap<char, i32> {
    // todo!("How will you transform the tree {h:?}?")

    use std::iter;

    h.iter()
        .flat_map(|(&point, letters)| {
            letters
                .iter()
                .map(char::to_ascii_lowercase)
                .zip(iter::repeat(point))
        })
        .collect()
}
