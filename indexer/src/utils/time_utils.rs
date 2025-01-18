use std::time::{SystemTime, UNIX_EPOCH};

pub fn get_unix_timestamp_in_secs() -> i64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs() as i64
}
