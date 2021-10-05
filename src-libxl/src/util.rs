pub mod hex_slice;

pub fn get_utc_millis() -> i64 {
    chrono::Utc::now().timestamp_millis()
}