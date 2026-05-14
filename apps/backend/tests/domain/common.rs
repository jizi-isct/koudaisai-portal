use chrono::{DateTime, Utc};
use koudaisai_portal_backend::application::ports::clock::Clock;

pub struct FixedClock;

impl Clock for FixedClock {
    fn now(&self) -> DateTime<Utc> {
        Utc::now()
    }
}

impl Clock for &FixedClock {
    fn now(&self) -> DateTime<Utc> {
        Utc::now()
    }
}
