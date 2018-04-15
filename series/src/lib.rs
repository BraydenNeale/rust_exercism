pub fn series(digits: &str, len: usize) -> Vec<String> {
    let mut v: Vec<String> = Vec::new();
    let mut start = 0;
    let mut end = start + len;
    let count = digits.chars().count();

    while end <= count {
    	v.push(digits[start..end].to_string());
    	start += 1;
    	end = start + len;
    }

    v
}

// functional
// pub fn series(digits: &str, len: usize) -> Vec<String> {
//     if len > digits.len() {
//         return vec![];
//     }
//     let until = digits.len() - len + 1;
//     (0..until).map(|i| digits[i..(i + len)].to_string()).collect()
// }

