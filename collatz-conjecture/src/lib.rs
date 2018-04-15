// return Ok(x) where x is the number of steps required to reach 1
pub fn collatz(n: u64) -> Result<u64, &'static str> {
    if n < 1 {
    	return Err("+ve numbers only");
    }

    let mut steps = 0;
    let mut val = n;

    while val > 1 {
    	// val = if val % 2 == 0 {val / 2} else {3 * val + 1};

    	val = match val % 2 {
            0 => val / 2,
            _ => 3 * val + 1
        };

    	steps += 1;
    }

    Ok(steps)
}
