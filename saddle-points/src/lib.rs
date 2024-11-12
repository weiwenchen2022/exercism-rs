pub fn find_saddle_points(input: &[Vec<u64>]) -> Vec<(usize, usize)> {
    // todo!("find the saddle points of the following matrix: {input:?}")

    use std::cell::RefCell;
    use std::iter;
    use std::rc::Rc;

    let column_heights = Rc::new(RefCell::new(Vec::with_capacity(input.len())));

    input
        .iter()
        .enumerate()
        .flat_map(
            |(row, row_heights)| -> Box<dyn Iterator<Item = (usize, usize)>> {
                let Some(largest) = row_heights.iter().max() else {
                    return Box::new(iter::empty());
                };

                let largest = row_heights
                    .iter()
                    .enumerate()
                    .filter_map(|(i, h)| if largest == h { Some(i) } else { None })
                    .collect::<Vec<_>>();

                let column_heights = Rc::clone(&column_heights);
                Box::new((0..row_heights.len()).filter_map(move |column| {
                    if !largest.contains(&column) {
                        return None;
                    }

                    let mut column_heights = column_heights.borrow_mut();
                    let column_heights = match column_heights.get(column) {
                        Some(column_heights) => column_heights,
                        None => {
                            (column_heights.len()..=column).for_each(|col| {
                                let col_heights = (0..input.len())
                                    .map(|row| input[row][col])
                                    .collect::<Vec<_>>();
                                column_heights.push(col_heights);
                            });
                            &column_heights[column]
                        }
                    };

                    let smallest = column_heights.iter().min().unwrap();
                    let smallest = column_heights
                        .iter()
                        .enumerate()
                        .filter_map(|(j, h)| if smallest == h { Some(j) } else { None })
                        .collect::<Vec<_>>();

                    if smallest.contains(&row) {
                        Some((row, column))
                    } else {
                        None
                    }
                }))
            },
        )
        .collect()
}
