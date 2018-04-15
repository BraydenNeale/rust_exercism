// Another solution
pub fn lsp(s: &str, window: usize) -> Result<u64, &str> {
    if window == 0 {
        return Ok(1);
    } else if window > s.len() {
        return Err("window must be smaller than string length");
    }

    let numbers: Vec<u64> = s.chars()
        .filter_map(|c| c.to_digit(10))
        .map(|d| d as u64)
        .collect();

    if numbers.len() < s.len() {
        Err("Every character in the string must be a digit.")
    } else {
        Ok(numbers
            .windows(window)
            .map(|w| w.iter().cloned().product())
            .max()
            .unwrap())
    }
}