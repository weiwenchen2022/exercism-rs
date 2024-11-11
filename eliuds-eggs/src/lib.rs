pub fn egg_count(mut display_value: u32) -> usize {
    let mut eggs = 0;
    while display_value != 0 {
        display_value &= display_value - 1;
        eggs += 1;
    }
    eggs
}
