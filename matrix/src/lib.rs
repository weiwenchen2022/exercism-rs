pub struct Matrix {
    // Implement your Matrix struct
    rows: Vec<Vec<u32>>,
}

impl Matrix {
    pub fn new(input: &str) -> Self {
        let rows = input
            .lines()
            .map(|line| {
                line.split_whitespace()
                    .map(|n| n.parse::<u32>().unwrap())
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();
        Self { rows }
    }

    pub fn row(&self, row_no: usize) -> Option<Vec<u32>> {
        self.rows.get(row_no - 1).cloned()
    }

    pub fn column(&self, col_no: usize) -> Option<Vec<u32>> {
        let col_no = col_no - 1;
        if col_no >= self.rows.first().unwrap_or(&Vec::new()).len() {
            return None;
        }
        Some(self.rows.iter().map(|row| row[col_no]).collect())
    }
}
