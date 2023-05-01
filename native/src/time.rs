use chrono::{DateTime, Utc};

pub struct Instant {
    pub date_time: DateTime<Utc>,
}

impl Instant {
    pub fn now() -> Self {
        Self {
            date_time: Utc::now(),
        }
    }
    pub fn elapsed(&self) -> std::time::Duration {
        Utc::now()
            .signed_duration_since(self.date_time)
            .to_std()
            .unwrap()
    }
}
