pub fn find() -> Option<u32> {
	const SUM: u32 = 1000;

    for a in 1..SUM/3 {
        for b in 1..(SUM - a) {
            let c = SUM - a - b;
            if is_triplet((a,b,c)) {
                return Some(a * b * c);
            }
        }
    }

    None
}

fn is_triplet((a,b,c): (u32,u32,u32)) -> bool {
	(a*a) + (b*b) == (c*c)
}
