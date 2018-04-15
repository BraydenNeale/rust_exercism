pub fn find_saddle_points(input: &[Vec<u64>]) -> Vec<(usize, usize)> {
    let mut rows: Vec<(usize, usize)> = Vec::new();
    let mut cols: Vec<(usize, usize)> = Vec::new();

    if input.len() == 0 || input[0].len() == 0 {
        return rows;
    }

    // find maxs for each row
    for x in 0..input.len() {
        let mut max = input[x][0];
        let mut maxs: Vec<(usize, usize)> = Vec::new();

        for y in 0..input[x].len() {
            let val = input[x][y];
            if val > max {
                max = val;
                maxs = vec![(x,y)];
            } else if val == max {
                maxs.push((x,y));
            }
        }

        for tuple in maxs {
            rows.push(tuple);
        }
    }

    // find mins for each col
    for y in 0..input[0].len() {
        let mut min = input[0][y];
        let mut mins: Vec<(usize, usize)> = Vec::new();

        for x in 0..input.len() {
            let val = input[x][y];
            if val < min {
                min = val;
                mins = vec![(x,y)];
            } else if val == min {
                mins.push((x,y));
            }
        }

        for tuple in mins {
            cols.push(tuple);
        }
    }

    // rows.iter().filter(|x| cols.contains(x)).collect::<Vec<_>>()
    rows.retain(|x| cols.contains(x));
    rows
}

// nicer looking solution... I think it's less efficient though
pub fn col(input: &[Vec<u64>],i: usize) -> Vec<u64>{
    input.iter().map(|v| v[i]).collect()
}
pub fn find_saddle_points(input: &[Vec<u64>]) -> Vec<(usize, usize)> {
    let mut result = vec![];
    for i in 0..input.len(){
        for j in 0..input[i].len(){
            let current = input[i][j];
            let column = col(input,j);
            let &max_by_row = input[i].iter().max().unwrap();
            let &min_by_col = column.iter().min().unwrap();

            if current >= max_by_row && current <= min_by_col {
                result.push((i,j));
            }
        }
    }
    result
}