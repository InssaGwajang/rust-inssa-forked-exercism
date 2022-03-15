const PRODUCTIONS_PER_HOUR: u32 = 221;

pub fn production_rate_per_hour(speed: u8) -> f64 {
    (PRODUCTIONS_PER_HOUR * speed as u32) as f64
        * match speed {
            0 => 0.0,
            1..=4 => 1.0,
            5..=8 => 0.9,
            9..=u8::MAX => 0.77,
        }
}

pub fn working_items_per_minute(speed: u8) -> u32 {
    (production_rate_per_hour(speed) / 60.0) as u32
}
