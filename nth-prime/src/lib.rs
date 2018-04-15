// Would not recommend for really large primes...
pub fn nth(n: u32) -> Result<u64, String> {
	if n < 1 { return Result::Err(String::from("n can't be negative")) }
	if n == 1 { return Ok(2) }

	let mut primes_found = 1; // already found '2'
	let mut idx = 2; // start at n = 2
	let mut est: u64 = 0;

	while primes_found != n {
		// iterate through all odd numbers starting at 3
		est = 3 + (2 * (idx-2));
		if is_prime(est) {
			primes_found += 1;
		}

		idx += 1;
	}

	Ok(est)
}

pub fn is_prime(est: u64) -> bool {
	if est <= 1 { return false }
	if est <= 3 { return true }
	if est % 2 == 0 || est % 3 == 0 { return false }

	let mut i = 5;
	while i * i <= est {
		if est % i == 0 || est % (i+2) == 0 { return false }
		i += 6;
	}

	true
}