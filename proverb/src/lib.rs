pub fn build_proverb(list: &[&str]) -> String {
    // todo!("build a proverb from this list of items: {list:?}")

    list.iter()
        .enumerate()
        .map(|(i, &item)| {
            if i < list.len() - 1 {
                format!(
                    "For want of a {} the {} was lost.",
                    item,
                    list.get(i + 1).unwrap()
                )
            } else {
                format!("And all for the want of a {}.", list.first().unwrap())
            }
        })
        .collect::<Vec<_>>()
        .join("\n")
}
