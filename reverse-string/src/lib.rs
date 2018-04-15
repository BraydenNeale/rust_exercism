pub fn reverse(s: &str) -> String {
	// is changing function signature from _ to s allowed?
	s.chars().rev().collect()
}
