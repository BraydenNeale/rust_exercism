pub struct PascalsTriangle {
    rows: u32,
}

impl PascalsTriangle {
    pub fn new(row_count: u32) -> Self {
        PascalsTriangle { rows: row_count }
    }

    pub fn rows(&self) -> Vec<Vec<u32>> {
        use std::iter;

        let mut result: Vec<Vec<u32>> = if self.rows == 0 {
            vec![]
        } else {
            vec![vec![1]]
        };

        for i in 1..self.rows {
            let last_row: Vec<u32> = iter::once(0u32)
                .chain(result[(i - 1) as usize].iter().cloned())
                .chain(iter::once(0u32))
                .collect();

            result.push(last_row.windows(2).map(|x| x.iter().sum()).collect());
        }
        result
    }
}