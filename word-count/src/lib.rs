use std::collections::HashMap;

pub fn word_count(s: &str) -> HashMap<String, u32> {
	let mut map: HashMap<String, u32> = HashMap::new();

	s.split_whitespace().for_each(|word| {
		let clean: String = word.to_lowercase().chars()
			.filter(|c| !c.is_ascii_punctuation()).collect();

		if !clean.is_empty()  {
			let count = map.entry(clean).or_insert(0);
			*count += 1;
		}
	});
	
	map
}