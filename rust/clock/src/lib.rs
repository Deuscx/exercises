
use std::fmt;
const DAY: i64 = 24 * 60;
const HOUR: i64 = 60;
#[derive(Debug, Eq, PartialEq)]
pub struct Clock {
    minutes: i64,
}


impl Clock {
    pub fn new(hours: i64, minutes: i64) -> Self {
        // unimplemented!("Construct a new Clock from {hours} hours and {minutes} minutes");

        let mut minutes = minutes;
        minutes += hours * HOUR;
        minutes %= DAY;
        if minutes < 0 {
            minutes += DAY;
        }
        Clock { minutes }
    }

    pub fn add_minutes(&self, minutes: i64) -> Self {
        // unimplemented!("Add {minutes} minutes to existing Clock time");
        Clock::new(0, self.minutes + minutes)
    }
}

// to_string() is a trait of fmt::Display
impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:02}:{:02}", self.minutes / HOUR, self.minutes % HOUR)
    }
}
