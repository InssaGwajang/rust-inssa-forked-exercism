pub fn is_armstrong_number(num: u32) -> bool {
    let s = num.to_string();
    let l = s.len() as u32;

    num == s
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .map(|n| n.pow(l))
        .sum()

    // num == s.chars().fold(0, |mut r, c| {
    //     r += c.to_digit(10).unwrap().pow(l);
    //     r
    // })
}
