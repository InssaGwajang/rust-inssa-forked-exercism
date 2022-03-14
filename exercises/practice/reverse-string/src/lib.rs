pub fn reverse(input: &str) -> String {
    input.chars().rev().collect()

    // input.chars().rev().fold(String::new(), |mut s, c| {
    //     s.push(c);
    //     s
    // })
}
