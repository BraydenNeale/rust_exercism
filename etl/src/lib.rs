use std::collections::BTreeMap;

pub fn transform(scores: &BTreeMap<i32, Vec<char>>) -> BTreeMap<char, i32> {
	let mut map = BTreeMap::new();

	for (key, value) in scores.iter() {
    	value.iter().for_each(|c| {
    		map.insert(c.to_ascii_lowercase(), *key);
    	})
	}

	map
}