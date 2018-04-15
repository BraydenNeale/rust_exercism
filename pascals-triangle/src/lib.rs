pub struct PascalsTriangle {
	tri: Vec<Vec<u32>>
}

impl PascalsTriangle {
    pub fn new(row_count: u32) -> Self {
        PascalsTriangle {
        	tri: build_triangle(row_count)
        }
    }

    pub fn rows(&self) -> Vec<Vec<u32>> {
        self.tri.clone()
    }
}

fn build_triangle(row_count: u32) -> Vec<Vec<u32>> {
	let mut tri = Vec::new();

	if row_count < 1 {
		return tri;
	}

	let mut prev_row = vec![1];
	tri.push(prev_row.clone());

	for _ in 2..row_count+1 {
		prev_row = tri.last().cloned().unwrap();
		let mut new_row = Vec::with_capacity(prev_row.len()+1);
		let mut iter = prev_row.iter();
		let mut last_num = 0;

		while let Some(num) = iter.next() {
			new_row.push(*num + last_num);
			last_num = *num;
		}

		new_row.push(last_num);
		tri.push(new_row);
	}

	tri
}
