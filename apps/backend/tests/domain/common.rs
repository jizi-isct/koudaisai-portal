use chrono::{DateTime, Utc};
use koudaisai_portal_backend::application::ports::clock::Clock;

pub struct FixedClock;

impl Clock for FixedClock {
    fn now(&self) -> DateTime<Utc> {
        Utc::now()
    }
}

// `&FixedClock` 用の Clock impl は本体の `impl<C: Clock + ?Sized> Clock for &C`
// ブランケット実装でカバーされるため不要(個別に書くと衝突する)。
