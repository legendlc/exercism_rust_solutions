/* Compiled with rustc 1.38.0-nightly */

use std::fmt;

const MIN_OF_HOUR: i32 = 60;
const HOUR_OF_DAY: i32 = 24;

#[derive(Debug)]
pub struct Clock {
    hours: i32,
    minutes: i32,
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        Clock{ 
            hours: (hours + minutes.div_euclid(MIN_OF_HOUR)).rem_euclid(HOUR_OF_DAY),
            minutes: minutes.rem_euclid(MIN_OF_HOUR),
        }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        Clock { 
            hours: (self.hours + (self.minutes + minutes).div_euclid(MIN_OF_HOUR)).rem_euclid(HOUR_OF_DAY),
            minutes: (self.minutes + minutes.rem_euclid(MIN_OF_HOUR)).rem_euclid(MIN_OF_HOUR),
        }
    }
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:02}:{:02}", self.hours, self.minutes)
    }
}

impl PartialEq for Clock {
    // When using Clock::new to create an instance,
    // each (hours:minutes) only has one way to respresent
    fn eq(&self, other: &Self) -> bool {
        self.hours == other.hours && self.minutes == other.minutes
    }
}

impl Eq for Clock {}

impl From<Clock> for String {
    fn from(v: Clock) -> Self {
        format!("{:02}:{:02}", v.hours, v.minutes)
    }
}

mod test {
    use super::*;
    #[test]
    fn test_string_from() {
        assert_eq!(String::from(Clock::new(16, 45)), "16:45");
    }
}