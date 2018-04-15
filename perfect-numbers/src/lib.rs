// use std::cmp::Ordering;

// #[derive(Debug, PartialEq, Eq)]
// pub enum Classification {
//     Abundant,
//     Perfect,
//     Deficient
// }

// pub fn classify(num: u64) -> Result<Classification, & 'static str> {
// 	if num == 0 {
// 		return Err("Number must be positive");
// 	}

// 	let aliquot_sum: u64 = (1..num/2+1)
// 		.filter(|x| num % x == 0)
// 		.sum();

//   	// match aliquot_sum {
//   	// 	x if x == num => Ok(Classification::Perfect),
//   	// 	x if x > num => Ok(Classification::Abundant),
//   	// 	_  => Ok(Classification::Deficient),
//   	// }

//     Ok(match aliquot_sum.cmp(&num) {
//         Ordering::Less => Classification::Deficient,
//         Ordering::Equal => Classification::Perfect,
//         Ordering::Greater => Classification::Abundant,
//     })

// }

use std::iter::once;
use std::cmp::Ordering;

#[derive(Debug, PartialEq, Eq)]
pub enum Classification {
    Abundant,
    Perfect,
    Deficient
}

// much more efficient
fn aliquot_sum(num: u64) -> u64 {
    let ceil = (num as f64).sqrt().ceil() as u64;
    let sum: u64 = (1..ceil)
        .filter(|x| num % x == 0)
        .flat_map(|x| once(num / x).chain(once(x)))
        .sum();

    sum - num
}

pub fn classify(num: u64) -> Result<Classification, & 'static str> {
    if num == 0 {
        return Err("Number must be positive");
    } 
    if num == 1 {
        return Ok(Classification::Deficient);
    }

     Ok(match aliquot_sum(num).cmp(&num) {
        Ordering::Less => Classification::Deficient,
        Ordering::Equal => Classification::Perfect,
        Ordering::Greater => Classification::Abundant,
    })
}
