pub mod hex_slice;

pub fn get_utc_millis() -> i64 {
    chrono::Utc::now().timestamp_millis()
}

pub fn get_launcher_formatted_time() -> String {
    chrono::Utc::now().format("%Y-%m-%d-%H").to_string()
}

pub fn get_launcher_formatted_time_long() -> String {
    chrono::Utc::now().format("%Y-%m-%d-%H-%M").to_string()
}
