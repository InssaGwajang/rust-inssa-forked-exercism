const EXPECTED_OVEN_MINUTES: i32 = 40;
const PREPARATION_MINUTES_PER_LAYER: i32 = 2;

pub fn expected_minutes_in_oven() -> i32 {
    EXPECTED_OVEN_MINUTES
}

pub fn remaining_minutes_in_oven(actual_minutes_in_oven: i32) -> i32 {
    EXPECTED_OVEN_MINUTES - actual_minutes_in_oven
}

pub fn preparation_time_in_minutes(number_of_layers: i32) -> i32 {
    number_of_layers * PREPARATION_MINUTES_PER_LAYER
}

pub fn elapsed_time_in_minutes(number_of_layers: i32, actual_minutes_in_oven: i32) -> i32 {
    preparation_time_in_minutes(number_of_layers) + actual_minutes_in_oven
}
