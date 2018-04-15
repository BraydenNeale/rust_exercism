pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    (1..limit).filter(|x| has_factor(x, factors)).sum()
}

fn has_factor(check: &u32, factors: &[u32]) -> bool {
	for val in factors {
		if check % val == 0 { return true; }
	}

	false
}


// nicer functional way
// pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
//     (1..limit).filter(|i| factors.iter().any(|f| i % f == 0)).sum()
// }