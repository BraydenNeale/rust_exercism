pub fn rotate(msg: &str, rotation: u8) -> String {
	const LOWER_A: u8 = 'a' as u8;
	const LOWER_Z: u8 = 'z' as u8;
	const UPPER_A: u8 = 'A' as u8;
	const UPPER_Z: u8 = 'Z' as u8;
	const ALPHABET_LEN: u8 = 26;

	let mut rot = rotation;
	if rot > ALPHABET_LEN {
		rot = rotation % ALPHABET_LEN;
	}

	msg.chars()
		.filter(|c| c.is_ascii())
		.map(|c| {
			let mut asc = c as u8;

			if asc >= LOWER_A && asc <= LOWER_Z {
				asc += rot;
				if asc > LOWER_Z {
					asc -= ALPHABET_LEN;
				}
			} else if asc >= UPPER_A && asc <= UPPER_Z {
				asc += rot;
				if asc > UPPER_Z {
					asc -= ALPHABET_LEN;
				}
			}

			asc as char
		})
		.collect::<String>()
}