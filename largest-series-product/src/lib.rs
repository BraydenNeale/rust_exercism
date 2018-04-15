pub fn lsp(num: &str, wsize: usize) -> Result<u32, &'static str> {
	if wsize == 0 {
		return Ok(1);
	} else if wsize > num.len() {
		return Err("Window size > str length")	
	}

	let num: Result<Vec<u32>, ()> = num.chars()
        .map(|c| convert(c))
        .collect();

   	if num.is_err() { return Err("Contains an invalid char") }

   	let vec: Vec<u32> = num.unwrap();
	let mut iter = vec.windows(wsize);
	let mut max = 0;

	while let Some(arr) = iter.next() {
		let prod = arr.iter().product();
		if prod > max {
			max = prod;
		}
	}

	Ok(max)
}

fn convert(c: char) -> Result<u32, ()> {
    match c.to_digit(10) {
    	Some(x) => Ok(x),
    	None => Err(())
    }
}