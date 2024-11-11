pub fn annotate(minefield: &[&str]) -> Vec<String> {
    // todo!("\nAnnotate each square of the given minefield with the number of mines that surround said square (blank if there are no surrounding mines):\n{minefield:#?}\n");

    let mut board = Vec::with_capacity(minefield.len());

    for (i, &row) in minefield.iter().enumerate() {
        let mut board_row = Vec::with_capacity(row.len());

        let row = row.as_bytes();
        for (j, space) in row.iter().enumerate() {
            if b'*' == *space {
                board_row.push(b'*');
                continue;
            }

            let dirs = [
                (-1, 0),  // west
                (-1, 1),  // west-north
                (0, 1),   // east
                (1, 1),   // east-north
                (1, 0),   // east
                (1, -1),  // east-south
                (0, -1),  // south
                (-1, -1), // west-north
            ];

            let mut mines = 0;
            for dir in dirs {
                let Some(row_as_bytes) = minefield
                    .get(i.wrapping_add_signed(dir.1))
                    .map(|r| r.as_bytes())
                else {
                    continue;
                };

                if matches!(row_as_bytes.get(j.wrapping_add_signed(dir.0)), Some(&b'*')) {
                    mines += 1;
                }
            }

            if mines > 0 {
                board_row.push(mines.to_string().as_bytes()[0]);
            } else {
                board_row.push(b' ');
            }
        }

        board.push(String::from_utf8(board_row).unwrap());
    }

    board
}
