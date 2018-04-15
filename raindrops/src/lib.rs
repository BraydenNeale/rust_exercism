pub fn raindrops(n: usize) -> String {
	const PLING: &str = "Pling";
	const PLANG: &str = "Plang";
	const PLONG: &str = "Plong";
	let mut output = String::new();

    if n % 3 == 0 { output.push_str(PLING) }
    if n % 5 == 0 { output.push_str(PLANG) }
    if n % 7 == 0 { output.push_str(PLONG) }
    if output.is_empty() { output.push_str(&n.to_string()) }

    output
}
