use chrono::{DateTime, Utc};

// Let to set custom now time
pub struct UtcDateTime {
    pub now: Option<DateTime<Utc>>,
}

impl UtcDateTime {}

impl Default for UtcDateTime {
    fn default() -> UtcDateTime {
        Self { now: None }
    }
}
