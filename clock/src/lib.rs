#[derive(Debug, PartialEq, Eq)]
pub struct Clock {
    pub hours: i32,
    pub minutes: i32,
}

impl Clock {
    pub fn new(mut hours: i32, mut minutes: i32) -> Self {
        while minutes < 0 {
            hours -= 1;
            minutes += 60;
        }

        while hours < 0 {
            hours += 24;
        }

        hours = (hours + minutes / 60) % 24;
        minutes %= 60;

        Self { hours, minutes }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        let mut minutes = self.minutes + minutes;
        let mut hours = self.hours;
        while minutes < 0 {
            hours -= 1;
            minutes += 60;
        }

        while hours < 0 {
            hours += 24;
        }

        hours = (hours + minutes / 60) % 24;
        minutes %= 60;

        Self { hours, minutes }
    }
}

use std::fmt::{self, Display};

impl Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:02}:{:02}", self.hours, self.minutes)
    }
}
