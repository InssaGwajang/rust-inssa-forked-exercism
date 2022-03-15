use std::fmt::{Display, Formatter, Result};

const MINUTES_PER_DAY: i32 = 24 * 60;
const MINUTES_PER_HOUR: i32 = 60;

#[derive(Debug, Eq, PartialEq)]
pub struct Clock(i32);

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        Self(
            ((hours * MINUTES_PER_HOUR + minutes) % MINUTES_PER_DAY + MINUTES_PER_DAY)
                % MINUTES_PER_DAY,
        )
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        Self::new(0, self.0 + minutes)
    }
}

impl Display for Clock {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(
            f,
            "{:02}:{:02}",
            self.0 / MINUTES_PER_HOUR,
            self.0 % MINUTES_PER_HOUR
        )
    }
}
