pub fn abbreviate(name: &str) -> String {
	let mut acr = String::new();

	name.split_whitespace()
		.for_each(|s| {
			let mut iter = s.chars();
			let mut last = ' ';
			while let Some(c) = iter.next() {
				if last == ' ' || last == '-' {
					acr.push(c.to_ascii_uppercase());
				} else if last.is_ascii_lowercase() && c.is_ascii_uppercase() {
					acr.push(c)
				}

				last = c;
			}
		});

	acr
}

// functional
// pub fn abbreviate(input: &str) -> String {
//     let mut s = String::from(" ");
//     s.push_str(input);

//     s.chars()
//     	.collect::<Vec<_>>()
//         .windows(2)
//         .filter(|pair|
//             (!pair[0].is_alphanumeric() && pair[1].is_alphanumeric()) ||
//             (!pair[0].is_uppercase() && pair[1].is_uppercase())
//         )
//         .map(|pair| pair[1])
//         .collect::<String>()
//         .to_uppercase()
// }