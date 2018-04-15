#[macro_use]
extern crate lazy_static;

use std::collections::HashMap;

lazy_static! {
    static ref SCORE_CARD: HashMap<char, u32> = {
        let mut map = HashMap::new();
        for c in "aeioulnrst"	.chars() { map.insert(c,1); }
        for c in "dg"			.chars() { map.insert(c,2); }
        for c in "bcmp"			.chars() { map.insert(c,3); }
        for c in "fhvwy"		.chars() { map.insert(c,4); }
        for c in "jx"			.chars() { map.insert(c,8); }
        for c in "qz"			.chars() { map.insert(c,10);}
        map.insert('k',5);
        map
    };
}

pub fn score(word: &str) -> u32 {
	word.to_lowercase().chars().fold(0, |acc, ch| acc + *SCORE_CARD.get(&ch).unwrap_or(&0))
}

