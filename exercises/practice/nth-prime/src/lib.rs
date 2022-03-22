fn is_prime(n: &u32) -> bool {
    !(2..=(*n as f64).sqrt() as u32).any(|i| n % i == 0)
}

pub fn nth(n: u32) -> u32 {
    (2..).filter(|i| is_prime(i)).nth(n as usize).unwrap()
}
