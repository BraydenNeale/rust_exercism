pub fn encode(msg: &str) -> String {
	const GROUP_SIZE: u8 = 5;
	let mut code = String::with_capacity(msg.len());
	let mut i = 0;

	msg.to_lowercase().chars()
		.filter(|ch| ch.is_ascii() && !ch.is_ascii_punctuation() && !ch.is_ascii_whitespace())
		.for_each(|ch| {
			if i == GROUP_SIZE {
				code.push(' ');
				i = 0;
			}

			code.push(cipher(ch));
			i += 1;
		});


	code
}

pub fn decode(code: &str) -> String {
	let mut msg = String::with_capacity(code.len());

	code.chars().filter(|ch| !ch.is_ascii_whitespace())
		.for_each(|ch| {
			msg.push(cipher(ch));
		});

	msg
}

fn cipher(ch: char) -> char {
	const A: u8 = 97;
	const Z: u8 = 122;
	const ALPHABET_LENGTH: u8 = 26;

	let asc = ch as u8;
	let mut enc =  Z - asc;

	if enc < ALPHABET_LENGTH {
		enc += A
	}

	if enc < A || enc > Z {
		// reset numbers
		enc = asc;
	}

	enc as char
}

