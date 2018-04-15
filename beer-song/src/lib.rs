macro_rules! bottle {
    ( $n:expr ) => {
        match $n {
            0 => String::from("no more bottles"),
            1 => String::from("1 bottle"),
            _ => format!("{} bottles", $n),
        }
    };
}

macro_rules! taken {
    ( $n:expr ) => {
        match $n {
            1 => "it",
            _ => "one",
        }
    };
}

pub fn verse(n: i32) -> String {
    if n == 0 {
        String::from("No more bottles of beer on the wall, no more bottles \
            of beer.\nGo to the store and buy some more, 99 bottles of beer \
            on the wall.\n")
    } else {
        format!("{cur} of beer on the wall, {cur} of beer.\nTake {one} down \
            and pass it around, {next} of beer on the wall.\n",
            cur = bottle!(n), next = bottle!(n - 1), one = taken!(n))
    }
}

pub fn sing(start: i32, end: i32) -> String {
    // I'm doing stable-only so there's no "..=".
    (end..(start + 1)).map(verse).rev().collect::<Vec<_>>().join("\n")
}
