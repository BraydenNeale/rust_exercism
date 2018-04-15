pub fn check(test: &str) -> bool {
	let copy = test.trim().to_lowercase();
	let mut iter = copy.chars();

	while let Some(ch) = iter.next() {
		if ch.is_alphabetic() && iter.as_str().contains(ch) {
			return false;
		}
	}

	true
}

// Hash map user submission
use std::collections::HashMap;

// pub fn check(input: &str) -> bool {
//     let filtered_word = filter_word(input);
//     let mut word_list = HashMap::new();

//     for letter in filtered_word.chars() {
//         let count = word_list.entry(letter).or_insert(0);
//         *count += 1;
//     }

//     word_list.iter().all(|(_, &v)| v == 1)
// }

// fn filter_word(input: &str) -> String {
//     input
//         .chars()
//         .filter(|x| x.is_alphabetic())
//         .map(|x| x.to_lowercase().to_string())
//         .collect::<String>()
// }