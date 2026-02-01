use crate::application::ports::clock::Clock;
use chrono::{DateTime, Utc};

#[derive(Clone)]
pub struct MemoryClock {
    now: DateTime<Utc>,
}

impl MemoryClock {
    pub fn new(now: DateTime<Utc>) -> Self {
        Self { now }
    }

    pub fn set_now(&mut self, now: DateTime<Utc>) {
        self.now = now;
    }
}

impl Clock for MemoryClock {
    fn now(&self) -> DateTime<Utc> {
        self.now
    }
}
