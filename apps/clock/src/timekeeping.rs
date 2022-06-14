use std::fmt::Display;

use chrono::{Timelike, Local};
use druid::Data;

// this struct is potentially unnecessary
#[derive(Clone, Data)]
pub struct Time {
    #[data(eq)]
    time: chrono::DateTime<Local>
}

impl Time {
    pub fn set_time(&mut self) -> &Self {
        self.time = chrono::Local::now();
        self
    }
    pub fn new() -> Time {
        Time { time: chrono::Local::now() } 
    }
}

impl Display for Time {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{:0>2}:{:0>2}:{:0>2}", self.time.hour(), self.time.minute(), self.time.second())
    }
}