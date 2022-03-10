const FIBONACCI_NUM: usize = 5;

pub fn create_empty() -> Vec<u8> {
    Vec::<u8>::new()
}

pub fn create_buffer(count: usize) -> Vec<u8> {
    vec![0; count]
}

pub fn fibonacci() -> Vec<u8> {
    let mut result = Vec::<u8>::with_capacity(FIBONACCI_NUM);
    let mut prev = 0;
    let mut current = 1;

    for _ in 0..FIBONACCI_NUM {
        result.push(current);
        (prev, current) = (current, prev + current);
    }
    result
}
