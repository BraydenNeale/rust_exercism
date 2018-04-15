// /// Determines whether the supplied string is a valid ISBN number
// pub fn is_valid_isbn(isbn: &str) -> bool {
// 	if isbn.trim().chars().count() > 13 { return false };

// 	let mut x: Vec<u32> = Vec::with_capacity(10);
// 	let mut count = 0;
// 	let mut iter = isbn.trim().chars();

// 	while let Some(ch) = iter.next() {
// 		match ch {
// 			_ if ch.is_numeric() => {
// 				count += 1;

// 				x.push(ch.to_digit(10).unwrap());
// 			},

// 			_ if ch == 'X' || ch == 'x' => {
// 				count += 1;

// 				if let Some(_) = iter.next() {
//         			return false;
//     			}

// 				x.push(10)
// 			}

// 			'-' => continue,

// 			_ => return false,
// 		}
// 	}

// 	if count != 10 {
// 		return false;
// 	}

// 	(x[0] * 10 + x[1] * 9 + x[2] * 8 + 
// 		x[3] * 7 + x[4] * 6 + x[5] * 5 + 
// 		x[6] * 4 + x[7] * 3 + x[8] * 2 + 
// 		x[9] * 1) % 11 == 0
// }

// beautiful functional solution
/// Determines whether the supplied string is a valid ISBN number
pub fn is_valid_isbn(isbn: &str) -> bool {
    let x: Vec<u32> = isbn.chars()
        .filter(|c| c.is_digit(10) || *c == 'X')
        .map(|c| c.to_digit(10).unwrap_or(10) )
        .collect();
    
    x.len() == 10
    && x[0..9].iter().filter(|n| **n < 10).count() == 9
    && (x[0] * 10 + x[1] * 9 + x[2] * 8 + x[3] * 7 + x[4] * 6 +
        x[5] * 5 + x[6] * 4 + x[7] * 3 + x[8] * 2 + x[9] * 1) % 11 == 0
}
