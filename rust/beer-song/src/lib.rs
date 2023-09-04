pub fn verse(n: u32) -> String {
    // Initielized an empty string
    let mut result = String::new();

    // Match the argument and return appropriate number of bottles of beer on the wall
    match n {
        0 => {
            result.push_str("No more bottles of beer on the wall, no more bottles of beer.\n");
            result.push_str("Go to the store and buy some more, 99 bottles of beer on the wall.\n");
        },
        1 => {
            result.push_str("1 bottle of beer on the wall, 1 bottle of beer.\n");
            result.push_str("Take it down and pass it around, no more bottles of beer on the wall.\n");
        },
        2 => {
            result.push_str("2 bottles of beer on the wall, 2 bottles of beer.\n");
            result.push_str("Take one down and pass it around, 1 bottle of beer on the wall.\n");
        },
        _ => {
            result.push_str(&format!("{} bottles of beer on the wall, {} bottles of beer.\n", n, n));
            result.push_str(&format!("Take one down and pass it around, {} bottles of beer on the wall.\n", n - 1));
        }
    }

    result
}

pub fn sing(start: u32, end: u32) -> String {
    let mut song = String::new();

    // loop through the range of start to end and push the verses to the song
    for i in (end..=start).rev() {
        song.push_str(&verse(i));
        if i != end {
            song.push_str("\n");
        }
    }

    song
}
