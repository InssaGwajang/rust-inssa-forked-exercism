fn bottles(n: u32) -> &'static str {
    match n {
        1 => "bottle",
        _ => "bottles",
    }
}

pub fn verse(n: u32) -> String {
    format!(
        "{}\n{}\n",
        match n {
            0 => String::from("No more bottles of beer on the wall, no more bottles of beer."),
            n => format!(
                "{0} {1} of beer on the wall, {0} {1} of beer.",
                n,
                bottles(n)
            ),
        },
        match n {
            0 => String::from("Go to the store and buy some more, 99 bottles of beer on the wall."),
            1 => String::from(
                "Take it down and pass it around, no more bottles of beer on the wall."
            ),
            n => format!(
                "Take one down and pass it around, {} {} of beer on the wall.",
                n - 1,
                bottles(n - 1)
            ),
        }
    )
}

pub fn sing(start: u32, end: u32) -> String {
    (end..=start)
        .rev()
        .map(verse)
        .collect::<Vec<String>>()
        .join("\n")
}
