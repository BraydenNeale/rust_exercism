pub fn encode(num: u64) -> String {
	// only passes 5 of 13 tests
 	let count = num.to_string().chars().count();

 	match count {
	    1 => say_ones(num_at(num,1)),
	    2 => say_tens(num_at(num,2), num_at(num,1)),
	    3 => {
	    	say_hundreds(
	    		num_at(num,3),
	    		num_at(num,2),
	    		num_at(num,1)
	    	)
	    },
	    _ if count >= 4 && count <= 6 => {
	    	say_larger(num, count, &String::from("thousand"))
	    },
	    _ if count >= 7 && count <= 9 => {
	    	say_larger(num, count, &String::from("million"))
	    },
	    _ if count >= 10 && count <= 12 => {
	    	say_larger(num, count, &String::from("billion"))
	    },
	    _ if count >= 13 && count <= 15 => {
	    	say_larger(num, count, &String::from("trillion"))
	    },
	    _ if count >= 16 && count <= 18 => {
	    	say_larger(num, count, &String::from("quadrillion"))
	    },
	    _ if count >= 19 && count <= 21 => {
	    	say_larger(num, count, &String::from("quintrillion"))
	    },

	    _ => String::from("Not supported"),
	}
}

fn num_at(num: u64, place: u32) -> u64{
	let div = 10u64.pow(place-1);
	num / div % 10
}

fn say_ones(num: u64) -> String {
	match num {
		0 => String::from("zero"),
		1 => String::from("one"),
		2 => String::from("two"),
		3 => String::from("three"),
		4 => String::from("four"),
		5 => String::from("five"),
		6 => String::from("six"),
		7 => String::from("seven"),
		8 => String::from("eight"),
		9 => String::from("nine"),
		_ => String::from("error"),
	}
}

fn say_tens(ten: u64, one: u64) -> String {
	match ten {
		0 => say_ones(one),
		1 => format!("{}teen", 		say_ones(one)),
		2 => format!("twenty-{}", 	say_ones(one)),
		3 => format!("thirty-{}", 	say_ones(one)),
		4 => format!("fourty-{}", 	say_ones(one)),
		5 => format!("fifty-{}", 	say_ones(one)),
		6 => format!("sixty-{}", 	say_ones(one)),
		7 => format!("seventy-{}", 	say_ones(one)),
		8 => format!("eighty-{}", 	say_ones(one)),
		9 => format!("ninety-{}", 	say_ones(one)),
		_ => String::from("error"),
	}
}

fn say_hundreds(hundred: u64, ten: u64, one: u64) -> String {
	format!("{} hundred {}", say_ones(hundred), say_tens(ten, one))
}


fn say_larger(_num: u64, count: usize, scale: &str) -> String {
	format!("{} {}", count, scale)
}