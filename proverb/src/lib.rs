// pub fn build_proverb(list: Vec<&str>) -> String {
//     let mut proverb = String::new();

//     for i in 0..list.len() {
//     	if i + 1 < list.len() {
//     		proverb.push_str(&build_line(list[i], list[i + 1]));
//     	} else {
//     		proverb.push_str(&final_line(list[0]));
//     	}
//     }

//     proverb
// }

// With Iterators
pub fn build_proverb(list: Vec<&str>) -> String {
	let mut proverb = String::new();
	let first = list.get(0).unwrap_or(&"");

    for (i, val) in list.iter().enumerate() {
    	let line = match list.get(i+1) {
    		Some(next) => build_line(val, next),
    		None => final_line(first)
    	};

    	proverb.push_str(&line);
    }
    
    proverb
}


pub fn build_line(want: &str, lost: &str) -> String {
	format!("For want of a {} the {} was lost.\n", want, lost)
}

pub fn final_line(want: &str) -> String {
	format!("And all for the want of a {}.", want)
}

// functional solution (from exercism user uranusjr)
// pub fn build_proverb(list: Vec<&str>) -> String {
//     if list.is_empty() {
//         return String::new();
//     }

//     let proverbs = list.windows(2).map(|window| {
//         format!("For want of a {} the {} was lost.", window[0], window[1])
//     }).chain(Some(format!(
//         "And all for the want of a {}.", list[0],
//     ))).collect::<Vec<_>>();
//     proverbs.join("\n")
// }
