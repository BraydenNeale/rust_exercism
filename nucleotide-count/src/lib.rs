use std::collections::HashMap;

pub fn count(ch: char, dna: &str) -> Result<usize, &'static str> {
	Ok(dna.chars().filter(|&c| {
		if !valid_char(ch) {
			return Err("invalid sequence");
		}
		c == ch
	}).count())
}

pub fn nucleotide_counts(dna: &str) -> Result<HashMap<char, usize>, &'static str> {

	let mut map: HashMap<char, usize> = HashMap::with_capacity(4);
	map.insert('A', 0);
	map.insert('C', 0);
	map.insert('G', 0);
	map.insert('T', 0);

	dna.chars().for_each(|ref c| {
		if !valid_char(c) {
			return Err("invalid sequence");
		} else if let Some(x) = map.get_mut(c) {
    		*x += 1;
    	}
	});

	Ok(map)
}

fn valid_char(ch: char) -> bool {
	"ACGT".contains(ch)
}