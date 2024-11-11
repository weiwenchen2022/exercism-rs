pub fn verse(n: u32) -> String {
    // todo!("emit verse {n}")

    #[allow(clippy::comparison_chain)]
    if n > 1 {
        format!(
            "{} bottles of beer on the wall, {0} bottles of beer.\n\
            Take one down and pass it around, {} bottle{} of beer on the wall.\n\
            ",
            n,
            n - 1,
            if n - 1 > 1 { "s" } else { "" }
        )
    } else if n == 1 {
        "1 bottle of beer on the wall, 1 bottle of beer.\n\
        Take it down and pass it around, no more bottles of beer on the wall.\n\
        "
        .to_string()
    } else {
        "No more bottles of beer on the wall, no more bottles of beer.\n\
        Go to the store and buy some more, 99 bottles of beer on the wall.\n\
        "
        .to_string()
    }
}

pub fn sing(start: u32, end: u32) -> String {
    // todo!("sing verses {start} to {end}, inclusive")

    let (start, end) = if start > end {
        (end, start)
    } else {
        (start, end)
    };
    (start..=end)
        .rev()
        .map(verse)
        .collect::<Vec<_>>()
        .join("\n")
}
