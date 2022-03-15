const FIBONACCI_LENGTH: usize = 5;

pub fn create_empty() -> Vec<u8> {
    Vec::<u8>::new()
}

pub fn create_buffer(count: usize) -> Vec<u8> {
    vec![0; count]
}

pub fn fibonacci() -> Vec<u8> {
    let mut buffer = create_buffer(FIBONACCI_LENGTH);
    let mut prev = 0;
    let mut last = 1;

    for number in buffer.iter_mut() {
        *number = last;
        (prev, last) = (last, prev + last);
    }

    buffer
}
