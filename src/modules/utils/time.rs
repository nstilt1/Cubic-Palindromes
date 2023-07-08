use std::time::{SystemTime, UNIX_EPOCH, Duration};

fn now() -> Duration {
    return SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
}

pub fn time_s() -> u64 {
    return now().as_secs();
}

pub fn time_ms() -> u128 {
    return now().as_millis();
}

pub fn time_micros() -> u128 {
    return now().as_millis();
}