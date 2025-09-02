use std::time::SystemTime;

pub fn random(low: i32, high: i32) -> i32 {
    let timestamp = (SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_millis() as i32)
        .abs();
    low + (timestamp % (high - low))
}
