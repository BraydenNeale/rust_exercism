use std::iter;

trait PascalsTriangleRow {
    fn next(&self) -> Self;
}

impl PascalsTriangleRow for Vec<u32> {
    fn next(&self) -> Self {
        iter::once(1)
            .chain(self.windows(2).map(|w| w[0] + w[1]))
            .chain(iter::once(1))
            .collect()
    }
}

pub struct PascalsTriangle {
    count: u32,
}

impl PascalsTriangle {
    pub fn new(row_count: u32) -> Self {
        Self { count: row_count }
    }

    pub fn rows(&self) -> Vec<Vec<u32>> {
        if self.count == 0 {
            return vec![];
        }
        let mut rows = vec![vec![1]];
        for _ in 1..self.count {
            let next = rows[rows.len() - 1].next();
            rows.push(next);
        }
        rows
    }
}
