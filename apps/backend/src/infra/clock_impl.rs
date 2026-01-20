use crate::application::ports::clock::Clock;
use chrono::{DateTime, Utc};

pub struct ClockImpl;

impl Clock for ClockImpl {
    fn now(&self) -> DateTime<Utc> {
        Utc::now()
    }
}
