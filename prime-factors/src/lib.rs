pub fn factors(mut n: u64) -> Vec<u64> {
    let mut v = Vec::new();
    let mut f = 2;
    while n > 1 {
        while n % f == 0 {
            n /= f;
            v.push(f);
        }
        f += 1;
    }
    v
}

// pub fn factors(val: u64) -> Vec<u64> {
// 	let mut vec: Vec<u64> = Vec::new();
// 	let mut idx = 1;

// 	while let Some(prime) = get_next_prime(idx, val) {
//         let mut test = val;
//         while test >= prime && test % prime == 0 {
//         	vec.push(prime);
//         	test /= prime;
//         }
//         idx = prime;
//     }
//     vec
// }

// fn get_next_prime(last: u64, test: u64) -> Option<u64> {
// 	if last < 2 { return Some(2) }
// 	if last == 2 { return Some(3) }

// 	let mut next = last + 2;

// 	while next <= test / 2 {
// 		if is_prime(next) {
// 			return Some(next)
// 		}
// 		else {
// 			next += 2;
// 		}
// 	}

// 	return None
// }

// pub fn is_prime(est: u64) -> bool {
// 	if est <= 1 { return false }
// 	if est <= 3 { return true }
// 	if est % 2 == 0 || est % 3 == 0 { return false }

// 	let mut i = 5;
// 	while i * i <= est {
// 		if est % i == 0 || est % (i+2) == 0 { return false }
// 		i += 6;
// 	}

// 	true
// }