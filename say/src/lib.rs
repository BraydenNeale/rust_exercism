const TEENS: [&str; 20] = [
    "", "one", "two", "three", "four", "five",
    "six", "seven", "eight", "nine", "ten",
    "eleven", "twelve", "thirteen", "fourteen", "fifteen",
    "sixteen", "seventeen", "eighteen", "nineteen",
];

const TENS: [&str; 8] = [
    "twenty", "thirty", "forty", "fifty",
    "sixty", "seventy", "eighty", "ninety",
];

// Encode a sub-thousand value.
fn enc999(i: usize) -> String {
    match i {
        0...19 => String::from(TEENS[i]),
        20...99 => match i % 10 {
            0 => String::from(TENS[i / 10 - 2]),
            _ => format!("{}-{}", TENS[i / 10 - 2], enc999(i % 10),
            ),
        },
        _ => match i % 100 {
            0 => format!("{} hundred", enc999(i / 100)),
            _ => format!("{} hundred {}", enc999(i / 100), enc999(i % 100)),
        },
    }
}

const SUFFIXES: [&str; 6] = [
    "thousand", "million", "billion",
    "trillion", "quadrillion", "quintillion",
];

const BASES: [u64; 7] = [
    1, 1_000, 1_000_000, 1_000_000_000, 1_000_000_000_000,
    1_000_000_000_000_000, 1_000_000_000_000_000_000,
];

pub fn encode(number: u64) -> String {
    if number == 0 {
        return String::from("zero")
    }

    // How many thousand parts there are, i.e. ceil(log_1000(number)).
    // Each part is encoded seperately, added suffix, and joined together.
    // The higher digits are done first to make joining easier.
    let thousand_count = (number.to_string().len() + 2) / 3;
    (0..thousand_count).map(|i| {
        let base_i = (thousand_count - i - 1) as usize;
        let part = enc999((number / BASES[base_i] % 1000) as usize);
        if part.is_empty() {
            vec![]
        } else if base_i == 0 {
            vec![part]
        } else {
            vec![part, String::from(SUFFIXES[base_i - 1])]
        }
    }).flat_map(|part| part.into_iter()).collect::<Vec<_>>().join(" ")
}
