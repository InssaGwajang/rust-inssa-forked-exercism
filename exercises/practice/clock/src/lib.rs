use std::fmt;

const DAY: i32 = 24 * 60;
const HOUR: i32 = 60;

#[derive(Debug, Eq, PartialEq)]
pub struct Clock(i32);

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        Self(((hours * HOUR + minutes) % DAY + DAY) % DAY)
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        Self::new(0, self.0 + minutes)
    }
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:02}:{:02}", self.0 / HOUR, self.0 % HOUR)
    }
}

// #[derive(Debug, Eq, PartialEq)]
// pub struct Clock(i32, i32);

// impl From<Clock> for String {
//     fn from(clock: Clock) -> String {
//         format!("{:02}:{:02}", clock.0, clock.1)
//     }
// }

// use std::fmt;
// impl fmt::Display for Clock {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         write!(f, "{:02}:{:02}", self.0, self.1)
//     }
// }

// impl Clock {
//     pub fn new(hours: i32, minutes: i32) -> Self {
//         Self(hours, minutes).make_time_as_clock()
//     }

//     pub fn add_minutes(&self, minutes: i32) -> Self {
//         Self(self.0, self.1 + minutes).make_time_as_clock()
//     }

//     fn make_time_as_clock(&self) -> Self {
//         let mut hours = self.0;
//         let mut minutes = self.1;

//         if minutes >= 60 {
//             let mut count = 0;
//             while minutes >= 60 {
//                 count += 1;
//                 minutes -= 60;
//             }
//             hours += count;
//         } else if minutes < 0 {
//             let mut count = 0;
//             while minutes < 0 {
//                 count += 1;
//                 minutes += 60;
//             }
//             hours -= count;
//         }

//         while hours >= 24 {
//             hours -= 24;
//         }
//         while hours < 0 {
//             hours += 24;
//         }

//         Self(hours, minutes)
//     }
// }
