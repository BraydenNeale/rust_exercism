pub fn encode(data: &str) -> String {
	let mut enc: Vec<String> = Vec::new();
	let mut iter = data.chars().peekable();

	while let Some(ch) = iter.next() {
		let mut count = 1;
		while iter.peek() == Some(&ch) {
			count += 1;
			iter.next();
		}

		match count {
			1 => enc.push(ch.to_string()),
			_ => enc.push(format!("{}{}", count, ch)),
		}
	}

	enc.concat()
}

pub fn decode(data: &str) -> String {
	let mut dec: Vec<String> = Vec::new();
	let mut iter = data.chars();

	while let Some(ch) = iter.next() {
		if ch.is_numeric() {
			let mut num = ch.to_string();
			let mut next = iter.next().unwrap();

			while next.is_numeric() {
				num.push(next);
				next = iter.next().unwrap();
			}

			let count = num.parse::<usize>().unwrap();
			dec.push(vec![next.to_string(); count].concat());
		} else {
			dec.push(ch.to_string());
		}
	}

	dec.concat()
}