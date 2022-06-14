use std::fmt::Display;

use chrono::{Local};
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
        write!(f, "{}", self.time.format("%H:%M:%S"))
    }
}