pub fn is_valid(luhn: &str) -> bool {
	let s = luhn.trim();

	if s.len() < 2 {
		return false;
	}

	// would like to be able to drop this check
	if s.chars().any(|x| !x.is_whitespace() && !x.is_numeric()) {
		return false;
	}

	let mut idx = 1;
	let sum = s.chars().rev().fold(0, |acc, c| {
		match c {
			' ' => acc,
			_ => {
				// ideally would return false if c.to_digit returns None...
				// but not sure how to do it from within this closure
				let mut x = c.to_digit(10).unwrap();

				if idx % 2 == 0 {
					x *= 2;
					if x > 9 {
						x -= 9;
					}
				}
				idx += 1;
				acc + x
			}
		}
    });

	sum % 10 == 0
}
