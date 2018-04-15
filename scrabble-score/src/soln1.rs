use std::ascii::AsciiExt;

pub fn score(s: &str) -> u32 {
    s.chars().filter(|c| c.is_ascii()).map(score_letter).sum()
}

fn score_letter(c: char) -> u32 {
    let results: [(char, u32); 26] = [
        ('A', 1),
        ('B', 3),
        ('C', 3),
        ('D', 2),
        ('E', 1),
        ('F', 4),
        ('G', 2),
        ('H', 4),
        ('I', 1),
        ('J', 8),
        ('K', 5),
        ('L', 1),
        ('M', 3),
        ('N', 1),
        ('O', 1),
        ('P', 3),
        ('Q', 10),
        ('R', 1),
        ('S', 1),
        ('T', 1),
        ('U', 1),
        ('V', 4),
        ('W', 4),
        ('X', 8),
        ('Y', 4),
        ('Z', 10),
    ];
    c.to_uppercase()
        .map(|c| {
            results
                .binary_search_by_key(&c, |x| x.0)
                .map(|i| results[i].1)
                .unwrap_or(0)
        })
        .next()
        .unwrap()
}