//
// See Rust Language Specific Instructions
// below normal exercise description.
//
pub fn get_tenth(n: u64) -> String {
    match n {
        20 => "twenty",
        30 => "thirty",
        40 => "forty",
        50 => "fifty",
        60 => "sixty",
        70 => "seventy",
        80 => "eighty",
        _ => "ninety",
    }.to_string()
}

pub fn encode(n: u64) -> String {
    let names = vec![
        ("quintillion", 1_000_000_000_000_000_000),
        ("quadrillion", 1_000_000_000_000_000),
        ("trillion", 1_000_000_000_000),
        ("billion", 1_000_000_000),
        ("million", 1_000_000),
        ("thousand", 1_000),
        ("hundred", 100),
    ];
    for &(name, value) in names.iter() {
        if n >= value {
            return [
                encode((n - n % value) / value),
                " ".to_string(),
                name.to_string(),
                match n % value {
                    0 => "".to_string(),
                    _ => [" ".to_string(), encode(n % value)].join(""),
                },
            ].join("");
        }
    }
    if n > 19 {
        return [
            get_tenth((n - n % 10)),
            match n % 10 {
                0 => "".to_string(),
                _ => ["-".to_string(), encode(n % 10)].join(""),
            },
        ].join("");
    } else {
        return match n {
            0 => "zero",
            1 => "one",
            2 => "two",
            3 => "three",
            4 => "four",
            5 => "five",
            6 => "six",
            7 => "seven",
            8 => "eight",
            9 => "nine",
            10 => "ten",
            11 => "eleven",
            12 => "twelve",
            13 => "thirteen",
            14 => "fourteen",
            15 => "fifteen",
            16 => "sixteen",
            17 => "seventeen",
            18 => "eighteen",
            _ => "nineteen",
        }.to_string();
    }
}

