pub fn encrypt(msg: &str) -> String {
    let s = msg.chars()
        .filter(|c| c.is_alphabetic())
        .collect::<String>()
        .to_lowercase();

    let len = s.len();

    let mut cols = 1;
    let mut rows = cols-1;

    while cols * rows < len {
        rows += 1;

        if cols < rows {
            cols += 1;
            rows = cols - 1;
        }
    }

    let mut code = String::with_capacity(len);

    for i in 0..cols {
        for j in 0..rows {
            let index = i + j*cols;
            if index > len {
                break;
            }

            // rust char_at alternative... without as_bytes
            if let Some(c) = s[index..].chars().next() {
                code.push(c)
            }
        }

        code.push(' ');
    }

    code.pop();
    code
}

// functional solution

// pub fn encrypt(s: &str) -> String {
//     let s = normalize(s);
//     let rectangle = get_dimensions(s.len() as i32).unwrap_or(RowCol { r: 1, c: 1 });

//     to_rows(&s, rectangle.c as usize).join(" ")
// }

// fn normalize(s: &str) -> String {
//     s.chars()
//         .filter(|c| c.is_ascii_alphabetic())
//         .map(|c| c.to_ascii_lowercase())
//         .collect()
// }

// fn to_rows(s: &str, lines: usize) -> Vec<String> {
//     let mut result: Vec<Vec<char>> = vec![vec![]; lines];
//     for (i, c) in s.chars().enumerate() {
//         result[i % lines].push(c);
//     }

//     result
//         .iter()
//         .map(|c| c.iter().collect::<String>())
//         .collect::<Vec<String>>()
// }

// #[derive(Debug)]
// struct RowCol {
//     r: i32,
//     c: i32,
// }
// /// Works out the columns and rows of the rectangle.
// fn get_dimensions(size: i32) -> Option<RowCol> {
//     (0..size / 2 + 1)
//         .flat_map(|s| vec![s, s])
//         .zip((0..size / 2 + 1).flat_map(|s| vec![s, s + 1]))
//         .find(|&(r, c)| r * c >= size)
//         .map(|(r, c)| RowCol { r, c })
// }

