pub trait Clock {
    fn now(&self) -> chrono::DateTime<chrono::Utc>;
}
