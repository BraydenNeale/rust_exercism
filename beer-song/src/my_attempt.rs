pub fn verse(n: i32) -> String {
    let mut v1 = String::new();
    let mut v2 = String::new();
    
    // first verse
    let mut plural = String::from("s");
    let mut count = n.to_string();
    let mut count2 = n.to_string();

    // second verse
    let mut take = String::from("one");
    let mut plural2 = String::from("s");
    let mut remaining = (n-1).to_string();

    if n == 0 {
    	count = String::from("No more");
    	count2 = String::from("no more");
    } else if n == 1 { 
    	plural = String::from("");
    	take = String::from("it");
    	remaining = String::from("no more");
    } else if n == 2 {
    	plural2 = String::from("");
    }

    v1 = format!("{} bottle{} of beer on the wall, {} bottle{} of beer.", count, plural, count2, plural);

    if n == 0 {
    	v2 = format!("Go to the store and buy some more, 99 bottles of beer on the wall.\n");
    } else {
    	v2 = format!("Take {} down and pass it around, {} bottle{} of beer on the wall.\n", take, remaining, plural2);
    }

    [v1, v2].join("\n")
}

pub fn sing(start: i32, end: i32) -> String {
    let mut song = String::new();
    let mut idx = start;

    while idx >= end {
    	song.push_str(&verse(idx));
    	song.push('\n');
    	idx -= 1;
    }
    song.pop();
    song
}
