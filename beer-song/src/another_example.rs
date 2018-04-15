pub fn verse(n: i32) -> String {
    let mut line1 = match n {
        0 => "No more bottles of beer on the wall, no more bottles of beer.".to_string(),
        1 => "1 bottle of beer on the wall, 1 bottle of beer.".to_string(),
        _ => format!("{} bottles of beer on the wall, {} bottles of beer.", n, n)
    };

    let line2 = match n - 1 {
        -1 => "Go to the store and buy some more, 99 bottles of beer on the wall.\n".to_string(),
        0  => "Take it down and pass it around, no more bottles of beer on the wall.\n".to_string(),
        1  => "Take one down and pass it around, 1 bottle of beer on the wall.\n".to_string(),
        _  => format!("Take one down and pass it around, {} bottles of beer on the wall.\n", n-1)
    };

    [line1, line2].join("\n")
}

pub fn sing(start: i32, end: i32) -> String {
    (end..start+1).rev().map(verse).collect::<Vec<String>>().join("\n")
}
