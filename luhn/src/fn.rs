// Functional solution from exercism

// #![feature(iterator_step_by)]

/// Checks if Luhnn algorithm
/// can be applied to the input.
///
/// First double every second digit from the right.
///   - If doubled digit is greater than 9 subtract 9.
///
/// Then sum all the digits.
///
/// If the sum is evenly divisible by 10 then true.
// pub fn is_valid(s: &str) -> bool {
//     let only_digit_whitespace: bool = s.chars()
//         .filter(|c| !(c.is_whitespace() || c.is_ascii_digit()))
//         .count() == 0;

//     let doubled: u32 = s.chars()
//         .filter(|c| c.is_ascii_digit())
//         .map(|c| c.to_digit(10).unwrap())
//         .rev()
//         .skip(1)
//         .step_by(2)
//         .map(|d| d * 2)
//         .map(|d| if d > 9 { d - 9 } else { d })
//         .sum();

//     let others: u32 = s.chars()
//         .filter(|c| c.is_ascii_digit())
//         .rev()
//         .map(|c| c.to_digit(10).unwrap())
//         .step_by(2)
//         .sum();

//     only_digit_whitespace && (doubled + others) % 10 == 0
//         && s.chars().filter(|s| s.is_ascii_digit()).count() > 1
// }