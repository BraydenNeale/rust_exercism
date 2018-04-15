use std::ascii::AsciiExt;

// pub fn is_pangram(sentence: &str) -> bool {
// 	const ALPHABET_LEN: usize = 26;
// 	let mut seen = Vec::with_capacity(ALPHABET_LEN);
// 	sentence.to_lowercase().chars()
// 		.filter(|c| c.is_alphabetic() && c.is_ascii())
// 		.for_each(|c| {
// 			if !seen.contains(&c) {
// 				seen.push(c);
// 			}
// 		});

// 	seen.len() == ALPHABET_LEN
// }

pub fn is_pangram(s: &str) -> bool {
    let mut seen_letter: Vec<bool> = vec![false; 26];

    for index in s.chars()
        .filter(|c| c.is_ascii())
        .filter_map(|c| c.to_lowercase().next())
        .filter(|c| (*c as u8) >= b'a' && (*c as u8) <= b'z')
        .map(|c| (c as u8 - b'a') as usize)
    {
        seen_letter[index] = true;
    }

    seen_letter.into_iter().all(|x| x)
}
