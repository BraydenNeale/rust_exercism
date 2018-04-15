pub fn hamming_distance(dna1: &str, dna2: &str) -> std::result::Result<u32, &'static str> {
	if dna1.chars().count() != dna2.chars().count() {
		return Err("Can't compare unequal strand lengths");
	}

	Ok(dna1.chars()
		.enumerate()
		.fold(0,|sum, (i, ch)| {
			sum + (dna2.chars().nth(i).unwrap() != ch) as u32
		})
	)
}

// with zip instead of fold
// pub fn hamming_distance(strand_a: &str, strand_b: &str) -> Result<u32, &'static str> {
//     if strand_a.len() != strand_b.len() {
//         return Err("Strands not the same length");
//     }

//     Ok(strand_a.chars()
//                .zip(strand_b.chars())
//                .filter(|&(a, b)| a != b)
//                .count() as u32)
// }