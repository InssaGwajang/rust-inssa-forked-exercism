pub fn is_valid(code: &str) -> bool {
    code.chars()
        .rev()
        .filter(|c| !c.is_whitespace())
        .try_fold((0, 0), |(sum, cnt), c| {
            c.to_digit(10)
                .map(|n| if cnt % 2 == 1 { n * 2 } else { n })
                .map(|n| if n > 9 { n - 9 } else { n })
                .map(|n| (sum + n, cnt + 1))
        })
        .map_or(false, |(sum, cnt)| cnt > 1 && sum % 10 == 0)
}
