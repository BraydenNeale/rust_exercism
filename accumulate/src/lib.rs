pub fn map_function<F>(list: Vec<i32>, f: &F) -> Vec<i32>
where F: Fn(i32) -> i32 {
	// list.iter().map(|x| f(*x)).collect()

	let mut output = Vec::with_capacity(list.len());
	for el in list {
		output.push(f(el));
	}

	output
}

pub fn map_closure<F>(list: Vec<i32>, f: F) -> Vec<i32>
where F: Fn(i32) -> i32 {
	map_function(list, &f)
}